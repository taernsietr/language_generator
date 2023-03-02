<script lang="ts">  
    import { onMount } from 'svelte';
    import { displaySettings, generators, currentGenerator, categories, patterns } from '../store.js';
    import CategoriesList from './CategoriesList.svelte';
    import PatternsList from './PatternsList.svelte';
    import NewGeneratorModal from './NewGeneratorModal.svelte';
    import Button from './Button.svelte';
    import { api_address } from '$lib/env.js';
    import { parseCatsFromJSONData, parseCatsToJSON } from '../helpers.js';

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
        console.log(data);
        categories.set(parseCatsFromJSONData(data.categories));
        patterns.set(data.patterns);
    }

    async function saveSettings() { 
        let parsedCategories = parseCatsToJSON($categories);
        let settings = {
            name: $currentGenerator,
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

    let modal: boolean = false;
    async function newGeneratorModal() {
        modal = !modal;
    }
    
    async function clearSettings() { 
        patterns.set([["", "Any", "Default"]]);
        categories.set([["", ""]]);
    }
</script>

{#if $displaySettings}
<div class="bg-bg1 flex flex-col m-2 p-2 shadow-xl">
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
        <Button fn={newGeneratorModal} label={"New Generator"} />
        <Button fn={saveSettings} label={"Save Settings"} />
        <Button fn={clearSettings} label={"Clear Settings"} />
        <Button fn={loadSettings} label={"Reload Generators"} />
    </div>

    <CategoriesList />
    <PatternsList />
    {#if modal}
        <NewGeneratorModal bind:state={modal}/>
    {/if}
</div>
{/if}

