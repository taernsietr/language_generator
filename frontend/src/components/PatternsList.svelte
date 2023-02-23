<script lang="ts">  
    import { patterns } from "../store.js";
</script>

<div class="flex flex-col m-2 p-2 justify-between">
    <h3 class="text-center text-blue">Patterns</h3>
    <input class="bg-bg2 text-center m-2 p-2 no-spinner text-yellow" type='text' bind:value={$patterns} />
</div>

