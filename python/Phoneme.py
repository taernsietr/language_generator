import json

class Phoneme:
  def __init__(self, s):
    self.symbol = s["symbol"]
    self.realizations = s["realizations"]
