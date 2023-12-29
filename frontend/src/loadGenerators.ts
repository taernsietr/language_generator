import { generators, currentGenerator } from './store';
import { api_address } from '$lib/env.js';

async function loadGenerators() {
    let data = await fetch(`${api_address}/generators`, { credentials: "same-origin" });
    let json: string[] = await data.json();

    generators.set(json);
    currentGenerator.set(json[0]);
}

export default loadGenerators
