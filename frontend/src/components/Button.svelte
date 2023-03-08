<script lang="ts">
    export let fn: any;
    export let label: string;
    export let textColor: string = "text-fg";
    export let bgColor: string = "bg-bg2";
    export let hoverColor: string = "bg-bg3";
</script>

<button class={`${bgColor} ${textColor} hover:${hoverColor} basis-1/5 m-2 p-2 transition duration-400`} type="submit" on:click={fn}>{label}</button>
