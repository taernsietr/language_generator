<script lang="ts">
    import { categories, queuedCategories, unsavedChanges } from '../store';
    import DynamicCloseButton from './DynamicCloseButton.svelte';

    export let id: string;
    export let symbol: string;
    export let elements: string;

    async function destroyCategory() {
        let temp = $categories;
        temp.splice(id, 1);
        categories.set(temp);
        unsavedChanges.set(true);
    }

    async function updateCategory() {
        setTimeout(() => {
            unsavedChanges.set(true);
            let temp = $queuedCategories;
            temp[id][0] = symbol;
            temp[id][1] = elements;
            queuedCategories.set(temp);
        }, 100);
    }

</script>

<div class="flex flex-row justify-between p-2" id={id}>
    <input class="bg-bg2 m-2 p-2 text-center text-yellow" type="text" maxlength="1" bind:value={symbol} on:input={updateCategory} />
    <input class="basis-9/12 m-2 p-2 bg-bg2 text-center text-yellow" type="text" bind:value={elements} on:input={updateCategory} />
    <DynamicCloseButton fn={destroyCategory} />
</div>

