import React from 'react';
import { render } from 'react-dom';
import DevTools from 'mobx-react-devtools';
import { Provider } from 'mobx-react';
import registerServiceWorker from './registerServiceWorker';

import 'semantic-ui-css/semantic.min.css';

import {
	BrowserRouter as Router,
	Route, Switch,
} from "react-router-dom";

import Root from './components/Root';
import Login from './components/Login';
import Register from './components/Register';
import Layout from './components/Layout';
import PersistGate from './components/PersistGate';
import Notifications from './components/Notifications';
import ExerciseBidder from './components/ExerciseBidder';
import ExerciseViewer from './components/ExerciseViewer';
import ExerciseReviewer from './components/ExerciseReviewer';
import {
	ExerciseBidRedirector,
	ExerciseReviewRedirector,
} from './components/ExerciseRedirector';
import BidViewer from './components/BidViewer';
import Users from './components/Users';

import RootStore from './stores/RootStore';

import agent from './agent';

const rootStore = new RootStore(agent);
const stores = {
	rootStore,
};

const useDevTools = false;

render(
	<Provider {...stores}>
	  <PersistGate>
	  <Router>
		<div style={{height: '100%'}}>
			<Layout>
				{useDevTools ? <DevTools /> : null}
				<Switch>
					<Route exact path="/login" component={Login} />
					<Route exact path="/register" component={Register} />
					<Route exact path="/users" component={Users} />
					<Route exact path="/review" component={ExerciseReviewRedirector} />
					<Route exact path="/bid" component={ExerciseBidRedirector} />
					<Route exact path="/bid/:id" component={BidViewer} />
					<Route exact path="/exercise/:id" component={ExerciseViewer} />
					<Route exact path="/exercise/:id/bid" component={ExerciseBidder} />
					<Route exact path="/exercise/:id/review" component={ExerciseReviewer} />
					<Route component={Root} />
				</Switch>
			</Layout>
			<Notifications />
		</div>
	  </Router>
	  </PersistGate>
	</Provider>,
  document.getElementById("root")
);

registerServiceWorker();
