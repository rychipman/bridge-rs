import React from 'react';
import { inject, observer } from 'mobx-react';
import { Container, Segment } from 'semantic-ui-react';

@inject('rootStore')
@observer
class Users extends React.Component {
    render() {
		const store = this.props.rootStore.userStore;
		return (
			<Container style={{ width: '40%' }}>
			{store.loading ? <Segment content={'Loading...'} /> : null}
			{store.users.map(u => (
			    <Segment key={u.id}>
				  <h3>{u.email}</h3>
				  <p>id: {u.id}</p>
				  <p>last active: {u.last_active_pretty}</p>
				</Segment>
			))}
			</Container>
		);
	}
}

export default Users;
