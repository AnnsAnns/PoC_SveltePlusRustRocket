<script lang="ts">
    import { ImageLoader, Tile, Grid, Row, Column } from "carbon-components-svelte";

    export let anime_id: string = "WLWV2Kimg";

    function x() {
        let Http = new XMLHttpRequest
        Http.open("GET", "http://localhost:8000/request/" + anime_id);
        Http.send();

        Http.onreadystatechange = (e) => {
            response = JSON.parse(Http.response)
            console.log(response);
        }
    }

    let img_url: string = "https://media.notify.moe/images/anime/original/WLWV2Kimg.png";
    let anime_name: string = "Houseki No Kuni";
    let response: JSON;

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
