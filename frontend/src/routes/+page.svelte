<script lang="ts">
    import "../app.css";
    import Settings from '../components/Settings.svelte';
    import Results from '../components/Results.svelte';
    import Information from '../components/Information.svelte';
    import Menu from '../components/Menu.svelte';
    import Generator from '../components/Generator.svelte';

    let showSettings = true;
    let showInfo = true;
    let min = 1;
    let max = 5;
    let length = 25;
    let results;
</script>

<div class="bg-bg1 flex m-4 p-4 place-content-center shadow-xl">
    <h1 class="text-lg text-orange">Language Generator</h1>
</div>
<div class="grid grid-flow-row grid-cols-9">
    <Menu />
    <Results />
    <Generator minSyllables={min} maxSyllables={max} textLength={length} />
    <Settings display={showSettings} />
    <Information display={showInfo} />
</div>

