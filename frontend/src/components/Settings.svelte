<script lang="ts">  
    import { onMount } from 'svelte';
    import { displaySettings, generators, currentGenerator, categories, patterns, unsavedChanges } from '../store.js';
    import { loadJSON } from '../helpers.ts';
    import SettingsSelector from './SettingsSelector.svelte';
    import CategoriesList from './CategoriesList.svelte';
    import PatternsList from './PatternsList.svelte';

    onMount(async () => {
        let data = await loadJSON("generators");
        generators.set(data.generators);
        currentGenerator.set(data.generators[0]);
        data = await loadJSON(`settings?generator=${$currentGenerator}`);
        categories.set(data.categories);
        patterns.set(data.patterns);
    });
</script>

{#if $displaySettings}
<div class="bg-bg1 col-span-5 flex-1 flex-col m-4 p-4 shadow-xl">
    <h2 class="text-center text-green">Settings</h2>

    <SettingsSelector />
    <CategoriesList />
    <PatternsList />
</div>
{/if}
