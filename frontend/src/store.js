import { writable } from 'svelte/store';

export const language = writable("enUS");
export const displaySettings = writable(true);
export const displayInfo = writable(true);
export const minSyllables = writable(1);
export const maxSyllables = writable(5);
export const textLength = writable(25);
export const results = writable("Please select one of the options above!");
