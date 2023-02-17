<script lang="ts">  
    import { onMount } from 'svelte';
    import { displaySettings, generators, currentGenerator, categories, patterns, unsavedChanges } from '../store.js';
    import { loadJSON } from '../helpers.js';
    import CategoriesList from './CategoriesList.svelte';
    import PatternsList from './PatternsList.svelte';

    onMount(async () => {
        await loadGenerators();
        await loadSettings();
    });

    function parseCats(data: any) { 
        let k = Object.keys(data);
        let v = Object.values(data);

        let cats = k.map((e, i) => {
            return [e, v[i]];
        });

        return cats;
    }

    async function loadGenerators() {
        // get generators loaded on the backend and set the first active 
        let data = await loadJSON("generators");
        generators.set(data.generators);
        currentGenerator.set(data.generators[0]);
    }

    async function loadSettings() {
        // load settings for the active generator
        let data = await loadJSON(`settings?generator=${$currentGenerator}`);
        categories.set(parseCats(data.categories));
        patterns.set(String(data.patterns).replace(/,/g, " "));
    }

    async function saveSettings() { 

        // TODO: remover isso, a.k.a. a pior gambiarra da minha vida
        // for future reference, this will get the destructured categories which are passed around on the frontend
        // and through string manipulation reforms it into the JSON format the backend understands
        // ... just to turn it into an object again so we can rebuild the actual JSON body that's expected
        let parsedCategories = JSON.stringify($categories);
        parsedCategories = parsedCategories.replace(/^./, "{");
        parsedCategories = parsedCategories.replace(/.$/, "}");
        parsedCategories = parsedCategories.replace(/\["([A-Z])",/g, "\"$1\":");
        parsedCategories = parsedCategories.replace(/\]\]/g, "]");
        parsedCategories = JSON.parse(parsedCategories);

        let settings = {
            name: $currentGenerator,
            categories: parsedCategories,
            patterns: $patterns.split(" ")
        };

        fetch("http://127.0.0.1:8080/api/update", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(settings),
        })
            .catch((error) => {
                console.error("Error:", error);
            });
    }

    async function newSettings() {

    }

    async function clearSettings() { console.log("clearSettings") }
</script>

{#if $displaySettings}
<div class="bg-bg1 col-span-5 flex-1 flex-col m-4 p-4 shadow-xl">
    <h2 class="text-center text-green">Settings</h2>

    <div class="flex-col m-4 p-4 place-content-center">
        <h3 class="text-center text-blue">Available Presets</h3>
        <select class="bg-bg2 text-center ml-2 no-spinner p-2 text-fg" bind:value={$currentGenerator} on:change={loadSettings} >
            {#each $generators as option}
                <option value={option}>{option}</option>
            {/each}
        </select>
        <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={saveSettings}>Save Settings</button>
        <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={loadSettings}>Load Settings</button>
        <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={clearSettings}>Clear Settings</button>
    </div>

    <CategoriesList />
    <PatternsList />
</div>
{/if}

