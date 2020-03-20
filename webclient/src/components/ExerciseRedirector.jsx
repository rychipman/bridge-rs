import React from 'react';
import { Redirect } from 'react-router';
import { inject, observer } from 'mobx-react';
import { observable } from 'mobx';

@inject('rootStore')
@observer
export class ExerciseBidRedirector extends React.Component {
	@observable exercise_id = null;

	componentDidMount() {
		const store = this.props.rootStore.exerciseStore;
		store.getExerciseForBidding().then(ex_id => this.exercise_id = ex_id);
	}

	render() {
		if (!this.exercise_id) {
			return null;
		}
		const path = `/exercise/${this.exercise_id}/bid`;
		return <Redirect to={{...this.props.location, pathname: path}} />;
	}
}

@inject('rootStore')
@observer
export class ExerciseReviewRedirector extends React.Component {
	@observable exercise_id = null;

	componentDidMount() {
		const store = this.props.rootStore.exerciseStore;
		store.getExerciseWithConflict().then(ex_id => this.exercise_id = ex_id);
	}

	render() {
		if (!this.exercise_id) {
			return null;
		}
		const path = `/exercise/${this.exercise_id}/review`;
		return <Redirect to={{...this.props.location, pathname: path}} />;
	}
}