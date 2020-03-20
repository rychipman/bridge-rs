import React from 'react';
import { inject, observer } from 'mobx-react';
import { observable } from 'mobx';
import { Container } from 'semantic-ui-react';
import Exercise from './Exercise';
import Bid from './Bid';

@inject('rootStore')
@observer
class ExerciseReviewer extends React.Component {
	@observable bidIds = [];

	componentDidMount() {
		const store = this.props.rootStore.bidStore;
		store.getBidsForExercise(this.props.match.params.id)
			.then(bidIds => this.bidIds = bidIds);
	}

	render() {
		return (
			<Container style={{ width: '40%' }}>
			  <Exercise exercise_id={this.props.match.params.id} />
			  {this.bidIds.map(id => (
				  <Bid bidId={id} />
			  ))}
			</Container>
		);
	}
}

export default ExerciseReviewer;
