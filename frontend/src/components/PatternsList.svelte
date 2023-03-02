<script lang="ts">  
    import { onMount } from "svelte";
    import { patterns } from "../store.js";
    import Pattern from "./Pattern.svelte";

    let loaded = false;

    onMount(() => {
        setTimeout(() => {loaded = true}, 300);
    });
</script>

<div class="flex flex-col m-2 p-2 justify-between">
    <h3 class="text-center text-blue">Patterns</h3>
    <div class="flex flex-row flex-wrap justify-between">
    {#if loaded}
        {#each $patterns as pat}
            <Pattern pattern={pat.pattern} position={pat.position} weight={pat.weight} id={$patterns.indexOf(pat)} />
        {/each}
    {/if}
    </div>
</div>

