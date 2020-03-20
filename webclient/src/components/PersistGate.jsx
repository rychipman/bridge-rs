import React from 'react';
import { inject, observer } from 'mobx-react';
import { Loader } from 'semantic-ui-react';

@inject('rootStore')
@observer
class PersistGate extends React.Component {
	render() {
		if (!this.props.rootStore.commonStore.hydrated) {
			return <Loader size='massive' active>Loading</Loader>
		}
		return this.props.children;
	}
}

export default PersistGate;
