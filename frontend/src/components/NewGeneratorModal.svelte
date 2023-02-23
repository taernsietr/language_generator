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

<div class="fixed top-0 left-0 w-full h-screen">
    <div class="backdrop-opacity-30 bg-black absolute w-full h-full">
        <div class="bg-bg1 col-span-4 flex-1 flex-col m-4 p-4 place-content-center shadow-xl text-fg">
            <h2 class="text-center text-green">New Generator</h2>
            <p>Click anywhere to cancel, or type a name for the new generator and press Enter</p>
            <input class="bg-bg2 text-center ml-2 no-spinner p-2 text-fg" type='text' bind:value={generatorNameInput} />
            <button class="bg-bg2 p-2 ml-2 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={createAndClose} >Create New Generator</button>
            <button class="bg-bg2 p-2 ml-2 text-red hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={closeModal} >X</button>
        </div>
    </div>
</div>
