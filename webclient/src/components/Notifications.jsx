import React from 'react'
import { Message } from 'semantic-ui-react'
import { inject, observer } from "mobx-react";

const notificationsStyle = {
    position: 'fixed',
    bottom: '0',
    right: '0',
    paddingRight: '30px',
    paddingBottom: '30px',
    width: '30%',
    zIndex: 10,
}

@inject('rootStore')
@observer
class Notifications extends React.Component {
  render() {
    const { notificationStore } = this.props.rootStore;
    return (
		<div className='notifications' style={notificationsStyle}>
		{notificationStore.notifications.map(note => (
			<Message
				key={note.id}
				floating
				onDismiss={() => notificationStore.dismiss(note.id)}
				header={note.title}
				content={note.message}
				error={note.purpose === 'error'}
				warning={note.purpose === 'warning'}
				success={note.purpose === 'success'}
			/>
		))}
		</div>
	)
  }
}

export default Notifications;
