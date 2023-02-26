<script lang="ts">
    import { generators, unsavedChanges } from "../store";

    export let state: boolean;
    let generatorNameInput: string;

    async function closeModal() {
        state = !state;
    }

    async function createAndClose() {
        let temp = $generators;

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
        <button class="absolute top-0 right-0 bg-bg2 p-2 text-red hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={closeModal} >X</button>
        <h2 class="text-center text-green p-2 m-2">New Generator</h2>
        <p class="p-2 m-2 opacity-100">Type in a name for the new generator and click the Confirm button, or click the <span class="text-red">X</span> to cancel.</p>
        <input class="bg-bg2 p-2 m-2 text-center no-spinner text-yellow" type="text" placeholder="generator name" bind:value={generatorNameInput} />
        <button class="bg-bg2 p-2 m-2 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={createAndClose} >Confirm</button>
    </div>
</div>
