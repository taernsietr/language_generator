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
    <div class="flex flex-col p-auto">
        <div class="bg-bg1 flex m-2 p-2 place-content-center shadow-xl">
            <h1 class="text-xl text-orange">Random Text Generator</h1>
        </div>

        <div class="flex md:flex-row sm:flex-col flex-wrap">
            <Menu />
            <Generator />
            <Results />
            <IPAPanel />
            <Settings />
        </div>
    </div>
{/if}

