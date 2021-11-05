<script>
	import Trade from './Trade.svelte';
	import Fetch from './Fetch.svelte';
	
	export let name;
	export let trades;
	
	const photo = {
		name: 'pizza'
	};
</script>

<main>
	<Fetch/>
	{#each trades as trade}
		<Trade { trade }/>
	{/each}
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>