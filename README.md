	# language_generator
a suite of functions / GUI application for generating conlang resources. primarily a Python study project

- bugs
fix apply_rules
read_rules regex is not parsing correctly

- improvements
redo error checking stuff where it's more relevant
improve weight handling (visually and functionally)
try to avoid using global variables? (currently using with words_all_fixedlength and syl_all_frompattern)
add default (None) value to parse_cats and parse_pats, and make all weights 1 if wgt == None

- features
GUI
	random weights (button)
	random inventories (button)
	random patterns (button)

functions
	generator 
		generator unique word 
		generate random syllabic patterns
		generate random phonetic inventory
		generate pseudotext
	single-word functions
		anagrams [all, n-, rule-based]
		analyze [size, letters, apparent pattern]
		apply paradigm
	extractor functions
		inventory
		pattern
		rules

separate syllable and word (sandhi) rules
ruleSets (as an argument passed into apply_rules); extend into class in order to add conditions?
pseudotranslation wordlists
other output display methods
another way to generate words where a list of category-based word patterns are generated
add weights to word sizes
