import pathlib

INSTALL_FOLDER = str(pathlib.Path().absolute())[:-6] + 'languages/' 
DEFAULT_SETTINGS = INSTALL_FOLDER + 'default.xml'
DEFAULT_CONFIG = INSTALL_FOLDER + 'config.xml'

MAX_PATTERN_SIZE = 7
MAX_WORD_SIZE = 5
MAX_SYLLABLE_COUNT = 100
MAX_WORD_COUNT = 100
