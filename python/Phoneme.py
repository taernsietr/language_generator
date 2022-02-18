import json

class Phoneme:
  
  def __init__(self, s):
    self.symbol = s["symbol"]
    self.realizations = s["realizations"]

  def symbol(self):
    return symbol

  def graphemes(self):
    return "".join([r for r["grapheme"] in realizations])

  def phones(self):
    return "".join([r for r["phone"] in realizations])
