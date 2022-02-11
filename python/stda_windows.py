import functions, io_functions
from tkinter import *
from tkinter.ttk import *

def summon_win_load(parent):
	# opens the prompt to load settings from a given xml file

	files = io_functions.settings_files()
	
	window_loadprompt = Toplevel(parent)
	window_loadprompt.title('Load Settings')
	
	combo_settingsname = Combobox(window_loadprompt, width = 40, height = 1, values = files)
	button_load = Button(window_loadprompt, text = 'Load', command = lambda: (io_functions.load_settings(io_functions.INSTALL_FOLDER + combo_settingsname.get()), window_loadprompt.destroy()))
	
	combo_settingsname.grid(row = 0, column = 0, padx = 2, pady = 2)
	button_load.grid(row = 0, column = 1, padx = 2, pady = 2)

def summon_win_save(parent):
	# opens the prompt to save settings to a new xml file
	
	savefile = StringVar()
	
	window_saveprompt = Toplevel(parent)
	window_saveprompt.title('Save Settings')
  
	entry_savefile_name = Entry(window_saveprompt, width = 40, textvariable = savefile)
	
	button_save = Button(window_saveprompt, text = 'Save', command = lambda: (io_functions.save_settings(savefile.get()), window_saveprompt.destroy()))

	entry_savefile_name.grid(row = 0, column = 0, padx = 2, pady = 2)
	button_save.grid(row = 0, column = 1, padx = 2, pady = 2)

def summon_win_setup(parent):
	# creates a window where the user can define the categories and patterns used to generate syllables
		
		button_generate_syllables['state'] = 'disabled'
		button_generate_words['state'] = 'disabled'
		button_generate_pseudotext['state'] = 'disabled'
		
		window_setup = Toplevel(parent)
		window_setup.title('Setup')

		def on_close():
		
			window_setup.destroy()
			button_generate_syllables['state'] = 'normal'
			button_generate_words['state'] = 'normal'
			button_generate_pseudotext['state'] = 'normal'
			
		# if there's something in text_categories and/or text_patterns, process it respectively as categories or patterns upon closing the window
		# also deletes whatever was in categories and patterns to avoid duplicates
		def commit_setup():
			
			if text_categories.get(1.0, END) != '\n':
				cats = text_categories.get(1.0, END)
				wgts = text_cweights.get(1.0, END)
				categories = functions.parse_cats(cats, wgts)

			if text_patterns.get(1.0, END) != '\n':
				pats = text_patterns.get(1.0, END)
				wgts = text_pweights.get(1.0, END)
				patterns = functions.parse_patterns(pats, wgts)
				
			if text_rules.get(1.0, END) != '\n':
				rls = text_rules.get(1.0, END)
				rules = functions.parse_rules(rls)

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