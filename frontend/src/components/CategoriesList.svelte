<script lang="ts">  
    import { onMount } from 'svelte';
    import { categories } from '../store.js';
    import Category from './Category.svelte';

    function addCategory() {
        let temp = $categories;
        temp.push(["", [""]]);
        categories.set(temp);
    }
    
    let loaded = false;

    onMount(() => {
        setTimeout(() => {loaded = true}, 300);
    });

</script>

<div class="flex-col m-4 p-4 place-content-center">
    <h3 class="text-center text-blue">Categories</h3>
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type='submit' on:click={addCategory} >Add new category</button>
    {#if loaded}
        {#each $categories as cat}
            <Category symbol={cat[0]} elements={String(cat[1]).replace(/,/g, " ")} id={$categories.indexOf(cat)} />
        {/each}
    {/if}
</div>
