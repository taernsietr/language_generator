<script>
    import { displayInfo, displaySettings, language } from '../store.js';

    function toggleInfo() { 
        displayInfo.update((current) => !current); 
    }

    function toggleSettings() {
        displaySettings.update((current) => !current); 
    }
    
    function toggleLanguage() {
        language.update((current) => current == "enUS" ? "ptBR" : "enUS"); 
    }
</script>

<div class="bg-bg1 flex flex-1 flex-col justify-around m-2 p-2 place-content-center shadow-xl text-fg">
    <h2 class="basis-1 text-center text-green">Menu</h2>
    <button class="bg-bg2 flex m-2 p-2 place-content-center text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={toggleInfo}>Toggle Info</button>
    <button class="bg-bg2 flex m-2 p-2 place-content-center text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={toggleSettings}>Toggle Settings</button>
    <button class="bg-bg2 flex m-2 p-2 place-content-center text-fg hover:bg-bg3 hover:fg-fg0 transition duration-400" type="submit" on:click={toggleLanguage}>{$language == "enUS"? "English" : "Português"}</button>
</div>
