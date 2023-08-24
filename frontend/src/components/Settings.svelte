<script lang="ts">  
    import { onMount } from 'svelte';
    import { currentlyDisplaying, displaySettings, currentGenerator, generators, categories, patterns, queuedCategories, queuedPatterns, unsavedChanges } from '../store';

    import loadSettings from '../loadSettings'; 
    import saveSettings from '../saveSettings';
    import newRandomGenerator from '../randomGenerator';

    import Button from './Button.svelte';
    import CategoriesList from './CategoriesList.svelte';
    import PatternsList from './PatternsList.svelte';
    import NewGeneratorModal from './NewGeneratorModal.svelte';

    async function clearSettings() { 
        patterns.set([["", "Any", "Default"]]);
        categories.set([["", ""]]);
    }

    onMount(() => {
        setTimeout(() => { 
            queuedCategories.set($categories);
            queuedPatterns.set($patterns);
        }, 300);
    });

</script>

<NewGeneratorModal />
<div class={`bg-bg1 basis-full ${$displaySettings ? "flex" : "hidden"} flex-col m-2 p-2 shadow-xl`}>
    <h2 class="text-center text-green">Settings</h2>
    <div class="flex flex-row justify-between m-2 p-2">
        <div class="flex flex-col">
            <h3 class="text-center text-blue">Generator</h3>
            <select 
                class="bg-bg2 text-center m-2 p-2 no-spinner text-yellow" 
                bind:value={$currentGenerator} 
                on:change={ () => {
                    if($unsavedChanges) {
                        saveSettings($unsavedChanges, $currentGenerator, $queuedPatterns, $queuedCategories);
                    } 
                    loadSettings($currentGenerator);
                    setTimeout(() => { 
                        queuedCategories.set($categories);
                        queuedPatterns.set($patterns);
                    }, 300);
                   
                }} 
            >
                {#each $generators as option}
                    <option value={option}>{option}</option>
                {/each}
            </select>
        </div>
        <Button fn={ () => { currentlyDisplaying.set("NewGeneratorModal") }} label={"New Generator"} />
        <Button fn={ () => { saveSettings($unsavedChanges, $currentGenerator, $queuedPatterns, $queuedCategories) }} label={"Save Settings"} />
        <Button fn={ () => { loadSettings($currentGenerator) }} label={"Reload Generators"} />
        <Button fn={clearSettings} label={"Clear Settings"} />
        <Button fn={newRandomGenerator} label={"Random Generator"} />
    </div>
    <CategoriesList />
    <PatternsList />
</div>
