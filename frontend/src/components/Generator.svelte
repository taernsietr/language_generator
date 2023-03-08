<script lang="ts">
    import { minSyllables, maxSyllables, textLength, results, currentGenerator } from '../store.js';
    import { api_address } from '$lib/env';
    import Button from './Button.svelte';

    async function getRandomText(length: number = 1) {
        let response = await fetch(`${api_address}/text?generator=${$currentGenerator}&min=${$minSyllables}&max=${$maxSyllables}&text_length=${length}`, { credentials: "same-origin" })
        let data = await response.text();
        results.set(data);
    }
</script>

<div class="bg-bg1 flex-1 flex flex-col m-2 p-2 place-content-center shadow-xl">
    <h2 class="basis-1 text-center text-green">Generation Settings</h2>
    <div class="flex flex-row">
        <div class="bg-bg1 flex-1 flex flex-col flex-nowrap place-content-center">
            <div class="flex flex-nowrap m-2 p-2 place-content-center">
                <label class="place-content-center m-auto text-fg" for="minSyllables">Min syllables:</label>
                <input class="bg-bg2 text-yellow text-center no-spinner p-2" type="number" bind:value={$minSyllables} min="1" max="255" id="minSyllables" name="minSyllables">
            </div>

            <div class="flex flex-nowrap m-2 p-2 place-content-center">
                <label class="place-content-center m-auto text-fg" for="maxSyllables">Max syllables:</label>
                <input class="bg-bg2 text-yellow text-center no-spinner p-2" type="number" bind:value={$maxSyllables} min="1" max="255" id="maxSyllables" name="maxSyllables">
            </div>

            <div class="flex flex-nowrap m-2 p-2 place-content-center">
                <label class="place-content-center m-auto text-fg" for="textLength">Text length:</label>
                <input class="bg-bg2 text-yellow text-center no-spinner p-2" type="number" bind:value={$textLength} min="1" max="128" id="textLength" name="textLength">
            </div>
        </div>

        <div class="bg-bg1 flex-1 flex flex-col flex-nowrap place-content-center">
            <Button fn={ () => { getRandomText($textLength) } } label={"Random Text"} />
            <Button fn={ () => { getRandomText() } } label={"Random Word"} />
        </div>
    </div>
</div>

