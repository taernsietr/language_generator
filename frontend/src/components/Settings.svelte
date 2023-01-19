<script>
    import Category from './Category.svelte';
    export let display;
</script>

{#if display}
<div class="bg-bg1 col-span-5 flex-1 flex-col m-4 p-4 shadow-xl">
    <h2 class="text-center text-green">Settings</h2>
            
    <div class="flex-col m-4 p-4 place-content-center">
        <h3 class="text-center text-blue">Categories</h3>
        <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type='submit' on:click{addCategory}>Add new category</button>
    </div>

    <div class="flex-row m-4 p-4 place-content-center">
        <h3 class="text-center text-blue">Patterns</h3>
        <input class="bg-bg2 text-center ml-2 no-spinner p-2 text-fg" type='text' />
    </div>

    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click{saveSettings}>Save Settings</button>
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click{loadSettings}>Load Settings</button>
</div>
{/if}
