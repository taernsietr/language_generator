<script lang="ts">  
    import { onMount } from 'svelte';
    import { categories } from '../store.js';
    import Category from './Category.svelte';
    import Button from './Button.svelte';

/*  THIS SHOULD WORK WITH THE LABELED OBJECTS
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
*/
    function addCategory() {
        let temp = $categories;
        temp.push(["", [""]]);
        categories.set(temp);
        console.log($categories);
    }
    
    let loaded = false;

    onMount(() => {
        setTimeout(() => {loaded = true}, 300);
    });

/*  THIS SHOULD WORK WITH THE LABELED OBJECTS
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
