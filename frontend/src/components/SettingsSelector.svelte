<script lang="ts">
    import { generators, currentGenerator } from '../store.js';

    function saveSettings() { console.log("saveSettings") }
    function loadSettings() { console.log("loadSettings") }
    function clearSettings() { console.log("clearSettings") }
</script>

<div class="flex-col m-4 p-4 place-content-center">
    <h3 class="text-center text-blue">Available Presets</h3>
    <span>Current generator: {$currentGenerator}</span>
    <select class="bg-bg2 text-center ml-2 no-spinner p-2 text-fg" bind:value={$currentGenerator}>
        {#each $generators as option}
            <option value={option}>{option}</option>
        {/each}
    </select>
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={saveSettings}>Save Settings</button>
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={loadSettings}>Load Settings</button>
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={clearSettings}>Clear Settings</button>
</div>
