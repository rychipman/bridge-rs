import React from 'react';
import queryString from 'query-string';
import { inject, observer } from 'mobx-react';
import { observable } from 'mobx';
import { Redirect } from 'react-router';
import { Container } from 'semantic-ui-react';
import Exercise from './Exercise';
import BidChooser from './BidChooser';

@inject('rootStore')
@observer
class ExerciseBidder extends React.Component {
	@observable bidId = null;

	renderRedirect() {
		if (this.bidId) {
			const params = queryString.parse(this.props.location.search);
			let path = `/bid/${this.bidId}`;
			if (params.next === 'bid') {
				path = '/bid';
			}
			return <Redirect push to={{...this.props.location, pathname: path}} />;
		}
	}

	render() {
		const ex_id = this.props.match.params.id;
		return (
			<Container style={{ width: '40%' }}>
				{this.renderRedirect()}
				<Exercise exercise_id={ex_id} hideComments />
				<BidChooser
				  exercise_id={ex_id}
				  onBidCreated={bidId => this.bidId = bidId} />
			</Container>
		);
	}
}

export default ExerciseBidder;
