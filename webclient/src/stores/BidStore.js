import { action, observable } from 'mobx';
import { parseBid } from '../util/bridge';

export class Bid {
	store;
	id;

	@observable hasLoaded = false;
	@observable isLoading = false;

	@observable user_id = null
	@observable exercise_id = null
	@observable next_bid = null

	constructor(store, id) {
		this.store = store;
		this.id = id;
		this.load();
	}

	get exercise() {
		return this.store.root.exerciseStore.getExercise(this.exercise_id);
	}

	@action load() {
		this.isLoading = true;
		this.store.loadBidById(this.id);
	}

	@action updateFromJson(json) {
		this.user_id = json.user_id;
		this.exercise_id = json.exercise_id;
		this.next_bid = parseBid(json.next_bid)
		this.hasLoaded = true;
		this.isLoading = false;
	}
}

class BidStore {
	root;
	@observable bids = [];
	@observable isLoading = false;

	constructor(root) {
		this.root = root;
	}

	getBid(id) {
		let bid = this.bids.find(b => b.id === id);
		if (!bid) {
			bid = new Bid(this, id);
			this.bids.push(bid);
		}
		return bid;
	}

	getBidsForExercise(id) {
		return this.root.api.getBidsByExerciseId(id)
			.then(({ bids }) => {
				bids.forEach(json => this.updateBidFromServer(json));
				return bids.map(b => b.exercise_bid_id);
			})
			.catch(err => {
				this.root.notificationStore.error(`Failed to fetch bids for exercise ${id}`, `${err}`);
			});
	}

	@action loadBidById(id) {
		this.isLoading = true;
		this.root.api.getBidById(this.root.commonStore.token, id)
			.then(json => {
				this.updateBidFromServer(json);
				this.isLoading = false;
			})
			.catch(err => {
				this.root.notificationStore.error(`Failed to fetch bid ${id}`, `${err}`);
			});
	}

	@action updateBidFromServer(json) {
		let bid = this.getBid(json.exercise_bid_id);
		bid.updateFromJson(json);
	}
}

export default BidStore;
