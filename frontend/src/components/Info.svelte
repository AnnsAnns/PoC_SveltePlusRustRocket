<script lang="ts">
    import { ImageLoader, Tile, Grid, Row, Column } from "carbon-components-svelte";

    export let anime_id: string;
    export let img_url: string;
    export let anime_name: string;
    let response: JSON;

    function x() {
        return;
    }

    if (img_url != "") {
        img_url = "https://media.notify.moe/images/anime/large/" + anime_id + ".webp";
    }

</script>

<main>
    <Tile>
        <div class="flexing">
            <div class="image">
                <ImageLoader
                    fadeIn
                    src={img_url}>
                </ImageLoader>
            </div>
            <div class="text"> 
                Name: {anime_name} <br>
                Mapping ID: {anime_id}
            </div>
        </div>
    </Tile>
</main>

<style>
    main {
        text-align: center;
        min-width: 30%;
    }

    .flexing {
        display: flex;
        flex-wrap: nowrap;
        align-content: center;
        justify-content: center;
        width: 33%;
    }

    .flexing > div {
        text-align: left;
    }

    .image {
        flex: 0 0 30%;
        margin: 0 auto;
    }

    .text {
        flex: 0 0 60%;
        margin: 0 auto;
    }
</style>
