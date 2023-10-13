<script lang="ts">
    import { minSyllables, maxSyllables, textLength, results, currentGenerator } from '../store.js';
    import { api_address } from '$lib/env';
    import Button from './Button.svelte';

    async function getRandomText(length: number = 1) {
        let response = await fetch(`${api_address}/words?generator=${$currentGenerator}&min=${$minSyllables}&max=${$maxSyllables}&text_length=${length}`, { credentials: "same-origin" })
        let data = await response.text();
        results.set(data);
    }

    async function getPseudotext(length: number = 10) {
        let response = await fetch(`${api_address}/pseudotext?generator=${$currentGenerator}&min=${$minSyllables}&max=${$maxSyllables}&text_length=${length}`, { credentials: "same-origin" })
        let data = await response.text();
        results.set(data);
    }
        
    /// TODO
    async function convertXSAMPAToIPA(xsampa: string) {
        let data = await fetch(`${api_address}/xsampa-ipa`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(xsampa),
        })
            .catch((error) => { console.error("Error:", error);
        });

        if (data) {
            let response = await data.json();
            results.set(response);
        }
    }

    async function convertIPAToXSAMPA(ipa: string) {
        let data = await fetch(`${api_address}/ipa-xsampa`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(ipa),
        })
            .catch((error) => { console.error("Error:", error);
        });

        if (data) {
            let response = await data.text();
            results.set(response);
        }
    }
</script>

<div class="bg-bg1 basis-full flex flex-col m-2 p-2 place-content-center shadow-xl">
    <h2 class="basis-1 text-center text-green">Generation Settings</h2>
    <div class="flex flex-row">
        <div class="bg-bg1 flex-1 flex flex-col flex-nowrap place-content-center">
            <div class="flex flex-nowrap m-2 p-2 place-content-center">
                <label class="place-content-center m-2 p-2 text-fg" for="minSyllables">Min syllables:</label>
                <input class="bg-bg2 text-yellow m-2 p-2 text-center no-spinner" type="number" bind:value={$minSyllables} min="1" max="255" id="minSyllables" name="minSyllables">
            </div>

            <div class="flex flex-nowrap m-2 p-2 place-content-center">
                <label class="place-content-center m-2 p-2 text-fg" for="maxSyllables">Max syllables:</label>
                <input class="bg-bg2 text-yellow m-2 p-2 text-center no-spinner" type="number" bind:value={$maxSyllables} min="1" max="255" id="maxSyllables" name="maxSyllables">
            </div>

            <div class="flex flex-nowrap m-2 p-2 place-content-center">
                <label class="place-content-center m-2 p-2 text-fg" for="textLength">Text length:</label>
                <input class="bg-bg2 text-yellow m-2 p-2 text-center no-spinner" type="number" bind:value={$textLength} min="1" max="128" id="textLength" name="textLength">
            </div>
        </div>

        <div class="bg-bg1 flex-1 flex flex-col flex-nowrap place-content-center">
            <Button fn={ () => { getPseudotext($textLength) } } label={"Pseudotext"} />
            <Button fn={ () => { getRandomText($textLength) } } label={"Random Words"} />
            <Button fn={ () => { getRandomText() } } label={"Random Word"} />
            <Button fn={ () => { convertXSAMPAToIPA($results) } } label={"Convert X-SAMPA to IPA"} />
            <Button fn={ () => { convertIPAToXSAMPA($results) } } label={"Convert IPA to X-SAMPA"} />
        </div>
    </div>
</div>

