<script lang="ts">  
    import { currentSettings } from '../store.js';

    let patterns: any;

    currentSettings.subscribe(settings => {
        let temp = String(settings.patterns).replace(/,/g, " ");
        patterns = temp;
    });

    $currentSettings.patterns
</script>

<div class="flex-row m-4 p-4 place-content-center">
    <h3 class="text-center text-blue">Patterns</h3>
    <input class="bg-bg2 text-center ml-2 no-spinner p-2 text-fg" type='text' bind:value={patterns} />
</div>

