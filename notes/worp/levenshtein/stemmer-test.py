# https://github.com/snowballstem/pystemmer/blob/master/docs/quickstart.txt
# https://pypi.org/project/PyStemmer/


# Quickstart
# ==========

# This is a very brief introduction to the use of PyStemmer.

# First, import the library:

import Stemmer


# Just for show, we'll display a list of the available stemming algorithms:

print(Stemmer.algorithms())
# [u'arabic', u'basque', u'catalan', u'danish', u'dutch', u'english', u'finnish', u'french', u'german', u'greek', u'hindi', u'hungarian', u'indonesian', u'irish', u'italian', u'lithuanian', u'nepali', u'norwegian', u'porter', u'portuguese', u'romanian', u'russian', u'spanish', u'swedish', u'tamil', u'turkish']

# Now, we'll get an instance of the english stemming algorithm:

stemmer = Stemmer.Stemmer('english')

# Stem a single word:

print(stemmer.stemWord('cycling'))
# cycl

# Stem a list of words:

print(stemmer.stemWords(['cycling', 'cyclist']))
# ['cycl', 'cyclist']

# Strings which are supplied are assumed to be UTF-8 encoded.
# We can use unicode input, too:

print(stemmer.stemWords(['cycling', u'cyclist']))
# ['cycl', u'cyclist']

# Each instance of the stemming algorithms uses a cache to speed up processing of
# common words.  By default, the cache holds 10000 words, but this may be
# modified.  The cache may be disabled entirely by setting the cache size to 0:

print(stemmer.maxCacheSize)
# 10000

stemmer.maxCacheSize = 1000

print(stemmer.maxCacheSize)
# 1000


###############
###############
###############
###############

import Stemmer

import time
import random




stemmer = Stemmer.Stemmer('english')

words = [x.strip().lower().decode('utf-8') for x in open('words_alpha.txt')]


wordsSmall = words[0:10]

start = time.time()

stemmed = stemmer.stemWords(wordsSmall)

elapsed = time.time() - start


myset = set(stemmed)

print(len(words))
print(len(myset))
print elapsed

# 1 ms on mac to stem 10 words