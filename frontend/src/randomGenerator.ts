import loadSettings from "./loadSettings";
import loadGenerators from "./loadGenerators";
import { api_address } from "$lib/env";
import { currentGenerator } from './store';

// TODO: properly implement this
async function newRandomGenerator() {
    fetch(`${api_address}/random_generator`); 
    await loadGenerators();
    currentGenerator.set("alpha");
    await loadSettings("alpha");
}

export default newRandomGenerator;

