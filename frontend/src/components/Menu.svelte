<script lang="ts">
    import { currentlyDisplaying, displaySettings, language } from '../store.js';
    import Information from './Information.svelte';
    import Button from './Button.svelte';

    function toggleSettings() {
        displaySettings.update((current) => !current); 
    }
    
    function toggleLanguage() {
        language.update((current) => current == "enUS" ? "ptBR" : "enUS"); 
    }
</script>

<Information />
<div class="bg-bg1 basis-full flex flex-col justify-around m-2 p-2 place-content-center shadow-xl text-fg">
    <h2 class="basis-1 text-center text-green">Menu</h2>
    <Button fn={() => { currentlyDisplaying.set("Information")} } label={"Information"} />
    <Button fn={toggleSettings} label={"Toggle Settings"} />
    <Button fn={toggleLanguage} label={$language == "enUS"? "English" : "Português"} />
</div>
