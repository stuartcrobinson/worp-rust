from nltk.stem import WordNetLemmatizer
import nltk
from nltk.corpus import wordnet
from nltk.corpus import wordnet as wn
from nltk.stem.wordnet import WordNetLemmatizer
from nltk import word_tokenize, pos_tag
from collections import defaultdict
tag_map = defaultdict(lambda: wn.NOUN)
tag_map['J'] = wn.ADJ
tag_map['V'] = wn.VERB
tag_map['R'] = wn.ADV

text = "guru99 is a totally new kind of learning experience."
tokens = word_tokenize(text)
lemma_function = WordNetLemmatizer()
for token, tag in pos_tag(tokens):
    lemma = lemma_function.lemmatize(token, tag_map[tag[0]])
    print(token, "=>", lemma)


# https://www.machinelearningplus.com/nlp/lemmatization-examples-python/
# Lemmatize with POS Tag


def get_wordnet_pos(word):
    """Map POS tag to first character lemmatize() accepts"""
    tag = nltk.pos_tag([word])[0][1][0].upper()
    tag_dict = {"J": wordnet.ADJ,
                "N": wordnet.NOUN,
                "V": wordnet.VERB,
                "R": wordnet.ADV}
    return tag_dict.get(tag, wordnet.NOUN)


# 1. Init Lemmatizer
lemmatizer = WordNetLemmatizer()

# 2. Lemmatize Single Word with the appropriate POS tag
word = 'feet'
print(lemmatizer.lemmatize(word, get_wordnet_pos(word)))

# 3. Lemmatize a Sentence with the appropriate POS tag
sentence = "The striped bats are hanging on their feet for best"
print([lemmatizer.lemmatize(w, get_wordnet_pos(w))
       for w in nltk.word_tokenize(sentence)])
# > ['The', 'strip', 'bat', 'be', 'hang', 'on', 'their', 'foot', 'for', 'best']


##############
# spaCy vs nltk for lemmatization
# https://github.com/explosion/spaCy/issues/1837
# https://www.reddit.com/r/LanguageTechnology/comments/69xbkc/question_spacy_or_nltk/
# https://spacy.io/api/lemmatizer
#
# remove stopwords?
# https://towardsdatascience.com/a-short-introduction-to-nlp-in-python-with-spacy-d0aa819af3ad
# algolia hackernews doesnt remove stopwords ???!!?!??!!?
# https://discourse.algolia.com/t/get-list-of-stop-words/1565
# https://www.algolia.com/doc/api-reference/api-parameters/removeStopWords/ algolia recommends not to use????
# spacy?
# punctuation?
# different rules for different puncutation
# -     in phone number? keep, and keep whole phone number as a token
# -     hmm "node js" matches ...node-js... https://hn.algolia.com/?dateRange=all&page=0&prefix=true&query=%22node%20js%22&sort=byPopularity&type=story
# -     replace with a space
# '     remove and keep (dont split) "won't" --> "wont" and "won't"
# .     idk... try "s.m.a.r.t." in  https://hn.algolia.com/?dateRange=all&page=0&prefix=true&query=s.m.a.r.t.&sort=byPopularity&type=story
# .     also try "os x." vs "os x"
# .     so ... ??!?!  they index twice for every word followed by a . ??  i think --- just index periods when after and before a lteter
# .     so for "node.js", 3 toks: "node", "js", and "node.js" ... when someone types "node.", it searches all toks that start with "node."
# .     should also index as "nodejs" ??? algolia doesn't.
# /     algolia replaces w space
# ^     algolia replaces w space
# %     algolia replaces w space
# @     algolia replaces w space
# -     https://hn.algolia.com/?dateRange=all&page=0&prefix=true&query=867-5039&sort=byPopularity&type=story

# SPACES:  single space is treated differently from double-space!
# single space is removed in query
# https://hn.algolia.com/?dateRange=all&page=0&prefix=true&query=can%20not%20hing&sort=byPopularity&type=story
# vs
# https://hn.algolia.com/?dateRange=all&page=0&prefix=true&query=can%20not%20&sort=byPopularity&type=story

# synonyms are rarely used.  in google/ hn manual tests.

# allow regex when results are less than 100 or something !!!!

# so what next ... dont remove stopwords, do lemmatize ... how to tokenize?  determine punctuation splitting/removal pattern first?

# go through punctuation rules 1 by 1.  how to encode rules?  several options:
# split, remove, keep.  ?  and combinations of?
# first split by spaces ... and then for each result, handle internal punctuation.  do each word as whole.
# like.... "S.M.A.R.T." <-- do periods together... don't make like 20 different tokens per period

#  S.M.A.R.T  --> "s.m.a.r.t"  and [s, m, a, r, t]

# .     split and keep      remove isolated
# -     split               remove
# '     remove and keep     remove isolated
# /     split               remove
# &     split               remove
# ^     split               remove
# *     split               remove
# ,     split and remove    remove              unless surrounded by chars.  then
# %     split and keep      remove isolated
# @     split and keep      remove isolated
# $     split and keep      remove isolated (same for all currency symbols)

# should these be configurable ?????  yeah.  read these settings from a json file.

# split_removeIsolated
# split_keepIsolated
# remove
# keep
# --------------------
# default for non-specified punctuation: split_removeIsolated

# for split and remove, that == replace w/ a space?

# what about S.M.-A.R.T ?
# first handle the split/remove cases
# well, first split the whole doc by all the split/remove chars including spaces
# so we coudl have S.M'A.R. --> s,ma,m'a,r,s.m'a.r
# $35,234 --> split ,  --> $35  234  --> $  35  234
# $35,234 --> remove , --> $35234  --> $  35234
# -->  $, 35, 234, 35234, $35, $35234
# glitterbombs@gmail.com (both punctuation is split and keep so do it together)
#   glitterbombs@gmail.com, glitterbombs, gmail, com

# use queryType: prefixAll, prefixNone, prefixLast
# algolia features:
# https://www.algolia.com/doc/api-reference/api-parameters/queryType/
# https://www.algolia.com/doc/api-reference/api-parameters/removeWordsIfNoResults/
# https://www.algolia.com/doc/api-reference/api-parameters/advancedSyntax/
# https://www.algolia.com/doc/api-reference/api-parameters/optionalWords/
# https://www.algolia.com/doc/api-reference/api-parameters/disablePrefixOnAttributes/
# https://www.algolia.com/doc/api-reference/api-parameters/disableExactOnAttributes/
# minWordSizefor1Typo
# minWordSizefor2Typos
# typoTolerance
# https://www.algolia.com/doc/api-reference/api-parameters/allowTyposOnNumericTokens/
# etc
# https://www.algolia.com/doc/api-reference/api-parameters/aroundLatLng/


# -->  lemmatize.  dont remove stopwords.

# steps:
# 1. split and remove ONLY split_removeIsolated punctuation (inlcuding spaces)
# 2. split ONLY split_keepIsolated
# 3. for each multi-char token:
#       split and remove
#       or
#       split and keep
#
# NOTE - for each token, we need to know its location in original document .... how?
#           - just search for all tokens in document after splitting is complete??? inefficient... oh well! good enough 4 now
# now we have tokens...
# tokens and locations
# now get relevance of token for each token (num occurrences vs doc size? vs total num tokens?)
# save these values to object per token.
# <tok>_<field>_<docIds>
# <tok>_<field>_<docId>_start
# <tok>_<field>_<docId>_stop
# <tok>_<field>_<docId>_relevance 
# ? or
# <tok>_<field>_<docIds>
# <tok>_<field>_<docId>_start_stop_strength
# ?
# and add tokens to pure tokens list per field
# toks_<field>
#
# these objects are saved and updated in memory ^ and saved to file (prod: wasabi, dev: laptop) at regular intervals
#
# also per doc, save:
# <docId>_original
# <docId>_meta  <-- contains tags and sortables  (or shoudl this be in "original"?  keep separate for now.)
#
# so this is the tricky one:
# <tok>_<field>_docIds_<n>
# object max size 400kb.  object must always be sorted.  how to keep inventory?  how to know which 'n' to save to.
# need metadata per tok/field:
# <tok>_<field>_docIds_meta  <-- this stores the total number of <tok>_<field>_docIds objects.  the last object (highest n) is always the one to write to
#  so, when a size get to a certain point, immediately creage the next object if it doesn't exist yet. 
#
# easy :) 