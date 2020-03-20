import AuthStore from './AuthStore';
import BidStore from './BidStore';
import CommonStore from './CommonStore';
import ExerciseStore from './ExerciseStore';
import NotificationStore from './NotificationStore';
import UserStore from './UserStore';

import { observable, action } from 'mobx';
import { create } from 'mobx-persist';

class RootStore {
	api;

	authStore;
	bidStore;
	commonStore;
	exerciseStore;
	notificationStore;
	userStore;

	@observable loading = false;

	constructor(api) {
		this.api = api;
		this.authStore = new AuthStore(this);
		this.bidStore = new BidStore(this);
		this.commonStore = new CommonStore(this);
		this.exerciseStore = new ExerciseStore(this);
		this.notificationStore = new NotificationStore(this);
		this.userStore = new UserStore(this);
		this.hydrate();
	}

	@action hydrate() {
		const hydrate = create({});
		hydrate('commonStore', this.commonStore).then(() => {
			console.log('commonStore rehydrated');
			this.commonStore.setHydrated();
		});
	}

	@action setLoading(loading) {
		this.loading = loading;
	}
}

export default RootStore;
