import React from 'react';
import { observable } from 'mobx';
import { observer, inject } from 'mobx-react';

@inject('rootStore')
@observer
class UserName extends React.Component {
	@observable user = null;

	componentDidMount() {
		const store = this.props.rootStore.userStore;
		this.user = store.getUser(this.props.userId);
	}

	render() {
		if (this.user) {
			return (<b>{this.user.email}</b>);
		}
		return (<b>{'<placeholder>'}</b>);
	}
}

export default UserName;
