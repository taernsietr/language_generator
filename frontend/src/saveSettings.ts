import type { PatternType, GeneratorSettings } from './types';
import { parseCatsToJSON } from './parser';
import { api_address } from '$lib/env';
import { categories, queuedCategories, patterns, queuedPatterns, unsavedChanges } from './store';

async function saveSettings(unsaved: boolean, currentGenerator: string, pats: PatternType[], cats: string[][]) {
    
    console.log("unsaved:", unsaved);

    if(unsaved) {
        categories.set(cats);
        patterns.set(pats);
        unsavedChanges.set(false);
        
        console.log("PATS", pats);
        
        let settings: GeneratorSettings = {
            name: currentGenerator,
            categories: parseCatsToJSON(cats),
            patterns: pats
        };

        fetch(`${api_address}/save`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(settings),
        })
            .catch((error) => { console.error("Error:", error); });

        console.log("Saving changes...");
    }
    else {
        console.log("No unsaved changes, doing nothing.");
    }
}

export default saveSettings

