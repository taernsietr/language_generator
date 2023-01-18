<script>
    import "../app.css";
    import Settings from '../components/Settings.svelte';
    import Results from '../components/Results.svelte';
    import Information from '../components/Information.svelte';

    let minSyllables = 1;
    let maxSyllables = 5;
    let textLength = 25;
    let results;
</script>

<h1>Language Generator</h1>

<div id="generatorSettings" class="">
    <label for="minSyllables">Min syllables:</label>
    <input type="number" bind:value={minSyllables} min="1" max="255" id="minSyllables" name="minSyllables">
    <label for="maxSyllables">Max syllables:</label>
    <input type="number" bind:value={maxSyllables} min="1" max="255" id="maxSyllables" name="maxSyllables">
    <label for="textLength">Text length:</label>
    <input type="number" bind:value={textLength} min="1" max="128" id="textLength" name="textLength">
</div>

<button type="submit" onclick="getRandom('word')">Random Word</button>
<button type="submit" onclick="getRandom('text')">Random Text</button>
<button type="submit" onclick="toggleInfo()">Toggle Info</button>
<button type="submit" onclick="toggleSettings()">Toggle Settings</button>
<button type="submit" onclick="saveSettings()">Save Settings</button>
<button type="submit" onclick="loadSettings()">Load Settings</button>

<Results />
<Settings />
<Information />

