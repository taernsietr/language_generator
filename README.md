# language_generator

(*language generator* is a placeholder name)

a small library/application intended for generating [conlang](https://en.wikipedia.org/wiki/Constructed_language) - artificial language - resources, such as basic phonologies and word lists.

started primarily as a Python study project, but gradually extended to include studies in other topics.

## milestones

- [x] basic library functionality (Python)
- [ ] phonology generation
- [ ] standalone app (currently: Python, tkinter)
- [ ] web version (currently: React.js)

## bugs
- [ ] fix apply_rules
- [ ] read_rules regex is not parsing correctly
- [ ] currently, the tkinter GUI is broken

## pending improvements
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

### functions
* single-word functions
  * anagrams [all, n-, rule-based]
    * analyze [size, letters, apparent pattern]
    * apply paradigm
  * extractor functions
    * inventory
    * pattern
    * rules
* other
  * pseudotranslation wordlists (near 1:1 assignment, for visualization only)