import type { CategoryType } from "./types";

function parseCatsFromJSON(data: any) { 
    // console.log("FROM JSON - INPUT\n", data);

    let symbols = Object.keys(data);
    let elements = Object.values(data);
    let cats: string[][] = symbols.map((symbol, index) => {
        return [symbol, String(elements[index]).replace(/,/g, " ")];
    });

    // console.log("FROM JSON - OUTPUT\n", cats);
    return cats;
}

function parseCatsToJSON(data: string[][]): CategoryType[] {
    // console.log("TO JSON - INPUT\n", data);

    // Error handling, refactor to a better form
    data.forEach((e) => {
        if(e.length != 2 || e[1].length < 1) {
            console.log(`Warning: attempting to parse invalid category (${e}), aborting...`);
            return;
        }
    });

    let cat: Record<string, string[]> = data.reduce((cat: Record<string, string[]>, [symbol, elements]) => {
        cat[symbol] = String(elements).split(" ");

        return cat;
    }, {});

    // console.log("TO JSON - OUTPUT\n", cat);
    return cat;
}

export { parseCatsToJSON, parseCatsFromJSON }

