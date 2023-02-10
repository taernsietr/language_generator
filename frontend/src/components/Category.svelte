<script lang="ts">
    import { categories } from '../store.js';

    export let elements: string;
    export let symbol: string;
    export let id: string;

    async function destroyCategory() {
        let temp = $categories;
        console.log(id);
        temp.splice(id, 1);
        categories.set(temp);
    }

    async function updateCategories() {

        elements = String(elements).replace(/,/g, " ").split(" ");

        let temp = $categories;
        temp[id][0] = symbol;
        temp[id][1] = elements;
        categories.set(temp);
    }
</script>

<div class="grid grid-flow-row grid-cols-10 m-4 p-4" id={id}>
    <input class="bg-bg2 col-span-2 text-center ml-2 p-2 text-fg" type="text" maxlength="1" bind:value={symbol} on:input={updateCategories} />
    <input class="bg-bg2 col-span-7 text-center ml-2 p-2 text-fg" type="text" bind:value={elements} on:input={updateCategories} />
    <button class="bg-bg2 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={destroyCategory} >X</button>
</div>

