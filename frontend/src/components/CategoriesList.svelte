<script lang="ts">  
    import { currentSettings } from '../store.js';
    import Category from './Category.svelte';

    let categories: any;

    currentSettings.subscribe(settings => {
        let symbols = Object.keys(settings.categories);
        let cats = Object.values(settings.categories);
        categories = symbols.map((sym, i) => {
            return [sym, cats[i]];
        });
    });

    function addCategory() { 
        // currentSettings.update();
        //console.log("addCategory") 
    }
</script>

<div class="flex-col m-4 p-4 place-content-center">
    <h3 class="text-center text-blue">Categories</h3>
        {#each categories as cat}
            <Category elements={cat[1]} symbol={cat[0]} />
        {/each}
<button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type='submit' on:click={addCategory}>Add new category</button>
</div>
