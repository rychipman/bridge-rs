import axios from "axios";

const agent = axios.create({
	baseURL: '',
	timeout: 15000,
})

const api = {
	login: (email, password) =>
		agent.request({
			method: 'post',
			url: '/login',
			data: { email, password },
		}),
	register: (email, password) =>
		agent.request({
			method: 'post',
			url: '/register',
			data: { email, password },
		}),
	getExercise: (token) =>
		agent.request({
			method: 'get',
			url: '/exercises/bid',
			headers: { 'Authorization': `bearer ${token}` },
		}),
	getExerciseWithConflict: (token) =>
		agent.request({
			method: 'get',
			url: '/exercises/conflict',
			headers: { 'Authorization': `bearer ${token}` },
		}),
	getExerciseById: (token, exercise_id) =>
		agent.request({
			method: 'get',
			url: `/exercise/${exercise_id}`,
			headers: { 'Authorization': `bearer ${token}` },
		}),
	makeBid: (token, exercise_id, bid) =>
		agent.request({
			method: 'post',
			url: `/exercise/${exercise_id}/bid`,
			headers: { 'Authorization': `bearer ${token}` },
			data: { bid },
		}),
	getBidById: (token, bid_id) =>
		agent.request({
			method: 'get',
			url: `/bid/${bid_id}`,
			headers: { 'Authorization': `bearer ${token}` },
		}),
	getBidsByExerciseId: (exercise_id) =>
		agent.request({
			method: 'get',
			url: `/exercise/${exercise_id}/bids`,
		}),
	getUsers: () =>
		agent.request({
			method: 'get',
			url: '/users',
		}),
	getUserById: (uid) =>
		agent.request({
			method: 'get',
			url: `/user/${uid}`,
		}),
	submitComment: (token, exercise_id, text) =>
		agent.request({
			method: 'post',
			url: `/exercise/${exercise_id}/comment`,
			headers: { 'Authorization': `bearer ${token}` },
			data: { text },
		}),
};

const handleSuccess = (apiCall, res) => {
	const { success, error, data } = res.data;
	if (success) {
		console.log(`api call '${apiCall}' succeeded`);
		return data;
	} else {
		console.log(`api call '${apiCall}' failed with error: ${error}`);
		throw error;
	}
};

Object.keys(api).map(key => {
	const fn = api[key];
	const wrapper = (...args) => fn(...args).then(res => handleSuccess(key, res));
	api[key] = wrapper;
	return null;
});

export default api;
