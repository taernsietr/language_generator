# Language Generator

A small project/application/library intended for generating
[conlang](https://en.wikipedia.org/wiki/Constructed_language) (_artificial
language_) resources, such as basic phonologies and word lists, however one may
wish to use it.  

This started primarily as a Python study project, but gradually extended to
include studies in other topics and languages. I then badly borked some work I
had done both in Python and Rust on February 2022, because of not adequately
commiting stuff and then profoundly messing up git commands (at least, I learned
a bit).  

As a result, I restarted from scratch in Rust and vanilla JS, and later a very
glued together frontend in Svelte. This has been an enormously important project
for me to learn dev practices, Rust, JS, Python, git and overall building
slightly more complex applications.  This is still a super ambitious project,
and I don't know when it's going to be usable, but it remains as it began - a
study project. 

There's no predefined set of features and capabilities I want to add to this
project, just some overall ideas such as generating (broadly) phoneme
inventories, phonologies and pseudotext. Some stuff I might work on at some
point are listed below.

- IPA symbol picker, for easier input
- Parse X-SAMPA to IPA and vice-versa
- Analyze word patterns for setup extraction
- Generate all anagrams for a given word
- Generate all possible words for a given setup
- Generate filtered anagrams for a given word
- Generate random words with weighted/filtered probabilities
- Generate words with static affixes (_e.g. baCV_)  
- Generate text with weighted word probabilities
- Generate pseudotext (_text that is not just a bunch of fully random words
  without punctuation, but could be mistaken for actual text_)  
- Support syllable and word-level rules
- Support sandhi rules

Later on, if things go right, I might try adding real language concepts to the
generator, supporting phological, morphosyntactical and other kinds of
representation layers. Words would be generated based on real-world phonetic and
phonemic rules per setup, and we could support variations such as noun and verb
declension for each generated word.

