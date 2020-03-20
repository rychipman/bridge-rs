import { action, observable } from "mobx";

class NotificationStore {
	root;
	lastID = 0
	@observable notifications = []

	constructor(root) {
		this.root = root;
	}

	@action dismiss(id) {
		this.notifications = this.notifications.filter(n => n.id !== id)
	}

	@action add(purpose, title, message) {
		this.notifications.push({
			id: this.lastID,
			purpose,
			title,
			message,
		})
		this.lastID++
	}

	@action success(title, message) {
		this.add('success', title, message)
	}

	@action warning(title, message) {
		this.add('warning', title, message)
	}

	@action error(title, message) {
		this.add('error', title, message)
	}
}

export default NotificationStore;
