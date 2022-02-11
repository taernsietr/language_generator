import json
from config import *

with open(INSTALL_FOLDER + "languages.json", 'r') as langfile:
    data = langfile.read()

obj = json.loads(data)
