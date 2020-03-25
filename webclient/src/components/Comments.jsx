import React from 'react';
import moment from 'moment-timezone';
import { observer } from 'mobx-react';
import { action, observable } from 'mobx';
import { Form, Segment } from 'semantic-ui-react';
import UserName from './UserName';

@observer
class CommentForm extends React.Component {
	@observable text = '';

	@action handleChange(e) {
		this.text = e.target.value
	}

	@action handleSubmit() {
		this.props.onSubmit(this.text);
		this.text = '';
	}

	render() {
		return (
			<Form onSubmit={() => this.handleSubmit()}>
				<Form.Input
			      label='Comment'
			      placeholder='comment'
			      value={this.text}
			      onChange={e => this.handleChange(e)}  />
			</Form>
		)
	}
}

const Comments = ({ comments, onSubmit }) => (
  <Segment>
	<h3>Comments</h3>
	<ul>
	{comments.map(c => (
	<li key={c.id}>
		<p>{c.text} [<UserName userId={c.user_id} /> {moment.tz(c.created, 'UTC').tz('America/New_York').fromNow()}]</p>
	</li>
	))}
    </ul>
	<CommentForm onSubmit={onSubmit} />
  </Segment>
);

export default Comments;
