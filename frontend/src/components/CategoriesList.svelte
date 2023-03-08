<script lang="ts">  
    import { onMount } from 'svelte';
    import { categories } from '../store.js';
    import Category from './Category.svelte';
    import Button from './Button.svelte';

    function addCategory() {
        let temp = $categories;
        temp.push(
            [
                {
                    symbol: "",
                    elements: [""]
                }
            ]
        );
        categories.set(temp);
    }
    
    let loaded = false;

    onMount(() => {
        setTimeout(() => {loaded = true}, 300);
    });

/*
        {#each $categories as cat}
            <Category symbol={cat.symbol} elements={String(cat.elements).replace(/,/g, " ")} id={$categories.indexOf(cat)} />
        {/each}
*/
</script>

<div class="flex flex-col m-2 p-2 place-content-center">
    <h3 class="text-center text-blue">Categories</h3>
    <Button fn={addCategory} label={"Add new category"} />
    {#if loaded}
        {#each $categories as cat}
            <Category symbol={cat[0]} elements={String(cat[1]).replace(/,/g, " ")} id={$categories.indexOf(cat)} />
        {/each}
    {/if}
</div>
