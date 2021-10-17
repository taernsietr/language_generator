# language_generator

A small library/application intended for generating [conlang](https://en.wikipedia.org/wiki/Constructed_language) resources, such as basic phonologies, word lists and pseudotext.

Started primarily as a Python study project, but gradually extended to include studies in other topics.

Currently planning a refactor into another language (probably some flavor of C or Rust, but still as a study project.

## milestones

- [x] basic library functionality (Python)
  - [ ] remake into another language
- [ ] phonology generation
- [ ] standalone app (currently: Python, tkinter; planning: C or Rust, Qt or GTK)
- [ ] web version (planning: React.js and/or php)

## pending
- [ ] redo error checking stuff where it's more relevant
- [ ] improve weight handling (visually and functionally)
- [ ] remove code using global variables (currently using with words_all_fixedlength and syl_all_frompattern)
- [ ] default weights to equiprobable if none are given
- [ ] separate syllable and word (sandhi) rules
- [ ] add weights to word sizes
- [ ] ruleSets (as an argument passed into apply_rules); extend into class in order to add conditions?
- [ ] another way to generate words where a list of category-based word patterns are generated
- [ ] rework program to work based on multiple layers? (phonetic, phonemic, syntactic etc)
- [ ] pretty formatted saved program settings
- [ ] change saved program settings from xml to json 
- [ ] single-word functions
  - [ ] anagrams [all, n-, rule-based]
  - [ ] analyze [size, letters, apparent pattern]
  - [ ] apply paradigm
- [ ] extractor functions
  - [ ] inventory
  - [ ] pattern
  - [ ] rules
- other
  - [ ] pseudotranslation wordlists (near 1:1 assignment, for visualization only)
