<script lang="ts">
    import { displaySettings, language } from '../store.js';
    import Information from './Information.svelte';
    import Button from './Button.svelte';

    function toggleSettings() {
        displaySettings.update((current) => !current); 
    }
    
    function toggleLanguage() {
        language.update((current) => current == "enUS" ? "ptBR" : "enUS"); 
    }

    let modal: boolean = false;
    async function informationModal() {
        modal = !modal;
    }
</script>

<div class="bg-bg1 flex flex-1 flex-col justify-around m-2 p-2 place-content-center shadow-xl text-fg">
    <h2 class="basis-1 text-center text-green">Menu</h2>
    <Button fn={informationModal} label={"Information"} />
    <Button fn={toggleSettings} label={"Toggle Settings"} />
    <Button fn={toggleLanguage} label={$language == "enUS"? "English" : "Português"} />
</div>
{#if modal}
    <Information bind:state={modal}/>
{/if}
