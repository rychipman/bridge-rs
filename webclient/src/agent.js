import axios from "axios";

const agent = axios.create({
	baseURL: '',
	timeout: 15000,
})

const api = {
	login: (email, password) =>
		agent.request({
			method: 'post',
			url: '/api/login',
			data: { email, password },
		}),
	register: (email, password) =>
		agent.request({
			method: 'post',
			url: '/api/register',
			data: { email, password },
		}),
	getExercise: (token) =>
		agent.request({
			method: 'get',
			url: '/api/exercises/bid',
			headers: { 'Authorization': `bearer ${token}` },
		}),
	getExerciseWithConflict: (token) =>
		agent.request({
			method: 'get',
			url: '/api/exercises/conflict',
			headers: { 'Authorization': `bearer ${token}` },
		}),
	getExerciseById: (token, exercise_id) =>
		agent.request({
			method: 'get',
			url: `/api/exercise/${exercise_id}`,
			headers: { 'Authorization': `bearer ${token}` },
		}),
	makeBid: (token, exercise_id, bid) =>
		agent.request({
			method: 'post',
			url: `/api/exercise/${exercise_id}/bid`,
			headers: { 'Authorization': `bearer ${token}` },
			data: { bid },
		}),
	getBidById: (token, bid_id) =>
		agent.request({
			method: 'get',
			url: `/api/bid/${bid_id}`,
			headers: { 'Authorization': `bearer ${token}` },
		}),
	getBidsByExerciseId: (exercise_id) =>
		agent.request({
			method: 'get',
			url: `/api/exercise/${exercise_id}/bids`,
		}),
	getUsers: () =>
		agent.request({
			method: 'get',
			url: '/api/users',
		}),
	getUserById: (uid) =>
		agent.request({
			method: 'get',
			url: `/api/user/${uid}`,
		}),
	submitComment: (token, exercise_id, text) =>
		agent.request({
			method: 'post',
			url: `/api/exercise/${exercise_id}/comment`,
			headers: { 'Authorization': `bearer ${token}` },
			data: { text },
		}),
};

const handleSuccess = (apiCall, res) => {
	console.log(`api call '${apiCall}' succeeded`);
	return res.data;
};

const handleFailure = (apiCall, err) => {
	console.log(`api call '${apiCall}' failed`);
	console.log(err.response)
	if (err.response.data != "") {
		throw err.response.data;
	}
	throw err;
};

Object.keys(api).map(key => {
	const fn = api[key];
	const wrapper = (...args) => (
		fn(...args)
			.then(res => handleSuccess(key, res))
		    .catch(err => handleFailure(key, err))
	);
	api[key] = wrapper;
	return null;
});

export default api;
