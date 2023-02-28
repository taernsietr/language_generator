<script lang="ts">
    import "../app.css";
    import Settings from '../components/Settings.svelte';
    import Results from '../components/Results.svelte';
    import Menu from '../components/Menu.svelte';
    import Generator from '../components/Generator.svelte';
// <div class="flex flex-row flex-wrap">
</script>

<div class="bg-bg1 flex m-2 p-2 place-content-center shadow-xl">
    <h1 class="text-xl text-orange">Random Text Generator</h1>
</div>

<div class="grid grid-flow-row grid-cols-9">
    <Menu />
    <Generator />
    <Results />
    <Settings />
</div>

