import React from 'react';
import moment from 'moment-timezone';
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
				  <p>last active: {moment.tz(u.last_active, 'UTC').tz('America/New_York').fromNow()}</p>
				</Segment>
			))}
			</Container>
		);
	}
}

export default Users;