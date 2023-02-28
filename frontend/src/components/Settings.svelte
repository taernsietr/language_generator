<script lang="ts">  
    import { onMount } from 'svelte';
    import { displaySettings, generators, currentGenerator, categories, patterns, unsavedChanges } from '../store.js';
    import CategoriesList from './CategoriesList.svelte';
    import PatternsList from './PatternsList.svelte';
    import NewGeneratorModal from './NewGeneratorModal.svelte';
    import { api_address } from '$lib/env.js';

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
        let data = await fetch(`${api_address}/generators`, { credentials: "same-origin" });
        data = await data.json();
        generators.set(data.generators);
        currentGenerator.set(data.generators[0]);
    }

    async function loadSettings() {
        // load settings for the active generator
        let data = await fetch(`${api_address}/settings?generator=${$currentGenerator}`, { credentials: "same-origin" });
        data = await data.json();
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
        
        fetch(`${api_address}/save`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(settings),
        })
            .catch((error) => { console.error("Error:", error); });
    }

    let modal: boolean = false;
    async function newGeneratorModal() {
        modal = !modal;
    }
    
    async function clearSettings() { 
        patterns.set("");
        categories.set([["", ""]]);
    }

</script>

{#if $displaySettings}
<div class="bg-bg1 col-span-5 flex flex-col m-2 p-2 shadow-xl">
    <h2 class="basis-1 text-center text-green">Settings</h2>
    <div class="flex flex-row justify-between m-2 p-2">
        <div class="flex flex-col basis-1/5">
            <h3 class="text-center text-blue">Generator</h3>
            <select class="bg-bg2 text-center m-2 p-2 no-spinner text-yellow" bind:value={$currentGenerator} on:change={loadSettings} >
                {#each $generators as option}
                    <option value={option}>{option}</option>
                {/each}
            </select>
        </div>
        <button class="bg-bg2 basis-1/5 m-2 p-2 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={newGeneratorModal}>New Generator</button>
        <button class="bg-bg2 basis-1/5 m-2 p-2 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={saveSettings}>Save Settings</button>
        <button class="bg-bg2 basis-1/5 m-2 p-2 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={clearSettings}>Clear Settings</button>
        <button class="bg-bg2 basis-1/5 m-2 p-2 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={loadSettings}>Reload Generators</button>
    </div>

    <CategoriesList />
    <PatternsList />
    {#if modal}
        <NewGeneratorModal bind:state={modal}/>
    {/if}
</div>
{/if}

