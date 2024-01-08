import { writable } from 'svelte/store';

// Application display language
export const language = writable("enUS");

// Component toggles
export const displaySettings = writable(true);
export const currentlyDisplaying = writable("App");

// Generation settings
export const minSyllables = writable(1);
export const maxSyllables = writable(3);
export const syllableBias = writable(0.0);
export const textLength = writable(20);

// Generators and generator data
export const generators = writable(["loading generators..."]);
export const currentGenerator = writable();
export const categories = writable();
export const patterns = writable();
export const ruleset = writable();
export const queuedCategories = writable();
export const queuedPatterns = writable();

export const results = writable("Generated text will appear here!");
export const unsavedChanges = writable(false);

