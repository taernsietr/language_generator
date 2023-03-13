<script lang="ts">  
    import { onMount } from "svelte";
    import { patterns } from "../store.js";
    import Pattern from "./Pattern.svelte";
    import Button from "./Button.svelte";

    function addPattern() {
        let temp = $patterns;
        temp.push({
            "pattern": "",
            "position": "Any",
            "weight": "Default"
        });
        patterns.set(temp);
        console.log($patterns);
    }
    
    let loaded = false;

    onMount(() => {
        setTimeout(() => {loaded = true}, 300);
    });
</script>

<div class="flex flex-col m-2 p-2 justify-between">
    <h3 class="text-center text-blue">Patterns</h3>
    <Button fn={addPattern} label={"Add new pattern"} />
    <div class="flex flex-row flex-wrap justify-between">
    {#if loaded}
        {#each $patterns as pat}
            <Pattern pattern={pat.pattern} position={pat.position} weight={pat.weight} id={$patterns.indexOf(pat)} />
        {/each}
    {/if}
    </div>
</div>

