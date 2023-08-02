<script lang="ts">
    import { categories } from '../store.js';
    import DynamicCloseButton from './DynamicCloseButton.svelte';

    export let elements: string;
    export let symbol: string;
    export let id: string;

    async function destroyCategory() {
        let temp = $categories;
        temp.splice(id, 1);
        categories.set(temp);
    }

    async function updateCategories() {
        setTimeout(() => {
            elements = String(elements).replace(/,/g, " ").split(" ").join().trim();

            let temp = $categories;
            temp[id][0] = symbol;
            temp[id][1] = elements;
            categories.set(temp);
        }, 300);
    }

</script>

<div class="flex flex-row justify-between p-2" id={id}>
    <input class="bg-bg2 m-2 p-2 text-center text-yellow" type="text" maxlength="1" bind:value={symbol} on:input={updateCategories} />
    <input class="basis-9/12 m-2 p-2 bg-bg2 text-center text-yellow" type="text" bind:value={elements} on:input={updateCategories} />
    <DynamicCloseButton fn={destroyCategory} />
</div>

