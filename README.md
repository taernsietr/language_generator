# Language Generator
<sub>*placeholder name*</sub>

A small project/application/library intended for generating 
[conlang](https://en.wikipedia.org/wiki/Constructed_language) (_artificial
 language_) resources, such as basic phonologies and word lists, however one
 may wish to use it.  

This started primarily as a Python study project, but gradually extended to
include studies in other topics and languages. I then badly borked some
work I had done both in Python and Rust on February 2022, because of not
adequately commiting stuff and then profoundly messing up git commands (at
least, I learned a bit :grimace:).  

As a result, I restarted from scratch in Rust and vanilla JS, and later a very
glued together frontend in Svelte. This has been an enormously important
project for me to learn dev practices, Rust, JS, Python, git and overall
building slightly more complex applications.  

This is still a super ambitious project, and I don't know when it's going to
be usable, but it remains as it began - a study project. However, should
someone wish to use this as-is, I _will_ at some point make releases - after
all, that is also something I need to learn how to do properly.  

*Currently, all this is is a barebones Rust API written with Actix and a simple
(and fragile) frontend in Svelte.*

## Features  

There's no well-defined set of features and capabilities I want to add to this
project, just some overall ideas such as generating (broadly) phoneme
inventories, phonologies and pseudotext. Some specific use cases are listed
below, but they can be changed, removed or more might be added to the list.  

### Core Stuff
- Include IPA symbol picker 
- Parse X-SAMPA to IPA  
- Parse IPA to X-SAMPA   
- Save to JSON (_SimpleGenerators_)  
- Load from JSON (_SimpleGenerators_)  

### Simpler text stuff
- All anagrams
- Filtered anagrams
- Words: Random set  
- Words: All possible  
- Words: Weighted  
- Words: Filtered  
- Words: Static affixes (_e.g. baCV_)  
- Words: Analyze a given word
- Text: Word size weighting
- Text: Pseudotext (_text that is not just a bunch of fully random words
  without punctuation, but could be mistaken for actual text_)  

### Harder text stuff
- Syllable-level rules
- Word-level rules
- Sandhi rules

#### Phonology
- Fully random generation  
- Filtered random generation
- Sample-based generation  
- Phonetic layer  
- Phonemic layer  

#### Morphosyntax
- Noun declension  
- Verb declension  

## To-do and other notes
- Weights should default to equiprobable if none are provided
- Consider case where the user inputs a generator name with spaces
