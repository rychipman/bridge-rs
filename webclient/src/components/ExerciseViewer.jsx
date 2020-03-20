import React from 'react';
import { inject, observer } from 'mobx-react';
import { Container } from 'semantic-ui-react';
import Exercise from './Exercise';

@inject('rootStore')
@observer
class ExerciseViewer extends React.Component {
	render() {
		return (
			<Container style={{ width: '40%' }}>
			  <Exercise exercise_id={this.props.match.params.id} />
			</Container>
		);
	}
}

export default ExerciseViewer;
