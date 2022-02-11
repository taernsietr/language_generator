from tkinter import *
from tkinter.ttk import *

def generate_n_syllables(display, settings):
	# generates syllables and prints them to text_results

	display.insert(INSERT, ' '.join(functions.syl_n(settings.categories, settings.indices, settings.patterns, settings.weights, syllable_count.get())))
	
def generate_all_syllables(display, settings):

	for pattern in settings.patterns:
		functions.syl_all(settings.categories, settings.indices, pattern)
		
	display.insert(INSERT, ' '.join(functions.all_syllables))

def generate_words(display, min_s, max_s, count):
	# generates all possible words and prints them to display
	# if entry_word_count is zero, use generate_all_words, otherwise use generate_n_words
	# also generates new syllables
	# currently also applies any rule in {rules}

	syllables = display.get(1.0, END).split(' ')
	
	# output_words = apply_rules(output_words)
	display.insert(INSERT, ' '.join(functions.words_n(syllables, min_s, max_s, word_count.get())))

def generate_pseudotext(display):

	words = display.get(1.0, END).split(' ')
	display.insert(INSERT, ' '.join(functions.pseudotext(words, pseudotext_size.get())))