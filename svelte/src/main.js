import App from './App.svelte';
import responseJSON from './response.js';
import { combineDescriptions } from './trades.js';

const trades = combineDescriptions(responseJSON);

console.log(trades[0]);
const app = new App({
	target: document.body,
	props: {
		name: 'Julia',
		trades
	}
});


export default app;