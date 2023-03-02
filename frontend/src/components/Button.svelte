<script lang="ts">  
    export let fn: any;
    export let label: string;
</script>

<button class="bg-bg2 basis-1/5 m-2 p-2 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={fn}>{label}</button>
