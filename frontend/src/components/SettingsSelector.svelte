<script lang="ts">
    import { loadJSON } from '../helpers.js';
    import { generators, currentGenerator, categories, patterns } from '../store.js';

    function saveSettings() { 
        let settings = {
            generator: $currentGenerator,
            categories: $categories,
            patterns: $patterns 
        };

        fetch("http://127.0.0.1:8080/api/update", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify(settings),
        })
            .then((response) => response.json())
            .then((data) => {
                console.log("Success:", data);
            })
            .catch((error) => {
                console.error("Error:", error);
            });
    }

    function clearSettings() { console.log("clearSettings") }

    async function loadSettings() { 
        let data = await loadJSON(`settings?generator=${$currentGenerator}`);
        categories.set(data.categories);
        patterns.set(data.patterns);

        // console.log("loadSettings");
    }
</script>

<div class="flex-col m-4 p-4 place-content-center">
    <h3 class="text-center text-blue">Available Presets</h3>
    <select class="bg-bg2 text-center ml-2 no-spinner p-2 text-fg" bind:value={$currentGenerator}>
        {#each $generators as option}
            <option value={option}>{option}</option>
        {/each}
    </select>
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={saveSettings}>Save Settings</button>
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={loadSettings}>Load Settings</button>
    <button class="bg-bg2 basis-1/4 m-4 p-4 text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={clearSettings}>Clear Settings</button>
</div>
