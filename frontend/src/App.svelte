<script lang="ts">
	import { Button } from "carbon-components-svelte";

	export let name: string;

	function buttonClick() {
		const Http = new XMLHttpRequest();
		Http.open("GET", "http://localhost:8000/test");
		Http.send();

		Http.onreadystatechange = (e) => {
			name = Http.responseText
			console.log(name);
		}
	}

</script>

<main>
	<h1>Status: {name}!</h1>
	<Button kind="danger" on:click={buttonClick} style="text-align:center">Request Le Poggers</Button>
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
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>