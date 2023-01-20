<script lang="ts">  
    import { onMount } from 'svelte';
    import { displaySettings, generators, currentGenerator, currentSettings } from '../store.js';
    import SettingsSelector from './SettingsSelector.svelte';
    import CategoriesList from './CategoriesList.svelte';
    import PatternsList from './PatternsList.svelte';

    async function loadJSON(url: any) {
        let data = await fetch(`http://127.0.0.1:8080/api/${url}`);
        return await data.json();
    }

    onMount(async () => {
        let data = await loadJSON("generators");
        generators.set(data.generators);
        data = await loadJSON(`settings?generator=${$currentGenerator}`);
        currentSettings.set(data);
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
