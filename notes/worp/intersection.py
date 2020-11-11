import random
import time

a = [random.randint(0, 1000) for _ in range(1000)]
b = [random.randint(0, 1000) for _ in range(1000)]

start = time.time()
for _ in range(1000):
    result = [x for x in a if x in b]

elapse = time.time() - start
print(elapse)

#########

start = time.time()
for _ in range(1000):
    result = set.intersection(set(a), set(b))

elapse = time.time() - start
print(elapse)
#########
import random
import time


a = [random.randint(0, 10000000) for _ in range(100000)]
b = [random.randint(0, 10000000) for _ in range(100000)]
c = [random.randint(0, 10000000) for _ in range(100000)]
d = [random.randint(0, 10000000) for _ in range(100000)]
print(len(a))


start = time.time()
result1 = set.intersection(set(a), set(b))
result2 = set.intersection(set(c), set(d))
result = set.intersection(set(result1), set(result2))
elapse = time.time() - start
print(len(result))
print(elapse)




start = time.time()
result= set.intersection(set(a), set(b))
result = set.intersection(set(result), set(c))
result = set.intersection(set(result), set(d))
elapse = time.time() - start
print(len(result))
print(elapse)
#########

import time
from multiprocessing import Pool


def myintersect(mysets):
    s1 = mysets[0]
    s2 = mysets[1]
    return set.intersection(s1, s2)

start = time.time()

pool = Pool()
result = pool.map(myintersect, [[set(a), set(b)], [set(c), set(d)]])
pool.close()
pool.join()
result = set.intersection(set(result[0]), set(result[1]))
elapse = time.time() - start
print(len(result))
print(elapse)





############################################################################################################

import numpy as np

start = time.time()
for _ in range(1000):
    result = np.intersect1d(a, b)

elapse = time.time() - start
print(elapse) 


############################################################################################################
# lambda!
import json

import random
import time

a = [random.randint(0, 1000) for _ in range(100000)]
b = [random.randint(0, 1000) for _ in range(100000)]
c = [random.randint(0, 1000) for _ in range(100000)]
d = [random.randint(0, 1000) for _ in range(100000)]
print(len(a))

def lambda_handler(event, context):
    # TODO implement
    
    start = time.time()
    result1 = set.intersection(set(a), set(b))
    result2 = set.intersection(set(c), set(d))
    result = set.intersection(set(result1), set(result2))
    elapse = time.time() - start
    print(len(result))
    print(elapse)

    return {
        'statusCode': 200,
        'body': json.dumps('Hello from Lambda! '+str(len(result)) + ' ' + str(elapse))
    }

# a = [random.randint(0, 1000) for _ in range(100000)]
# b = [random.randint(0, 1000) for _ in range(100000)]
# c = [random.randint(0, 1000) for _ in range(100000)]
# d = [random.randint(0, 1000) for _ in range(100000)]
#3gb  15ms
#1gb  16ms
#0.5gb  35ms

# a = [random.randint(0, 1000) for _ in range(200000)]
# b = [random.randint(0, 1000) for _ in range(200000)]
# c = [random.randint(0, 1000) for _ in range(200000)]
# d = [random.randint(0, 1000) for _ in range(200000)]
#3gb  30ms
#0.5gb  90ms

# a = [random.randint(0, 1000) for _ in range(400000)]
# b = [random.randint(0, 1000) for _ in range(400000)]
# c = [random.randint(0, 1000) for _ in range(400000)]
# d = [random.randint(0, 1000) for _ in range(400000)]
#3gb  60ms

######################################################


# everything above this has to be ignored cos sets weren't full size cos random number range too small duh

import sortedcontainers
from sortedcontainers import SortedSet
import time
import random



a = [random.randint(0, 10000000) for _ in range(200000)]
b = [random.randint(0, 10000000) for _ in range(50000)]
c = [random.randint(0, 10000000) for _ in range(50000)]
d = [random.randint(0, 10000000) for _ in range(5000)]
print(len(a))


# ss = SortedSet([1, 2, 3, 4, 5])
# result = ss.intersection([4, 5, 6, 7])

# swords = SortedSet(words)
# swords10 = SortedSet(words10)
# swords100 = SortedSet(words100)


sa = SortedSet(a)
sb = SortedSet(b)
sc = SortedSet(c)
sd = SortedSet(d)



start = time.time()
result = sa.intersection(sb, sc, sd)
elapse = time.time() - start
print(len(result))
print(elapse)

# 14 ms  # 5 times faster :) using C-based SortedSet

a = [random.randint(0, 10000000) for _ in range(200000)]
b = [random.randint(0, 10000000) for _ in range(50000)]
c = [random.randint(0, 10000000) for _ in range(50000)]
d = [random.randint(0, 10000000) for _ in range(5000)]


start = time.time()
result1 = set.intersection(set(a), set(b))
result2 = set.intersection(set(c), set(d))
result = set.intersection(set(result1), set(result2))
elapse = time.time() - start
print(len(result))
print(elapse)

# 70 ms

