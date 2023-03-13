<script lang="ts">  
    import { onMount } from 'svelte';
    import { api_address } from '$lib/env.js';
    import { currentlyDisplaying, displaySettings, generators, currentGenerator, categories, patterns } from '../store.js';
    import { parseCatsFromJSONData, parseCatsToJSON } from '../helpers.js';
    import CategoriesList from './CategoriesList.svelte';
    import PatternsList from './PatternsList.svelte';
    import NewGeneratorModal from './NewGeneratorModal.svelte';
    import Button from './Button.svelte';

    onMount(async () => {
        await loadGenerators();
        await loadSettings();
    });

    async function loadGenerators() {
        // get generators loaded on the backend and set the first active 
        let data = await fetch(api_address, { credentials: "same-origin" });
        data = await data.json();
        generators.set(data.generators);
        currentGenerator.set(data.generators[0]);
    }

    async function loadSettings() {
        // load settings for the active generator
        let data = await fetch(`${api_address}/settings?generator=${$currentGenerator}`, { credentials: "same-origin" });
        data = await data.json();
        categories.set(parseCatsFromJSONData(data.categories));
        // categories.set(data.categories);
        patterns.set(data.patterns);
    }

    async function saveSettings() { 
        let parsedCategories = parseCatsToJSON($categories);
        let settings = {
            name: $currentGenerator,
        //    categories: $categories,
            categories: parsedCategories,
            patterns: $patterns
        };
        
        fetch(`${api_address}/save`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(settings),
        })
            .catch((error) => { console.error("Error:", error); });
    }

    // TODO: properly implement this
    async function newRandomGenerator() {
        fetch(`${api_address}/testing`); 
        loadGenerators();
        currentGenerator.set("alpha");
        loadSettings();
    }
    
    async function clearSettings() { 
        patterns.set([["", "Any", "Default"]]);
        categories.set([["", ""]]);
        // categories.set([[{symbol: "", elements: [""]}]]);
        
    }
</script>

<NewGeneratorModal />
{#if $displaySettings}
<div class="bg-bg1 basis-full flex flex-col m-2 p-2 shadow-xl">
    <h2 class="text-center text-green">Settings</h2>
    <div class="flex flex-row justify-between m-2 p-2">
        <div class="flex flex-col">
            <h3 class="text-center text-blue">Generator</h3>
            <select class="bg-bg2 text-center m-2 p-2 no-spinner text-yellow" bind:value={$currentGenerator} on:change={loadSettings} >
                {#each $generators as option}
                    <option value={option}>{option}</option>
                {/each}
            </select>
        </div>
        <Button fn={() => { currentlyDisplaying.set("NewGeneratorModal")} } label={"New Generator"} />
        <Button fn={saveSettings} label={"Save Settings"} />
        <Button fn={clearSettings} label={"Clear Settings"} />
        <Button fn={loadSettings} label={"Reload Generators"} />
        <Button fn={newRandomGenerator} label={"New Random Generator"} />
    </div>

    <CategoriesList />
    <PatternsList />
    <NewGeneratorModal />
</div>
{/if}

