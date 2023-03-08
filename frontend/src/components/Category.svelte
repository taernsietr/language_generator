<script lang="ts">
    import { categories } from '../store.js';
    import Button from './Button.svelte';

    export let elements: string;
    export let symbol: string;
    export let id: string;

    async function destroyCategory() {
        let temp = $categories;
        temp.splice(id, 1);
        categories.set(temp);
    }

    async function updateCategories() {
        elements = String(elements).replace(/,/g, " ").split(" ").join().trim();

        let temp = $categories;
        temp.symbol = symbol;
        temp.elements = elements;
        // temp[id][0] = symbol;
        // temp[id][1] = elements;
        categories.set(temp);
    }
</script>

<div class="flex flex-row justify-between p-2" id={id}>
    <input class="bg-bg2 mr-2 p-2 text-center text-yellow" type="text" maxlength="1" bind:value={symbol} on:input={updateCategories} />
    <input class="basis-9/12 mx-2 p-2 bg-bg2 text-center text-yellow" type="text" bind:value={elements} on:input={updateCategories} />
    <Button fn={destroyCategory} textColor={"text-red"} label={"X"} />
</div>

