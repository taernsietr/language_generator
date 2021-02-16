'''
paradigm tester

affixes
    format (e.g. 'post-', '-ing')
    type of affixation (prefix, suffix etc)
    type of process (inflection, derivation)
    type of meaning (tense, augmentative, etc)
    valid word classes (verb, noun, etc)
    specific morphophonemic rules
    is combinable with affixes of same type?

words
    word class
'''

# probably should be a method of a Word class amirite
# remember to distinguish between derivation and inflection
# add option to test custom affix on the fly, or only a specific type of affix (whether by format, meaning or other)
def inflectWord(word, return_type = 'output', customAffix = (None, None), affixFilter = None):
    output = []
    valid_affixes = getAffixes(word.word_class)
    # check output type: output means only the affixed words are returned 
    if return_type == 'output':
        for affix in valid_affixes:
            output.append(affix.apply(word))
    
    return output.join('\n')
