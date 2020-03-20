import React from "react";
import { inject, observer } from "mobx-react";
import {
    Container,
    Form,
	Message,
	Segment,
} from "semantic-ui-react";

@inject('rootStore')
@observer
class LoginPage extends React.Component {

render() {
	const authStore = this.props.rootStore.authStore;
	const { values, inProgress } = authStore;
    return (
      <Container style={{ width: '40%' }}>
        <Message >
            <Message.Header content='Login' />
            <Message.Content content='Sign in to your BridgeSkills account' />
        </Message>
		<Segment>
		<Form loading={inProgress} onSubmit={() => authStore.login()}>
			<Form.Input
			  label='Email'
			  placeholder='email'
			  value={values.email}
			  onChange={this.handleEmailChange}
			/>
			<Form.Input
			  type='password'
			  label='Password'
			  placeholder='password'
			  value={values.password}
			  onChange={this.handlePasswordChange}
			/>
			<Form.Button content='Login' />
		</Form>
		</Segment>
      </Container>
    );
  }

	handlePasswordChange = e => this.props.rootStore.authStore.setPassword(e.target.value);
	handleEmailChange = e => this.props.rootStore.authStore.setEmail(e.target.value);
}

export default LoginPage;
