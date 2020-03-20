import React from 'react';
import { inject, observer } from 'mobx-react';
import { Segment } from 'semantic-ui-react';
import { bidToString } from '../util/bridge';
import UserName from './UserName';

@inject('rootStore')
@observer
class Bid extends React.Component {

	bid() {
		const store = this.props.rootStore.bidStore;
		const bid = store.getBid(this.props.bidId);
		return bid;
	}

	render() {
		const bid = this.bid();

		if (bid.isLoading) {
			return <Segment content={'Loading...'} />;
		}

		if (bid.hasLoaded) {
			return (
				<Segment>
					<UserName userId={bid.user_id} /> bid: {bidToString(bid.next_bid)}
				</Segment>
			);
		}

		return null;
	}
}

export default Bid;
