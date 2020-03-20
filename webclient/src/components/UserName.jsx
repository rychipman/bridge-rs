import React from 'react';
import { observer, inject } from 'mobx-react';

@inject('rootStore')
@observer
class UserName extends React.Component {
	render() {
		const store = this.props.rootStore.userStore;
		const user = store.getUser(this.props.userId);
		return (
			<b>{user.email}</b>
		);
	}
}

export default UserName;
