<script lang="ts">  
    import { patterns } from "../store.js";

    let pats: any;

    patterns.subscribe(settings => {
        let temp = String(settings).replace(/,/g, " ");
        pats = temp;
    });
</script>

<div class="flex-row m-4 p-4 place-content-center">
    <h3 class="text-center text-blue">Patterns</h3>
    <input class="bg-bg2 text-center ml-2 no-spinner p-2 text-fg" bind:value={pats} type='text' />
</div>

