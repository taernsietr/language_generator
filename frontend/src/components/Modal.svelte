<script lang="ts">
    import { fade } from 'svelte/transition';
    import CloseButton from "./CloseButton.svelte";

    export let state: boolean;
    export let label: string;

    async function closeModal() {
        state = !state;
    }
</script>

<div class="fixed top-0 left-0 flex justify-center items-center opacity-90 bg-black w-screen h-screen" transition:fade>
    <div class="bg-bg1 p-2 m-2 flex flex-col relative justify-center max-w-md shadow-xl text-fg">
        <CloseButton fn={closeModal} />
        <h2 class="text-center text-green p-2 m-2">{label}</h2>
        <slot></slot>
    </div>
</div>
