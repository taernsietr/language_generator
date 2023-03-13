import { currentlyDisplaying } from "./store";

export function parseCatsFromJSONData(data: any) { 
    let k = Object.keys(data);
    let v = Object.values(data);

    let cats = k.map((e, i) => {
        return [e, v[i]];
    });

    return cats;
}

export function parseCatsToJSON(data: any) {
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

