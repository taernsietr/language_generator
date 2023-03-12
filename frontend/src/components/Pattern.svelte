<script lang="ts">
    import { patterns } from '../store.js';
    import DynamicCloseButton from './DynamicCloseButton.svelte';

    export let pattern: string;
    export let position: string;
    export let weight: string;
    export let id: string;

    async function destroyPattern() {
        let temp = $patterns;
        temp.splice(id, 1);
        patterns.set(temp);
    }

    async function updatePatterns() {
        let temp = $patterns;
        temp[id][0] = pattern;
        temp[id][1] = position;
        temp[id][2] = weight;
        patterns.set(temp);
    }
</script>

<div class="flex flex-row justify-between p-2" id={id}>
    <input class="bg-bg2 m-2 p-2 text-center text-yellow max-w-[100px]" type="text" bind:value={pattern} on:input={updatePatterns} />
    <select class="bg-bg2 text-center m-2 p-2 no-spinner text-yellow" bind:value={position} on:change={updatePatterns} >
        <option value="Any">Any</option>
        <option value="Initial">Initial</option>
        <option value="Medial">Medial</option>
        <option value="Final">Final</option>
        <option value="NonInitial">NonInitial</option>
        <option value="NonMedial">NonMedial</option>
        <option value="NonFinal">NonFinal</option>
    </select>
    <select class="bg-bg2 text-center m-2 p-2 no-spinner text-yellow" bind:value={weight} on:change={updatePatterns} >
        <option value="Default">Default</option>
        <option value="Light">Light</option>
        <option value="Heavy">Heavy</option>
    </select>
    <DynamicCloseButton fn={destroyPattern} />
</div>

