import { action, observable } from "mobx";
import { persist } from 'mobx-persist';

class CommonStore {
	root;

	@observable hydrated = false;
	@persist @observable token = undefined;
	@persist @observable currentUser = undefined;
	@observable appLoaded = false;

	constructor(root) {
		this.root = root;
	}

	@action setHydrated() {
		this.hydrated = true;
	}

	@action setUser(user) {
		this.currentUser = user;
	}

	@action setToken(token) {
		this.token = token;
	}

	@action setAppLoaded() {
		this.appLoaded = true;
	}
}

export default CommonStore;
