import React from 'react';
import Hand from './Hand';
import BidTable from './BidTable';
import Comments from './Comments';
import { inject, observer } from 'mobx-react';
import { Segment } from 'semantic-ui-react';

const Exercise = ({ dealer, vulnerable, cards, bids, nextBid }) => (
  <div>
	<Hand cards={cards} />
	<BidTable
		nextBid={nextBid}
		bids={bids}
		dealer={dealer}
		vulnerable={vulnerable} />
  </div>
);

@inject('rootStore')
@observer
class ExerciseContainer extends React.Component {
	exercise() {
		const store = this.props.rootStore.exerciseStore;
		const exercise = store.getExercise(this.props.exercise_id);
		return exercise;
	}

	componentWillMount() {
		this.exercise();
	}

	render() {
		const exercise = this.exercise();
		if (!exercise.hasLoaded) {
			return <Segment content={`Finding exercise ${exercise.id}...`} />;
		}

		let comments = <Comments comments={exercise.comments} onSubmit={txt => exercise.comment(txt)} />;
		if (this.props.hideComments) {
		    comments = null;
		}

		return (
			<div>
				<Exercise
				  dealer={exercise.dealer}
				  vulnerable={exercise.vulnerable}
				  cards={exercise.cards}
				  bids={exercise.bids}
				  nextBid={this.props.nextBid} />
				{comments}
			</div>
		);
	}
}

export default ExerciseContainer;
