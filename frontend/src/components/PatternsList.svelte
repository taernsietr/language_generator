<script lang="ts">  
    import { patterns, queuedPatterns } from "../store.js";

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
        queuedPatterns.set(temp);
    }
    
</script>

<div class="flex flex-col m-2 p-2 justify-between">
    <h3 class="text-center text-blue">Patterns</h3>
    <Button fn={addPattern} label={"Add new pattern"} />
    <div class="flex flex-row flex-wrap justify-between">
        {#each $patterns as pat}
            <Pattern pattern={pat.pattern} position={pat.position} weight={pat.weight} id={$patterns.indexOf(pat)} />
        {/each}
    </div>
</div>

