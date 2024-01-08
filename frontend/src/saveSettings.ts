import type { PatternType, GeneratorSettings, RuleType } from './types';
import { parseCatsToJSON } from './parser';
import { api_address } from '$lib/env';
import { categories, patterns, ruleset, unsavedChanges } from './store';

async function saveSettings(
    unsaved: boolean,
    currentGenerator: string,
    pats: PatternType[],
    cats: string[][],
    rules: RuleType[]
) {
    // console.log("unsaved:", unsaved);

    if(unsaved) {
        categories.set(cats);
        patterns.set(pats);
        ruleset.set(rules);
        unsavedChanges.set(false);
        
        let settings: GeneratorSettings = {
            name: currentGenerator,
            categories: parseCatsToJSON(cats),
            patterns: pats,
            ruleset: [{context: "", result: ""}]
        };

        fetch(`${api_address}/generators/save`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(settings),
        })
            .catch((error) => {
                console.error("Error:", error);
                unsavedChanges.set(true);
            });
        console.log("Saving changes...");
    }
    else {
        console.log("No unsaved changes, doing nothing.");
    }
}

export default saveSettings

