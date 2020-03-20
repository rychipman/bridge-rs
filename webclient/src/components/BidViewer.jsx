import React from 'react';
import { inject, observer } from 'mobx-react';
import { Container, Segment } from 'semantic-ui-react';
import Exercise from './Exercise';
import Bid from './Bid';

@inject('rootStore')
@observer
class BidViewer extends React.Component {

	bid() {
		const store = this.props.rootStore.bidStore;
		const bid = store.getBid(this.props.match.params.id);
		return bid;
	}

	render() {
		const bid = this.bid();

		let loading = null;
		if (bid.isLoading) {
			loading = <Segment content={'Updating...'} />;
		}

		let body = null;
		if (bid.hasLoaded) {
			body = (
				<div>
					<Exercise exercise_id={bid.exercise_id} nextBid={bid.next_bid} />
					<Bid bidId={bid.id} />
				</div>
			);
		}

		return (
			<Container style={{ width: '40%' }}>
			  {loading}
			  {body}
			</Container>
		);
	}
}

export default BidViewer;
