<script lang="ts">  
    import { onMount } from 'svelte';
    import { api_address } from '$lib/env.js';
    import { currentlyDisplaying, displaySettings, currentGenerator, generators, categories, patterns } from '../store.js';
    import { loadGenerators, loadSettings, saveSettings } from '../helpers.js';
    import CategoriesList from './CategoriesList.svelte';
    import PatternsList from './PatternsList.svelte';
    import NewGeneratorModal from './NewGeneratorModal.svelte';
    import Button from './Button.svelte';

    onMount(async () => {
        await loadGenerators();
        await loadSettings($currentGenerator);
    });

    // TODO: properly implement this
    async function newRandomGenerator() {
        fetch(`${api_address}/random_generator`); 
        loadGenerators();
        currentGenerator.set("alpha");
        loadSettings($currentGenerator);
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
            <select class="bg-bg2 text-center m-2 p-2 no-spinner text-yellow" bind:value={$currentGenerator} on:change={() => { loadSettings($currentGenerator) } } >
                {#each $generators as option}
                    <option value={option}>{option}</option>
                {/each}
            </select>
        </div>
        <Button fn={ () => { currentlyDisplaying.set("NewGeneratorModal") }} label={"New Generator"} />
        <Button fn={ () => { loadSettings($currentGenerator) }} label={"Reload Generators"} />
        <Button fn={ () => { saveSettings($currentGenerator, $categories, $patterns) }} label={"Save Settings"} />
        <Button fn={clearSettings} label={"Clear Settings"} />
        <Button fn={newRandomGenerator} label={"Random Generator"} />
    </div>

    <CategoriesList />
    <PatternsList />
    <NewGeneratorModal />
</div>
{/if}

