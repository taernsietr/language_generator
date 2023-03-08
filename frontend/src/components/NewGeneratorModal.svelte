<script lang="ts">
    import { generators, unsavedChanges } from "../store";
    import Button from "./Button.svelte";
    import CloseButton from "./CloseButton.svelte";

    export let state: boolean;
    let generatorNameInput: string;

    async function closeModal() {
        state = !state;
    }

    async function createAndClose() {
        let temp = $generators;

        // TODO: validate input
        if(temp.find(element => element == generatorNameInput)) {
            alert("Invalid generator name. A generator with that name already exists.");
        } else {
            temp.push(generatorNameInput);
            generators.set(temp);
            unsavedChanges.set(true);
            closeModal();
        };
    }
</script>

<div class="fixed top-0 left-0 flex justify-center items-center opacity-90 bg-black w-screen h-screen">
    <div class="bg-bg1 p-2 m-2 flex flex-col relative justify-center max-w-md shadow-xl text-fg">
        <CloseButton fn={closeModal} />
        <h2 class="text-center text-green p-2 m-2">New Generator</h2>
        <p class="p-2 m-2 opacity-100">Type in a name for the new generator and click the Confirm button, or click the <span class="text-red">X</span> to cancel.</p>
        <input class="bg-bg2 p-2 m-2 text-center no-spinner text-yellow placeholder:opacity-50" type="text" placeholder="generator name" bind:value={generatorNameInput} />
        <Button fn={createAndClose} label={"Confirm"} />
    </div>
</div>
