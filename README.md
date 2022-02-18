# Language Generator
<sub>*placeholder name*</sub>

A small project/application/library intended for generating [conlang](https://en.wikipedia.org/wiki/Constructed_language) - artificial language - resources, such as basic phonologies and word lists, however one may wish to use it.

This started primarily as a Python study project, but gradually extended to include studies in other topics and languages. As a note, I badly borked some work I had done both in Python and Rust on 10/02/2022, because of not adequately commiting stuff and then profoundly messing up git commands.  

I took this oportunity to redo most of what I had done, in a more thoughtful manner. This is still a super ambitious project, and I don't know when it's going to be usable, but it remains as it began - a study project.

| Feature                               | Status |
| :------------------------------------ | :----: |
| Core: Include IPA symbols             |        |
| Core: Parse X-SAMPA                   |        |
| Core: Show available languages        |        |
| Core: Save to JSON                    |        |
| Core: Load from JSON                  |        |
| Interface: Graphical                  |        |
| Interface: Command Line               |        | 
| Interface: Web                        |        |
| Syllables: All possible               |        |
| Syllables: Filtered                   |        |
| Syllables: Weighted                   |        |
| Syllables: Random set                 |        |
| Words: All possible                   |        | 
| Words: Filtered                       |        |
| Words: Weighted                       |        |
| Words: Random set                     |        |
| Words: Analyze (pattern, size)        |        |
| Pseudotext                            |        |
| Anagrams: All                         |        |
| Anagrams: Filtered                    |        |
| Phonology: Fully random generation    |        |
| Phonology: Filtered random generation |        |
| Phonology: Sample-based generation    |        |
| Phonology: Apply rules                |        |
| Phonology: Phonetic layer             |        |
| Phonology: Phonemic layer             |        |
| Morphosyntax: Noun declension         |        |
| Morphosyntax: Verb declension         |        |

## Notes
- Weights should default to equiprobable if none are provided
- Separate syllable and word (sandhi) rules
- Add weights to word sizes
