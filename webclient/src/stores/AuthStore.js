import { action, observable } from "mobx";

class AuthStore {
	root;
	@observable inProgress = false;
	@observable values = {
		email: '',
		password: '',
	};

	constructor(root) {
		this.root = root;
	}

	@action setEmail(email) {
		this.values.email = email;
	}

	@action setPassword(password) {
		this.values.password = password;
	}

	@action login() {
		this.inProgress = true;
		this.root.api.login(this.values.email, this.values.password)
			.then(this.loginResult)
			.catch(err => {
				this.root.notificationStore.error(`Failed to log in`, `${err}`);
				this.inProgress = false;
			});
	}

	@action.bound
	loginResult(data) {
		this.inProgress = false;
		this.values.password = '';
		const { email, token } = data;
		this.root.notificationStore.success('Login Successful', 'You may now use your account')
		this.root.commonStore.setUser(email);
		this.root.commonStore.setToken(token);
	}

	@action register() {
		this.inProgress = true;
		this.root.api.register(this.values.email, this.values.password)
			.then(this.registrationResult)
			.catch(err => {
				this.root.notificationStore.error(`Failed to register user`, `${err}`);
				this.inProgress = false;
			});
	}

	@action.bound
	registrationResult(res) {
		this.inProgress = false;
		this.values.password = '';
		this.root.notificationStore.success('Registration Successful', 'You may now login with the newly-registered account');
	}

	@action logout() {
		this.root.commonStore.setUser(undefined);
		this.root.commonStore.setToken(undefined);
	}

	@action reset() {
		this.inProgress = false;
		this.values = {
			email: '',
			password: '',
		};
	}
}

export default AuthStore;
