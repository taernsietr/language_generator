<script lang="ts">
    import { generators, currentlyDisplaying, currentGenerator } from "../store";
    import loadSettings from "../loadSettings";
    import Modal from "./Modal.svelte";
    import Button from "./Button.svelte";
    import saveSettings from "../saveSettings";

    let generatorNameInput: string = "";

    async function createAndClose() {
        let temp = $generators;

        // TODO: validate input
        if(temp.find(element => element == generatorNameInput)) {
            alert("Invalid generator name. A generator with that name already exists.");
        }
        else if(!generatorNameInput.trim().length) {
            alert("Invalid generator name. Field cannot be empty.");
        }
        else {
            temp.push(generatorNameInput);
            generators.set(temp);
            saveSettings(
                true,
                generatorNameInput,
                [{ pattern: "CV", position: "Any", weight: "Default" }],
                [["C", "p"], ["V", "a"]]
            );
            currentGenerator.set(generatorNameInput);
            loadSettings(generatorNameInput);
            currentlyDisplaying.set("App");
        };
    }
</script>

{#if ($currentlyDisplaying == "NewGeneratorModal")}
    <Modal label="New Generator">
        <p class="p-2 m-2 opacity-100">Type in a name for the new generator and click the Confirm button, or click the <span class="text-red">X</span> to cancel.</p>
        <input class="bg-bg2 p-2 m-2 text-center no-spinner text-yellow placeholder:opacity-50" type="text" placeholder="generator name" bind:value={generatorNameInput} />
        <Button fn={createAndClose} label={"Confirm"} />
    </Modal>
{/if}
