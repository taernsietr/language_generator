<script lang="ts">
    import { onMount } from 'svelte';
    import { currentGenerator } from '../store';

    import "../app.css";
    import loadGenerators from '../loadGenerators'; 
    import loadSettings from '../loadSettings'; 

    import Settings from '../components/Settings.svelte';
    import Results from '../components/Results.svelte';
    import Menu from '../components/Menu.svelte';
    import Generator from '../components/Generator.svelte';
    import IPAPanel from '../components/IPAPanel.svelte';
    
    let loaded = false;
    
    onMount(async () => {
        await loadGenerators();
        await loadSettings($currentGenerator);
        loaded = true;
});
</script>

{#if loaded}
    <div class="flex flex-col p-auto page-container">
        <div class="bg-bg1 flex m-2 p-2 place-content-center shadow-xl">
            <h1 class="text-xl text-orange">Random Text Generator</h1>
        </div>

        <div class="section-container flex flex-col">
            <Menu />
            <div class="section-container flex flex-row flex-wrap">
                <Generator />
                <Results />
            </div>
            <!-- <IPAPanel /> -->
            <Settings />
        </div>
    </div>
{/if}

