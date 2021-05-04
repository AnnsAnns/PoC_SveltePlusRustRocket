<script lang="ts">
	import { Button } from "carbon-components-svelte";
import { each } from "svelte/internal";
	import Info from "./components/Info.svelte";

	export let request_status: string;
	let animelist = [];

	function buttonClick() {
        let Http = new XMLHttpRequest
        Http.open("GET", "https://notify.moe/api/anime/WLWV2Kimg");
        Http.send();

        Http.onreadystatechange = (e) => {
            let _ = JSON.parse(Http.response);
			console.log(_);
			animelist.push(_);
			animelist = animelist;
            console.log(animelist[-1]);
        
			request_status = "Worked!"
		}
	}

</script>

<main>
	<div>
		<h1>Status: {request_status}!</h1>
		<Button kind="danger" on:click={buttonClick} style="text-align:center">Request Le Poggers</Button>
	</div><br>

	<div class="flexList">
		{#each animelist as anime}
			<Info
				anime_id={anime["id"]}
				anime_name={anime["title"]["canonical"]}></Info>
		{/each}
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