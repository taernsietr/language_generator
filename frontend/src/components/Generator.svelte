<script>
    import { minSyllables, maxSyllables, textLength, results } from '../store.js';

    async function api(url) {
        return await fetch(url, { credentials: "same-origin" })
    }

    async function getRandomWord() {
        let response = await api(`http://127.0.0.1:8080/api/word?generator=default-settings&min=${$minSyllables}&max=${$maxSyllables}&text_length=${$textLength}`)
        let data = await response.text();
        results.set(data);
    }

    async function getRandomText() {
        let response = await api(`http://127.0.0.1:8080/api/text?generator=default-settings&min=${$minSyllables}&max=${$maxSyllables}&text_length=${$textLength}`)
        let data = await response.text();
        results.set(data);
    }
</script>

<div class="bg-bg1 col-span-3 flex-1 flex m-4 p-4 place-content-center shadow-xl">
    <div class="bg-bg1 flex-1 flex flex-col flex-nowrap place-content-center">
        <div class="flex flex-nowrap m-4 p-4 place-content-center">
            <label class="place-content-center m-auto text-fg" for="minSyllables">Min syllables:</label>
            <input class="bg-bg1 text-fg text-center ml-2 no-spinner p-2" type="number" bind:value={$minSyllables} min="1" max="255" id="minSyllables" name="minSyllables">
        </div>

        <div class="flex flex-nowrap m-4 p-4 place-content-center">
            <label class="place-content-center m-auto text-fg" for="maxSyllables">Max syllables:</label>
            <input class="bg-bg1 text-fg text-center ml-2 no-spinner p-2" type="number" bind:value={$maxSyllables} min="1" max="255" id="maxSyllables" name="maxSyllables">
        </div>

        <div class="flex flex-nowrap m-4 p-4 place-content-center">
            <label class="place-content-center m-auto text-fg" for="textLength">Text length:</label>
            <input class="bg-bg1 text-fg text-center ml-2 no-spinner p-2" type="number" bind:value={$textLength} min="1" max="128" id="textLength" name="textLength">
        </div>
    </div>

    <div class="bg-bg1 flex-1 flex flex-col flex-nowrap place-content-center">
        <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={getRandomText}>Random Text</button>
        <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={getRandomWord}>Random Word</button>
    </div>
</div>

