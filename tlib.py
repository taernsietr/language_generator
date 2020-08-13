## IMPORTS

import string
import random
import re

## CLASSES

class Category:

	def __init__(self, index, members, weights):

		self.index = index
		self.members = members.copy()
		self.weights = weights.copy()

class Pattern:
	
	def __init__(self, pattern, weight):
		
		self.pattern = pattern
		self.weight = weight

class Rule:
	
	def __init__(self, target, result, env = ('', '')):
		
		self.target = target
		self.result = result
		self.env = env
		
		if self.env[0].isupper() and self.env[1].isupper():
			self.envtype = 'both'
		elif self.env[0].isupper():
			self.envtype = 'pre'
		elif self.env[1].isupper():
			self.envtype = 'pos'
		else:
			self.envtype = 'none'
		
	def find(self):
		f = self.env[0] + self.target + self.env[1]
		return f 
	
	def put(self):
		p = self.env[0] + self.result + self.env[1]
		return p

## GLOBAL VARIABLES

all_syllables = []	# used with syl_all_frompattern()
all_words = []		# used with word_all_fixedlength()

## MISC FUNCTIONS

def pattern_max(categories, indices, pattern):
# returns the max number of unique syllables a given pattern can generate
	
	max_syl = 1
	
	for symbol in pattern:
		max_syl = max_syl * len(categories[indices.index(symbol)].members)
	
	return max_syl
	
## GENERATOR FUNCTIONS

def pseudotext(inp, size = 16):
# generates pseudotext of {size} length
# repeats words, may not include all words in input

	text = []
	wgts = []
	punct = ',.?!:;'
	
	words = sorted(inp, key = len)
	maxlen = max(map(len, words))
	
    # define the max chance of a given word appearing in reverse proportion to the max word size in the list
	w = 0
	for word in words:
		w = w + random.uniform(0.1, maxlen/len(word))
		wgts.append(w)

	for i in range(size):
		this_word = random.choices(words, cum_weights = wgts)
		text.append(this_word[0])
	
	return text

def words_n(syllables, min_len, max_len, words = 1):
# generate {words} random words of length between min_len and max_len using output_syllables
	
	word = ''
	output = []
	
	for _ in range(words):
		word_len = random.randint(min_len, max_len)
		for _ in range(word_len):
			word = word + random.choice(syllables)
		output.append(word)
		word = ''
	
	return output

def words_all_fixedlength(syllables, len, word = ''):
# generate all possible words given output_syllables

	global all_words

	if len == 1:
		for syllable in syllables:
			all_words.append(word + syllable)
	else:
		for syllable in syllables:
			words_all_fixedlength(syllables, len - 1, word = word + syllable)

def words_all(syllables, min_len, max_len):
	
	for length in range(min_len, max_len+1):
		words_all_fixedlength(syllables, length)

def syl_n(categories, indices, patterns, pwgts, syllables = 1):
# generate n random syllables according to patterns

	# syllable = str()					remove if working
	output = []

	for _ in range(syllables):
		while True:
			syllable = ''
			this_pattern = random.choices(patterns, pwgts)
			this_pattern = this_pattern[0].pattern
			for symbol in this_pattern:
				this_cat = categories[indices.index(symbol)]
				letter = random.choices(this_cat.members, this_cat.weights)
				syllable = syllable + letter[0]
			if syllable not in output:
				output.append(syllable)
				break
	
	return output

def syl_all_frompattern(categories, indices, pattern, syllable = ''):
# generate all possible syllables from a given pattern
	
	global all_syllables
	
	this_cat = categories[indices.index(pattern[0])]
	
	# if we are at the last member of a pattern, append results to output_syllables
	if len(pattern) == 1:
		for symbol in this_cat.members:
			all_syllables.append(syllable + symbol)
	
	# otherwise, add current letter to this syllable and go to the next symbol
	else:
		for symbol in this_cat.members:
			syl_all_frompattern(categories, indices, pattern[1:], syllable + symbol)

def syl_all(categories, indices, patterns):
# generate all possible syllables from every pattern in patterns

	for pattern in patterns:
		syl_all_frompattern(categories, indices, pattern.pattern)

## PARSER FUNCTIONS

def apply_rules(categories, indices, text, ruleset):
# apply the rules in ruleset to text
# currently might have some overwriting issues with rules applying where other rules have been applied
	
	# turn lists into string before processing
	output = ' '.join(text) if isinstance(text, list) else text
	
	if len(ruleset) != 0:
		for rule in ruleset:
		
			# specific rules													## check if it's working
			if rule.envtype == 'none':
				output = re.sub(rule.find(), rule.put(), output)
				
			# categoric rules													## if this works I'm a prodigy. but it won't
			if rule.envtype == 'both':
				for onset in categories[indices.index(rule.env[0])].members:
					for coda in categories[indices.index(rule.env[1])].members:
						output = re.sub(onset + rule.target + coda, rule.put(), output)
					
			elif rule.envtype == 'pre':
				for onset in categories[indices.index(rule.env[0])].members:
					output = re.sub(onset + rule.target + rule.env[1], rule.put(), output)
			
			elif rule.envtype == 'pos':
				for coda in categories[indices.index(rule.env[1])].members:
					output = re.sub(rule.env[0] + rule.target + coda, rule.put(), output)

	# if input was a list, output as a list
	if isinstance(text, list):
		output = output.split(' ')

	return output

def parse_categories(input, wgt = None):
# reads inp containing text in the format of 'C = p t tsh j'
# and parses categories where C is an index and other groupings 
# after the equal sign are members.
# wgt is a list of probability weights used later with random.choices
# if there isn't one weight per member, raises an error
# currently won't check for duplicate categories, or if an index is already in use

	categories = []
	indices = []

	cats_found = re.findall('([A-Z]{1}) = ([^A-Z0-9\n]+)', input, re.MULTILINE)
	wgts_found = re.findall('([A-Z]{1}) = ([^A-Za-z\n]+)', wgt, re.MULTILINE)
	
	if len(cats_found) != len(wgts_found):
		print(f'Warning: numbers of categories and weights are not matched. (cats = {str(len(cats_found))}), wgts = {str(len(wgts_found))}).')
	
	for cat in range(len(cats_found)):
		this_members = re.findall('\w+', cats_found[cat][1])
		this_weights = re.findall('\w+', wgts_found[cat][1])
		this_weights = list(map(int, this_weights))
		
		categories.append(Category(cats_found[cat][0], this_members, this_weights))
		indices.append(categories[cat].index)
		
	return categories, indices

def parse_patterns(input, weight):
# reads inp containing capital alphabetic letters and adds valid sequences to patterns
# wgt is a list of probability weights used later with random.choices
# if there isn't one weight per member, raises an error
# for now, hardcoded 5 as maximum pattern size
	
	patterns = []
	weights = []
	
	pats_found = re.findall(' ?([A-Z]{1,5}) ?', input, re.MULTILINE)
	wgts_found = re.findall('(\d{1,3})', weight, re.MULTILINE)

	# error checking
	if len(pats_found) != len(wgts_found):
		print(f'Error: numbers of patterns and weights are not matched. (cats = {len(pats_found)}, wgts = {len(wgts_found)}).')

	for pat in range(len(pats_found)):
		patterns.append(Pattern(pats_found[pat], int(wgts_found[pat])))
		weights.append(patterns[pat].weight)
		
	return patterns, weights

def parse_rules(input):
# reads inp containing a string in the format initial > final / env1_env2;
# which represents a phonotactical transformation rule
# currently, remember that rules must end with a semicolon, and avoid rules in the format a > b / _

	rules = []
	
	rules_found = re.findall('(\w+) *> *(\w+) *\/? *(\w*) *_? *(\w*) *;', input, re.MULTILINE)
	
	if len(rules_found) == 0:
		print('Warning: no rules found.')
	else:
		for rule in rules_found:
			if '_' in rule[2] or rule[3]:
				print('Error: Invalid rule parsing.')
				rules_found.remove(rule)
			else:
				rules.append(Rule(rule[0], rule[1], (rule[2], rule[3])))
		
	return rules