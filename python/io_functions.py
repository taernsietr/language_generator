import os, sys, pathlib
import xml.etree.ElementTree as ET
import classes, functions, config

def load_config(address = config.DEFAULT_CONFIG):
	# returns a dictionary containing program configuration
	
	current = {}
	
	try:
		xml = ET.parse(address)
		print(f'{address} loaded successfully.')
	except:
		sys.exit('Could not load config file.')
	
	root = xml.getroot()

	for item in root.iterfind('*'):
		current[item.tag] = item.text
	
	return current

def load_settings(address = config.DEFAULT_SETTINGS):
	# returns categories, cat weights, patterns, pat weights and rules, respectively, as lists
	
	cats = []
	cwgt = []
	pats = []
	pwgt = []
	rls = []

	try:
		xml = ET.parse(address)
		print(f'{address} loaded successfully.')
	except:
		sys.exit('Could not load settings file.')
		
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

	categories, indices = functions.parse_categories(''.join(cats), ''.join(cwgt))
	patterns, weights = functions.parse_patterns(''.join(pats), ''.join(pwgt))
	rules = functions.parse_rules(''.join(rls))

	settings = classes.Settings(categories, indices, patterns, weights, rules)
	
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
	
	filename = f'{config.INSTALL_FOLDER}{name}.xml'
	
	try:
		new_xml.write(filename, encoding = 'utf-8', xml_declaration=True)
		print(f'{filename} saved successfully.')
	except:
		print('Error saving file.')

def settings_files():
	# returns a list of xml files in the program folder

	with os.scandir(config.INSTALL_FOLDER) as Dir:
	
		valid_settings = []
	
		for file in Dir:
			if file.name.endswith('.xml') and file.name != 'config.xml':
				valid_settings.append(file.name)
	
	return valid_settings
