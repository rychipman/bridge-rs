import { action, observable } from 'mobx';
import { nextBid, parseBid, bidToString, shortenRankString } from '../util/bridge';

export class Exercise {
	store;
	id;

	@observable hasLoaded = false;
	@observable isLoading = false;

	@observable cards = {}
	@observable dealer = ''
	@observable vulnerable = ''
	@observable bids = []
	@observable comments = []

	constructor(store, id) {
		this.store = store;
		this.id = id;
		this.load();
	}

	@action load() {
		this.isLoading = true;
		this.store.loadExerciseById(this.id);
	}

	@action bid(bid) {
		return this.store.bidExerciseById(this.id, bid);
	}

	@action comment(text) {
		this.store.commentExerciseById(this.id, text);
	}

	@action updateFromJson(json) {
		this.dealer = json.deal.dealer.toLowerCase();
		this.vulnerable = json.deal.vulnerable;
		this.bids.replace(json.bids.map(parseBid));
		this.comments.replace(json.comments);

		const nextBidSeat = nextBid(this.dealer, this.bids);

		this.cards = json.deal[nextBidSeat].reduce(
			(acc, card) => {
				acc[card.suit].push(shortenRankString(card.rank));
				return acc;
			},
			{Spades: [], Hearts: [], Diamonds: [], Clubs: []},
		);

		this.hasLoaded = true;
		this.isLoading = false;
	}
}

class ExerciseStore {
	root;
	@observable exercises = [];
	@observable isLoading = false;

	constructor(root) {
		this.root = root;
	}

	getExercise(id) {
		let ex = this.exercises.find(x => x.id === id);
		if (!ex) {
			ex = new Exercise(this, id);
			this.exercises.push(ex);
		}
		return ex;
	}

	getExerciseForBidding() {
		return this.root.api.getExercise(this.root.commonStore.token)
		    .then(json => {
				this.updateExerciseFromServer(json);
				return json.exercise_id;
			})
			.catch(err => {
				this.root.notificationStore.error('Failed to get exercise for bidding', `${err}`);
			});
	}

	getExerciseWithConflict() {
		return this.root.api.getExerciseWithConflict(this.root.commonStore.token)
		    .then(json => {
				this.updateExerciseFromServer(json);
				return json.exercise_id;
			})
			.catch(err => {
				this.root.notificationStore.error('Failed to get exercise with conflict', `${err}`);
			});
	}

	@action bidExerciseById(id, bid) {
		const bidStr = bidToString(bid);
		return this.root.api.makeBid(this.root.commonStore.token, id, bidStr)
		    .then(data => data.exercise_bid_id)
			.catch(err => {
				this.root.notificationStore.error('Failed to submit bid', `${err}`);
			});
	}

	@action commentExerciseById(id, text) {
		this.root.api.submitComment(this.root.commonStore.token, id, text)
			.then(() => this.loadExerciseById(id))
			.catch(err => {
				this.root.notificationStore.error(`Failed to submit comment`, `${err}`);
			});
	}

	@action loadExerciseById(id) {
		this.isLoading = true;
		this.root.api.getExerciseById(this.root.commonStore.token, id)
			.then(json => {
				this.updateExerciseFromServer(json);
				this.isLoading = false;
			})
			.catch(err => {
				this.root.notificationStore.error(`Failed to fetch exercise ${id}`, `${err}`);
			});
	}

	@action updateExerciseFromServer(json) {
		let exercise = this.getExercise(json.exercise_id);
		exercise.updateFromJson(json);
	}
}

export default ExerciseStore;
