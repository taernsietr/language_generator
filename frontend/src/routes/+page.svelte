<script>
    import "../app.css";
    import Settings from '../components/Settings.svelte';
    import Results from '../components/Results.svelte';
    import Information from '../components/Information.svelte';
    import Menu from '../components/Menu.svelte';
    import Generator from '../components/Generator.svelte';
    import NewGeneratorModal from "../components/NewGeneratorModal.svelte";
</script>

<div class="bg-bg1 flex m-4 p-4 place-content-center shadow-xl">
    <h1 class="text-xl text-orange">Language Generator</h1>
</div>

<div class="grid grid-flow-row grid-cols-9">
    <Menu />
    <Results />
    <Generator />
    <Settings />
    <Information />
</div>

