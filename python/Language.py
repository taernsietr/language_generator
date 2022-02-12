import json
from Phoneme import Phoneme

class Language:
  def __init__(self, language_file):
    with open(language_file) as file:
      language = json.load(file)
      self.phonology = self.parse_phonemes(language["phonemes"])

  def parse_phonemes(self, data):
    out = [Phoneme(p) for p in data]
    return out
