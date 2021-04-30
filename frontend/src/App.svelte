<script lang="ts">
	import { Button } from "carbon-components-svelte";
	import Info from "./components/Info.svelte";

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
	<div>
		<h1>Status: {name}!</h1>
		<Button kind="danger" on:click={buttonClick} style="text-align:center">Request Le Poggers</Button>
	</div><br>

	<div class="flexList">
		<Info></Info>
		<Info></Info>
		<Info></Info>
	</div>
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

	.flexList {
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>