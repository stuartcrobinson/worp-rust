# https://pypi.org/project/rust-fst/#description

# but also what's this?


import pickle
import random
import time
from rust_fst import Map, Set

# Building a set in memory
keys = ["fa", "fo", "fob", "focus", "foo", "food", "foul"]
s = Set.from_iter(keys)

# Fuzzy searches on the set
matches = list(s.search(term="foo", max_dist=1))
assert matches == ["fo", "fob", "foo", "food"]

# Searching with a regular expression
matches = list(s.search_re(r'f\w{2}'))
assert matches == ["fob", "foo"]

# Store map on disk, requiring only constant memory for querying
items = [("bruce", 1), ("clarence", 2), ("stevie", 3)]
m = Map.from_iter(items, path="/tmp/map.fst")

# Find all items whose key is greater or equal (in lexicographical sense) to
# 'clarence'
matches = dict(m['clarence':])
assert matches == {'clarence': 2, 'stevie': 3}


words = [x.strip().lower().decode('utf-8') for x in open('words_alpha.txt')]
words.sort()
words10 = [x for x in words if random.random() <= 0.1]
words100 = [x for x in words if random.random() <= 0.01]


# Building a set in memory
keys = words
s = Set.from_iter(keys)

start = time.time()
matches = list(s.search(term="food", max_dist=1))
elapsed = time.time() - start
print(len(matches))
print elapsed

# 8ms on mac with all words, "food" dist = 2
# 2ms on mac with all words, "food" dist = 1
##################################

# testing searching for string starts with

start = time.time()
matches = list(s.search_re(r'col.*'))
elapsed = time.time() - start
print(len(matches))
print elapsed
# 2 ms


start = time.time()
result = filter(lambda x: x.startswith('col'), keys)
elapsed = time.time() - start
print(len(matches))
print elapsed
# 100 ms

#################

# testing save fst to compare memory size - straight to s3???


# Save a dictionary into a pickle file.


words = [x.strip().lower().decode('utf-8') for x in open('words_alpha.txt')]
words.sort()
words10 = [x for x in words if random.random() <= 0.1]
words100 = [x for x in words if random.random() <= 0.01]

s = Set.from_iter(words)
s10 = Set.from_iter(words10)
s100 = Set.from_iter(words100)


# Building a set in memory
keys = words
s = Set.from_iter(keys, "fstSet.fst")
# this is 1/4 the size of keys in file!!!!!

s = Set.from_iter(keys)


pickle.dump(s, open("s.p", "wb"))

# doesnt work.  read this abt how to store to disk:
# https://rust-fst.readthedocs.io/en/latest/#rust_fst.Set

#####################################################################################

# testing set intersection


words = [x.strip().lower().decode('utf-8') for x in open('words_alpha.txt')]
words.sort()
words10 = [x for x in words if random.random() <= 0.1]
words100 = [x for x in words if random.random() <= 0.01]

s = Set.from_iter(words)
s10 = Set.from_iter(words10)
s100 = Set.from_iter(words100)

start = time.time()
# i = s.intersection(s10)
i = s.intersection(s10, s100)
si = Set.from_iter(i)
elapsed = time.time() - start
mylen = len(si)
print(mylen)
print elapsed

# this is slower!! 140 ms -- or 400 ms???

# not using fst

start = time.time()
result = set.intersection(set(words), set(words10))
result = set.intersection(set(result), set(words100))
elapse = time.time() - start
print(len(result))
print(elapse)

# this is faster!! 80 ms - 100 ms


words.sort(reverse=True)
words.sort()


start = time.time()
result = set.intersection(set(words), set(words100), set(words10))
elapse = time.time() - start
print(len(result))
print(elapse)

# 80, 100 ms
# NOTE this isn't slower for unsorted set so i think it's bad algo.... ? not benefit from sorted sets


################
# this is fastest so far!!! but i bet something in C or rust would be faster...
# other
# https://www.geeksforgeeks.org/union-and-intersection-of-two-sorted-arrays-2/

# Python program to find intersection of
# two sorted arrays
# Function prints Intersection of arr1[] and arr2[]
# m is the number of elements in arr1[]
# n is the number of elements in arr2[]


def printIntersection(arr1, arr2):
    asdf = []
    i, j = 0, 0
    m = len(arr1)
    n = len(arr2)
    while i < m and j < n:
        if arr1[i] < arr2[j]:
            i += 1
        elif arr2[j] < arr1[i]:
            j += 1
        else:
            # print(arr2[j])
            asdf.append(arr2[j])
            j += 1
            i += 1
    return asdf


# Driver program to test above function
arr1 = [1, 2, 4, 5, 6]
arr2 = [2, 3, 5, 7]
m = len(arr1)
n = len(arr2)
printIntersection(arr1, arr2, m, n)



start = time.time()
# result = set.intersection(set(words), set(words10), set(words100))
result = printIntersection(words, words100)
result = printIntersection(result, words10)
elapse = time.time() - start
print(len(result))
print(elapse)

# 55 ms !!!


# This code is contributed by Pratik Chhajer


##################################
#sortedcontainers 
#


import sortedcontainers
from sortedcontainers import SortedSet
import time
import random


words = [x.strip().lower().decode('utf-8') for x in open('words_alpha.txt')]
words.sort()
words10 = [x for x in words if random.random() <= 0.1]
words100 = [x for x in words if random.random() <= 0.01]


ss = SortedSet([1, 2, 3, 4, 5])
result = ss.intersection([4, 5, 6, 7])

swords = SortedSet(words)
swords10 = SortedSet(words10)
swords100 = SortedSet(words100)



start = time.time()
result = swords.intersection(swords100, swords10)
elapse = time.time() - start
print(len(result))
print(elapse)

# 5 ms  !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!  10x faster than python way
# 20 ms the first time ???????/  D:  D:  D:  D:



start = time.time()
result = swords & swords100 & swords10 
elapse = time.time() - start
print(len(result))
print(elapse)

# slightly slower!!!!