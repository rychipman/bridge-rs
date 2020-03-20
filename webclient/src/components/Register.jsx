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
class RegisterPage extends React.Component {

render() {
	const { authStore } = this.props.rootStore;
	const { values, inProgress } = authStore;
    return (
      <Container style={{ width: '40%' }}>
        <Message>
            <Message.Header content='Register' />
            <Message.Content content='Create your BridgeSkills account' />
        </Message>
		<Segment>
		<Form loading={inProgress} onSubmit={() => authStore.register()}>
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
			<Form.Button content='Register' />
		</Form>
		</Segment>
      </Container>
    );
  }

	handlePasswordChange = e => this.props.rootStore.authStore.setPassword(e.target.value);
	handleEmailChange = e => this.props.rootStore.authStore.setEmail(e.target.value);
}

export default RegisterPage;