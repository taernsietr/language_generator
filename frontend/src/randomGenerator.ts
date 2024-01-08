import loadGenerators from "./loadGenerators";
import { api_address } from "$lib/env";
import { currentGenerator } from './store';
import loadSettings from "./loadSettings";

// TODO: properly implement this
async function newRandomGenerator() {
    let data = await fetch(`${api_address}/generators/random_generator`, { credentials: "same-origin" }); 
    let randomGeneratorName: string = await data.json();

    console.log("DEBUG:", randomGeneratorName);

    await loadGenerators();
    currentGenerator.set(randomGeneratorName);
    loadSettings(randomGeneratorName);
}

export default newRandomGenerator;

