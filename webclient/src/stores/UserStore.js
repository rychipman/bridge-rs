import { action, computed, observable } from 'mobx';
import moment from 'moment-timezone';

class User {
	id = null;
	store = null;

	@observable hasLoaded = false;
	@observable isLoading = false;

	@observable email = '<placeholder>';
	@observable last_active = '<never>';

	constructor(store, id) {
		this.store = store;
		this.id = id;
		this.load();
	}

	@computed get last_active_pretty() {
		if (this.last_active) {
			return moment.tz(this.last_active, 'UTC').tz('America/New_York').fromNow();
		}
		return 'never';
	}

	@action load() {
		this.isLoading = true;
		this.store.loadUserById(this.id);
	}

	@action updateFromJson(json) {
		this.email = json.email;
		this.last_active = json.last_active;
	}
}

class UserStore {
	root;
	@observable users = [];
	@observable isLoading = false;

	constructor(root) {
		this.root = root;
		this.loadUsers();
	}

	getUser(id) {
		let user = this.users.find(u => u.id === id);
		if (!user) {
			user = new User(this, id);
			this.users.push(user);
		}
		return user;
	}

	@action loadUsers() {
		this.isLoading = true;
		this.root.api.getUsers()
			.then(({ users }) => {
				users.forEach(json => this.updateUserFromServer(json));
				this.isLoading = false;
			})
			.catch(err => {
				this.root.notificationStore.error(`Failed to fetch users`, `${err}`);
			});
	}

	@action loadUserById(id) {
		this.isLoading = true;
		this.root.api.getUserById(id)
			.then(({ user }) => {
				this.updateUserFromServer(user);
				this.isLoading = false;
			})
			.catch(err => {
				this.root.notificationStore.error(`Failed to fetch user ${id}`, `${err}`);
			});
	}

	@action updateUserFromServer(json) {
		let user = this.getUser(json.id);
		user.updateFromJson(json);
	}
}

export default UserStore;
