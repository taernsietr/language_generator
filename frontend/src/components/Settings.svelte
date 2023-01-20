<script>  
    import { onMount } from 'svelte';
    import { displaySettings, generators, currentSettings } from '../store.js';
    import Category from './Category.svelte';

    onMount(async () => {
        let data = await fetch("http://127.0.0.1:8080/api/generators", { credentials: "same-origin" });
        data = await data.json();
        generators.set(data.generators);
        console.log($generators);
    });

    onMount(async () => {
        let data = await fetch("http://127.0.0.1:8080/api/settings?generator=default-settings", { credentials: "same-origin" });
        data = await data.json();
        currentSettings.set(data);
        console.log($currentSettings);
    });

</script>

{#if $displaySettings}
<div class="bg-bg1 col-span-5 flex-1 flex-col m-4 p-4 shadow-xl">
    <h2 class="text-center text-green">Settings</h2>

    <div class="flex-col m-4 p-4 place-content-center">
        <h3 class="text-center text-blue">Available Presets</h3>
        <select class="bg-bg2 text-center ml-2 no-spinner p-2 text-fg">
            {#each $generators as option}
                <option value={option}>{option}</option>
            {/each}
        </select>
    </div>
            
    <div class="flex-col m-4 p-4 place-content-center">
        <h3 class="text-center text-blue">Categories</h3>
        {#each $currentSettings.categories as cat}
            <Category elements={cat} symbol="X" />
        {/each}
<!--
-->
        <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type='submit' on:click{addCategory}>Add new category</button>
    </div>

    <div class="flex-row m-4 p-4 place-content-center">
        <h3 class="text-center text-blue">Patterns</h3>
        <input class="bg-bg2 text-center ml-2 no-spinner p-2 text-fg" type='text' />
    </div>

    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click{saveSettings}>Save Settings</button>
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click{loadSettings}>Load Settings</button>
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click{clearSettings}>Clear Settings</button>
</div>
{/if}
