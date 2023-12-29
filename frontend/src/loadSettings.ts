import type { GeneratorSettings } from './types'; 
import { api_address } from '$lib/env.js';
import { parseCatsFromJSON } from './parser';
import { categories, patterns } from './store';

async function loadSettings(currentGenerator: string) {
    let data = await fetch(`${api_address}/generators/settings?generator=${currentGenerator}`, { credentials: "same-origin" });
    let json: GeneratorSettings = await data.json();
    
    categories.set(parseCatsFromJSON(json.categories));
    patterns.set(json.patterns);
}

export default loadSettings

