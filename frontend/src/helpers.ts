import { currentGenerator, generators, categories, patterns } from "./store";
import { api_address } from '$lib/env.js';

interface Category {
    symbol: string;
    elements: string[]
}

interface Pattern {
    symbol: string;
    position: string;
    weight: string;
}

interface GeneratorSettings {
    name: string;
    categories: Category[];
    patterns: Pattern[];
}

interface GeneratorNames {
    generators: string[];
}

function parseCatsFromJSONData(data: any) { 
    let k = Object.keys(data);
    let v = Object.values(data);

    let cats = k.map((e, i) => {
        return [e, v[i]];
    });

    return cats;
}

function parseCatsToJSON(data: any) {
    // TODO: remover isso, a.k.a. a pior gambiarra da minha vida
    // for future reference, this will get the destructured categories which are passed around on the frontend
    // and through string manipulation reforms it into the JSON format the backend understands
    // ... just to turn it into an object again so we can rebuild the actual JSON body that's expected
    let cats = JSON.stringify(data);
    cats = cats.replace(/^./, "{");
    cats = cats.replace(/.$/, "}");
    cats = cats.replace(/\["([A-Z0-9])",/g, "\"$1\":");
    cats = cats.replace(/\]\]/g, "]");
    return JSON.parse(cats);
}

async function loadGenerators() {
    // get generators loaded on the backend and set the first active 
    let data = await fetch(api_address, { credentials: "same-origin" });
    let json: GeneratorNames = await data.json();
    generators.set(json.generators);
    currentGenerator.set(json.generators[0]);
}

async function loadSettings(currentGenerator: string) {
    // load settings for the active generator
    let data = await fetch(`${api_address}/settings?generator=${currentGenerator}`, { credentials: "same-origin" });
    let json: GeneratorSettings = await data.json();
    categories.set(parseCatsFromJSONData(json.categories));
    // categories.set(data.categories);
    patterns.set(json.patterns);
}

async function saveSettings(name: string, categories: Category[], patterns: Pattern[]) { 
    let parsedCategories = parseCatsToJSON(categories);
    let settings = {
        name,
    //    categories: $categories,
        categories: parsedCategories,
        patterns
    };
    
    fetch(`${api_address}/save`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(settings),
    })
        .catch((error) => { console.error("Error:", error); });
}

export { loadGenerators, loadSettings, saveSettings }
