<script lang="ts">
    import "../app.css";
    import Settings from '../components/Settings.svelte';
    import Results from '../components/Results.svelte';
    import Menu from '../components/Menu.svelte';
    import Generator from '../components/Generator.svelte';
</script>

<div class="flex flex-col p-auto">
    <div class="bg-bg1 flex m-2 p-2 place-content-center shadow-xl">
        <h1 class="text-xl text-orange">Random Text Generator</h1>
    </div>

    <div class="flex md:flex-row sm:flex-col flex-wrap">
        <Menu />
        <Generator />
        <Results />
        <Settings />
    </div>
</div>

