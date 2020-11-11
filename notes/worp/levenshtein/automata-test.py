import automata
import bisect
import random

#https://gist.github.com/Arachnid/491973
# this works

class Matcher(object):
    def __init__(self, l):
        self.l = l
        self.probes = 0
    def __call__(self, w):
        self.probes += 1
        pos = bisect.bisect_left(self.l, w)
        if pos < len(self.l):
            return self.l[pos]
        else:
            return None

'''
import urllib2
# response = urllib2.urlopen('http://python.org/')
response = urllib2.urlopen('https://raw.githubusercontent.com/dwyl/english-words/master/words_alpha.txt')
html = response.read()
'''
#https://github.com/dwyl/english-words/blob/master/words_alpha.txt
# https://github.com/dwyl/english-words/raw/master/words_alpha.txt
# https://raw.githubusercontent.com/dwyl/english-words/master/words_alpha.txt
words = [x.strip().lower().decode('utf-8') for x in open('words_alpha.txt')]
words.sort()
words10 = [x for x in words if random.random() <= 0.1]
words100 = [x for x in words if random.random() <= 0.01]


start = time.time()
m = Matcher(words)
matches = list(automata.find_all_matches('food', 1, m))
elapsed = time.time() - start
print elapsed


# words10
#mac: 30 ms.  food, dist = 2
#mac: 4  ms.  food, dist = 1
# words
#mac 8 ms ????? food, dist = 2 wtf seriously.  4 ms dist = 1



m = Matcher(words)
assert len(list(automata.find_all_matches('food', 1, m))) == 21 #18
print m.probes

m = Matcher(words)
assert len(list(automata.find_all_matches('food', 2, m))) == 388
print m.probes


def levenshtein(s1, s2):
    if len(s1) < len(s2):
        return levenshtein(s2, s1)
    if not s1:
        return len(s2)

    previous_row = xrange(len(s2) + 1)
    for i, c1 in enumerate(s1):
        current_row = [i + 1]
        for j, c2 in enumerate(s2):
            # j+1 instead of j since previous_row and current_row are one character longer
            insertions = previous_row[j + 1] + 1
            deletions = current_row[j] + 1     # than s2
            substitutions = previous_row[j] + (c1 != c2)
            current_row.append(min(insertions, deletions, substitutions))
        previous_row = current_row

    return previous_row[-1]


class BKNode(object):
    def __init__(self, term):
        self.term = term
        self.children = {}

    def insert(self, other):
        distance = levenshtein(self.term, other)
        if distance in self.children:
            self.children[distance].insert(other)
        else:
            self.children[distance] = BKNode(other)

    def search(self, term, k, results=None):
        if results is None:
            results = []
        distance = levenshtein(self.term, term)
        counter = 1
        if distance <= k:
            results.append(self.term)
        for i in range(max(0, distance - k), distance + k + 1):
            child = self.children.get(i)
            if child:
                counter += child.search(term, k, results)
        return counter

## lamnbda:  distance 1 takes a couple ms.  distance 2 takes 15 ms.  for words10.