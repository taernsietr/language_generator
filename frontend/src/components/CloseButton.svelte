<script lang="ts">  
    export let fn: any;
</script>

<button class="absolute top-0 right-0 bg-bg2 p-2 text-red hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={fn} >X</button>
