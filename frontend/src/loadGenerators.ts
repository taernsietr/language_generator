import type { GeneratorNames } from './types';
import { generators, currentGenerator } from './store';
import { api_address } from '$lib/env.js';

async function loadGenerators() {
    let data = await fetch(api_address, { credentials: "same-origin" });
    let json: GeneratorNames = await data.json();
    
    generators.set(json.generators);
    currentGenerator.set(json.generators[0]);
}

export default loadGenerators
