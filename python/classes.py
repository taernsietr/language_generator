class Settings:

	def __init__(self, categories, indices, patterns, weights, rules):
		
		self.categories = categories
		self.indices = indices
		self.patterns = patterns
		self.weights = weights
		self.rules = rules

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
