<script>
	import Photo from './Photo.svelte';
	import Trade from './Trade.svelte';
	
	export let name;
	export let trades;
	
	const photo = {
		name: 'pizza'
	};
</script>

<main>
	<h1>Hello {name}!</h1>
	<Photo {...photo}/>
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