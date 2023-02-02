import { writable, derived } from 'svelte/store';

// Application display language
export const language = writable("enUS");

// Card toggles
export const displaySettings = writable(true);
export const displayInfo = writable(true);

// Generation settings
export const minSyllables = writable(1);
export const maxSyllables = writable(5);
export const textLength = writable(25);

// Generators and generator data
export const generators = writable(["loading generators..."]);
export const currentGenerator = writable();
// export const currentGenerator = writable("default-settings");
export const categories = writable({});
export const patterns = writable([]);
/* export const currentSettings = writable({
    name: "",
    categories: {},
    patterns: []
});
*/

export const results = writable("Please select one of the options above!");
export const unsavedChanges = writable(false);

