<script lang="ts">  
    import { onMount } from 'svelte';
    import { categories } from '../store.js';
    import Category from './Category.svelte';

    let k: string[];
    let v: Array<String>;
    let cats = [["", ""]];

    function parseCategories() { 
        k = Object.keys($categories);
        v = Object.values($categories);
        cats = k.map((e, i) => {
            return [e, v[i]];
        });
    }

    function addCategory() {
        cats = cats;
        cats.push(["", [""]]);
    }

    onMount(async () => {
        setTimeout(() => { parseCategories(); }, 100);
    });
</script>

<div class="flex-col m-4 p-4 place-content-center">
    <h3 class="text-center text-blue">Categories</h3>
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type='submit' on:click={addCategory}>Add new category</button>
    {#each cats as cat}
        <Category symbol={cat[0]} elements={cat[1]} id={String(cats.indexOf(cat))}/>
    {/each}
</div>
