## MAIN

from tkinter import *
from tkinter.ttk import *

import os
import xml.etree.ElementTree as ET
import tlib as tlib

MAX_PATTERN_SIZE = 7
MAX_WORD_SIZE = 5
MAX_SYLLABLE_COUNT = 100
MAX_WORD_COUNT = 100

INSTALL_FOLDER = 'D:\Run\Python\Language Generator\\'
DEFAULT_SETTINGS = INSTALL_FOLDER + 'default.xml'
DEFAULT_CONFIG = INSTALL_FOLDER + 'config.xml'

root = Tk()
root.title = 'Language Generator'

class Settings:
	
	def __init__(self, categories, indices, patterns, weights, rules):
		
		self.categories = categories
		self.indices = indices
		self.patterns = patterns
		self.weights = weights
		self.rules = rules
	
def load_config(config = DEFAULT_CONFIG):
# returns a dictionary containing program configuration
	
	current = {}
	
	xml = ET.parse(config)
	root = xml.getroot()
	
	for item in root.iterfind('*'):
		current[item.tag] = item.text
	
	return current

def load_settings(address = DEFAULT_SETTINGS):
# returns categories, cat weights, patterns and pat weights, respectively, as lists
	
	cats = []
	cwgt = []
	pats = []
	pwgt = []
	rls = []

	try:
		xml = ET.parse(address)
		print(f'{address} loaded successfully.')
	except:
		print('Error loading file.')
		
	root = xml.getroot()

	xcats = root.findall('.//category')
	for item in xcats:
		cats.append(f'{item.find("index").text} = {item.find("members").text} ')
		cwgt.append(f'{item.find("index").text} = {item.find("weights").text} ')
		
	xpats = root.findall('.//pattern')
	for item in xpats:
		pats.append(f'{item.find("format").text} ')
		pwgt.append(f'{item.find("weight").text} ')
	
	xrules = root.findall('.//rule')
	for item in xrules:
		rls.append(f'{item.find("initial")} > {item.find("final")} / {item.find("prev")}_{item.find("next")} ')

	categories, indices = tlib.parse_categories(''.join(cats), ''.join(cwgt))
	patterns, weights = tlib.parse_patterns(''.join(pats), ''.join(pwgt))
	rules = tlib.parse_rules(''.join(rls))

	settings = Settings(categories, indices, patterns, weights, rules)
	
	return settings

def save_settings(name, settings):
# saves program settings to {name}.xml
# currently does not check if the given file already exists
	
	newSettings = ET.Element('settings')
	new_xml = ET.ElementTree(newSettings)
	
	cats = ET.Element('categories')
	newSettings.append(cats)
	for cat in range(len(settings.categories)):
		this_cat = ET.Element('category', {'id': str(cat)})
		this_cat_index = ET.Element('index')
		this_cat_index.text = settings.categories[cat].index
		this_cat_members = ET.Element('members')
		this_cat_members.text = ' '.join(settings.categories[cat].members)
		this_cat_weights = ET.Element('weights')
		this_cat_weights.text = ' '.join(list(map(str, settings.categories[cat].weights)))
		
		this_cat.append(this_cat_index)
		this_cat.append(this_cat_members)
		this_cat.append(this_cat_weights)
		cats.append(this_cat)
	
	pats = ET.Element('patterns')
	newSettings.append(pats)
	for pat in range(len(settings.patterns)):
		this_pat = ET.Element('pattern', {'id': str(pat)})
		this_pat_format = ET.Element('format')
		this_pat_format.text = settings.patterns[pat].pattern
		this_pat_weight = ET.Element('weight')
		this_pat_weight.text = str(settings.patterns[pat].weight)
		
		this_pat.append(this_pat_format)
		this_pat.append(this_pat_weight)
		pats.append(this_pat)
	
	rls = ET.Element('rules')
	newSettings.append(rls)
	for rl in range(len(settings.rules)):
		this_rl = ET.Element('rule', {'id': str(rl)})
		this_rl_target = ET.Element('target')
		this_rl_target.text = settings.rules[rl].target
		this_rl_result = ET.Element('result')
		this_rl_result.text = settings.rules[rl].result
		this_rl_preceded = ET.Element('preceded')
		this_rl_preceded.text = settings.rules[rl].preceded
		this_rl_succeeded = ET.Element('succeeded')
		this_rl_succeeded.text = settings.rules[rl].succeeded
	
	filename = f'{INSTALL_FOLDER}{name}.xml'
	
	try:
		new_xml.write(filename, encoding = 'utf-8', xml_declaration=True)
		print(f'{filename} saved successfully.')
	except:
		print('Error saving file.')

def settings_files():
# returns a list of xml files in the program folder

	with os.scandir(INSTALL_FOLDER) as Dir:
	
		valid_settings = []
	
		for file in Dir:
			if file.name.endswith('.xml') and file.name != 'config.xml':
				valid_settings.append(file.name)
	
	return valid_settings

def print_results(output):
# print results to text_results

	text_results.insert(INSERT, ' '.join(output))
	
def clear_output():
# clears text_results

	text_results.delete(1.0, END)

def generate_n_syllables():
# generates syllables and prints them to text_results

	global settings

	syllables = tlib.syl_n(settings.categories, settings.indices, settings.patterns, settings.weights, syllable_count.get())
	
	print_results(syllables)
	
def generate_all_syllables():

	global settings

	for pattern in settings.patterns:
		tlib.syl_all(settings.categories, settings.indices, pattern)
		
	print_results(tlib.all_syllables)

def generate_words():
# generates all possible words and prints them to text_results
# if entry_word_count is zero, use generate_all_words, otherwise use generate_n_words
# also generates new syllables
# currently also applies any rule in {rules}

	syllables = text_results.get(1.0, END)
	syllable = syllables.split(' ')
	
	output = tlib.words_n(syllables, word_min_s.get(), word_max_s.get(), word_count.get())
	
	# output_words = apply_rules(output_words)
	print_results(output)

## SECONDARY WINDOWS

def summon_win_load():
# opens the prompt to load settings from a given xml file
	
	files = settings_files()
	
	window_loadprompt = Toplevel(root)
	window_loadprompt.title('Load Settings')
	
	combo_settingsname = Combobox(window_loadprompt, width = 40, height = 1, values = files)
	button_load = Button(window_loadprompt, text = 'Load', command = lambda: (load_settings(INSTALL_FOLDER + combo_settingsname.get()), window_loadprompt.destroy()))
	
	combo_settingsname.grid(row = 0, column = 0, padx = 2, pady = 2)
	button_load.grid(row = 0, column = 1, padx = 2, pady = 2)

def summon_win_save():
# opens the prompt to save settings to a new xml file
	
	savefile = StringVar()
	
	window_saveprompt = Toplevel(root)
	window_saveprompt.title('Save Settings')
	
	entry_savefile_name = Entry(window_saveprompt, width = 40, textvariable = savefile)
	
	button_save = Button(window_saveprompt, text = 'Save', command = lambda: (save_settings(savefile.get()), window_saveprompt.destroy()))

	entry_savefile_name.grid(row = 0, column = 0, padx = 2, pady = 2)
	button_save.grid(row = 0, column = 1, padx = 2, pady = 2)

def summon_win_setup():
	# creates a window where the user can define the categories and patterns used to generate syllables
		
		button_generate_syllables['state'] = 'disabled'
		button_generate_words['state'] = 'disabled'
		
		window_setup = Toplevel(root)
		window_setup.title('Setup')

		def on_close():
		
			window_setup.destroy()
			button_generate_syllables['state'] = 'normal'
			button_generate_words['state'] = 'normal'
			
		# if there's something in text_categories and/or text_patterns, process it respectively as categories or patterns upon closing the window
		# also deletes whatever was in categories and patterns to avoid duplicates
		def commit_setup():
			
			if text_categories.get(1.0, END) != '\n':
				cats = text_categories.get(1.0, END)
				wgts = text_cweights.get(1.0, END)
				categories = read_cats(cats, wgts)

			if text_patterns.get(1.0, END) != '\n':
				pats = text_patterns.get(1.0, END)
				wgts = text_pweights.get(1.0, END)
				patterns = read_patterns(pats, wgts)
				
			if text_rules.get(1.0, END) != '\n':
				rls = text_rules.get(1.0, END)
				rules = read_rules(rls)

			on_close()
		
		def update():
		
			for cat in categories:
				text_categories.insert(INSERT, f'{cat.index} = {" ".join(cat.members)}\n')
				text_cweights.insert(INSERT, f'{cat.index} = {" ".join(list(map(str, cat.weights)))}\n')
				
			for pat in patterns:
				text_patterns.insert(INSERT, f'{pat.pattern}\n')
				text_pweights.insert(INSERT, f'{str(pat.weight)}\n')
			
			for rule in rules:
				text_rules.insert(INSERT, f'{rule.target} > {rule.result} / {rule.preceded}_{rule.succeeded}\n')
		
		# create widgets
		text_categories = Text(window_setup, width = 50, height = 6, wrap = WORD)
		text_cweights = Text(window_setup, width = 50, height = 6, wrap = WORD)
		text_patterns = Text(window_setup, width = 10, height = 10, wrap = WORD)
		text_pweights = Text(window_setup, width = 10, height = 10, wrap = WORD)
		text_rules = Text(window_setup, width = 30, height = 10, wrap = WORD)
		
		label_categories = Label(window_setup, text = 'Categories')
		label_patterns = Label(window_setup, text = 'Patterns')
		label_rules = Label(window_setup, text = 'Rules')
		
		button_commit_setup = Button(window_setup, width = 8, text = 'Commit', command = commit_setup)
		
		update()
		
		# place widgets
		text_categories.grid(row = 1, column = 0, padx = 3, pady = 0)
		text_cweights.grid(row = 2, column = 0, padx = 3, pady = 0)
		text_patterns.grid(row = 1, column = 1, padx = 3, pady = 3)
		text_pweights.grid(row = 1, column = 2, padx = 3, pady = 3)
		text_rules.grid(row = 1, column = 3, padx = 3, pady = 3)
		
		label_categories.grid(row = 0, column = 0, padx = 3, pady = 0)
		label_patterns.grid(row = 0, column = 1, columnspan = 2, padx = 3, pady = 6)
		label_rules.grid(row = 0, column = 3, padx = 3, pady = 6)
		
		button_commit_setup.grid(row = 2, column = 1, columnspan = 2)
		
		window_setup.protocol('WM_DELETE_WINDOW', on_close)

syllable_count = IntVar()
word_count = IntVar()
word_min_s = IntVar()
word_max_s = IntVar()

# load default starting configuration from config.xml and starting settings from default.xml
settings = load_settings()
config = load_config()
syllable_count.set(config.get('syllables'))
word_count.set(config.get('words'))
word_min_s.set(config.get('min_wordsize'))
word_max_s.set(config.get('max_wordsize'))

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

scale_syllable_count = Scale(root, from_ = 1, to = MAX_SYLLABLE_COUNT, length = 50, orient = HORIZONTAL, variable = syllable_count)
scale_word_count = Scale(root, from_ = 1, to = MAX_WORD_COUNT, length = 50, orient = HORIZONTAL, variable = word_count)
scale_word_min_size = Scale(root, from_ = 1, to = 4, length = 50, orient = HORIZONTAL, variable = word_min_s)
scale_word_max_size = Scale(root, from_ = 1, to = 6, length = 50, orient = HORIZONTAL, variable = word_max_s)

button_generate_syllables = Button(root, width = 12, text = 'Syllables', command = generate_n_syllables)
button_generate_words = Button(root, width = 12, text = 'Words', command = generate_words)
button_clear = Button(root, width = 12, text = 'Clear', command = clear_output)

## PLACE WIDGETS

text_results.grid(row = 0, column = 4, rowspan = 7, padx = 5, pady = 5, sticky = N)

label_syllable_count.grid(row = 0, column = 1, padx = 5, pady = 5, sticky = W)
label_word_count.grid(row = 1, column = 1, padx = 5, pady = 5, sticky = W)
label_word_min_size.grid(row = 2, column = 1, padx = 5, pady = 5, sticky = W)
label_word_max_size.grid(row = 3, column = 1, padx = 5, pady = 5, sticky = W)

scale_syllable_count.grid(row = 0, column = 2, padx = 5, pady = 5, sticky = W)
scale_word_count.grid(row = 1, column = 2, padx = 5, pady = 5, sticky = W)
scale_word_min_size.grid(row = 2, column = 2, padx = 5, pady = 5, sticky = W)
scale_word_max_size.grid(row = 3, column = 2, padx = 5, pady = 5, sticky = W)

button_generate_syllables.grid(row = 0, column = 0, padx = 5, pady = 5, sticky = W)
button_generate_words.grid(row = 1, column = 0, padx = 5, pady = 5, sticky = W)
button_clear.grid(row = 2, column = 0, padx = 5, pady = 5, sticky = W)

root.config(menu = menubar)
root.mainloop()