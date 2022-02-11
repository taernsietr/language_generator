import sys, functions, io_functions
import tkinter as tk
import tkinter.ttk as ttk
from stda_functions import *
from stda_windows import *
from config import *
try:
	root = Tk()
	root.title('Language Generator')
	print('Main window created successfully.')
except:
	sys.exit('Could not create main window.')

# load default starting configuration from config.xml and starting settings from default.xml
config = io_functions.load_config()
settings = io_functions.load_settings()

syllable_count = IntVar().set(int(config.get('syllables')))
word_count = IntVar().set(config.get('words'))
word_min_s = IntVar().set(config.get('min_wordsize'))
word_max_s = IntVar().set(config.get('max_wordsize'))
pseudotext_size = IntVar().set(config.get('pseudotext_size'))

## WIDGETS

menubar = Menu(root, tearoff = 0)
menubar.add_command(label = 'Load', command = summon_win_load)
menubar.add_command(label = 'Save', command = summon_win_save)
menubar.add_command(label = 'Setup', command = summon_win_setup)

text_results = Text(root, width = 60, height = 18, wrap = WORD)

label_syllable_count = Label(root, text = 'Syllables:')
label_word_count = Label(root, text = 'Words:')
label_word_min_size = Label(root, text = 'Min Word Size:')
label_word_max_size = Label(root, text = 'Max Word Size:')
label_pseudotext_size = Label(root, text = 'Pseudotext Size:')

scale_syllable_count = Scale(root, from_ = 1, to = MAX_SYLLABLE_COUNT, length = 50, orient = HORIZONTAL, variable = syllable_count)
scale_word_count = Scale(root, from_ = 1, to = MAX_WORD_COUNT, length = 50, orient = HORIZONTAL, variable = word_count)
scale_word_min_size = Scale(root, from_ = 1, to = 4, length = 50, orient = HORIZONTAL, variable = word_min_s)
scale_word_max_size = Scale(root, from_ = 1, to = 6, length = 50, orient = HORIZONTAL, variable = word_max_s)
scale_pseudotext_size = Scale(root, from_ = 1, to = 128, length = 50, orient = HORIZONTAL, variable = pseudotext_size)

button_generate_syllables = Button(root, width = 12, text = 'Syllables', command = generate_n_syllables)
button_generate_words = Button(root, width = 12, text = 'Words', command = generate_words)
button_generate_pseudotext = Button(root, width = 12, text = 'Text', command = generate_pseudotext)
button_clear = Button(root, width = 12, text = 'Clear', command = lambda: text_results.delete(1.0, END))

## PLACE WIDGETS

text_results.grid(row = 0, column = 4, rowspan = 7, padx = 5, pady = 5, sticky = N)

label_syllable_count.grid(row = 0, column = 1, padx = 5, pady = 5, sticky = W)
label_word_count.grid(row = 1, column = 1, padx = 5, pady = 5, sticky = W)
label_word_min_size.grid(row = 2, column = 1, padx = 5, pady = 5, sticky = W)
label_word_max_size.grid(row = 3, column = 1, padx = 5, pady = 5, sticky = W)
label_pseudotext_size.grid(row = 4, column = 1, padx = 5, pady = 5, sticky = W)

scale_syllable_count.grid(row = 0, column = 2, padx = 5, pady = 5, sticky = W)
scale_word_count.grid(row = 1, column = 2, padx = 5, pady = 5, sticky = W)
scale_word_min_size.grid(row = 2, column = 2, padx = 5, pady = 5, sticky = W)
scale_word_max_size.grid(row = 3, column = 2, padx = 5, pady = 5, sticky = W)
scale_pseudotext_size.grid(row = 4, column = 2, padx = 5, pady = 5, sticky = W)

button_generate_syllables.grid(row = 0, column = 0, padx = 5, pady = 5, sticky = W)
button_generate_words.grid(row = 1, column = 0, padx = 5, pady = 5, sticky = W)
button_generate_pseudotext.grid(row = 2, column = 0, padx = 5, pady = 5, sticky = W)
button_clear.grid(row = 3, column = 0, padx = 5, pady = 5, sticky = W)

root.config(menu = menubar)
root.mainloop()
