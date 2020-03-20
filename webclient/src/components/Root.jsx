import React from "react";
import { Redirect } from 'react-router';
import { inject, observer } from 'mobx-react';

@inject('rootStore')
@observer
class RootPage extends React.Component {
	render() {
		if (this.props.rootStore.commonStore.currentUser) {
			return <Redirect to='/bid?next=bid' />;
		} else {
			return <Redirect to='/login' />;
		}
	}
}

export default RootPage;
