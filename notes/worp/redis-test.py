import redis
import time
r = redis.Redis()
r.mset({"Croatia": "Zagreb", "Bahamas": "Nassau"})


start = time.time()
result1 = r.get("Bahamas")
result2 = r.get("Croatia")
elapsed = time.time() - start
print(result1)
print(result2)
print(elapsed)

#0.000221 seconds per get.  too slow.  so, we have to use language itself for in-memory store

mylist = [1,2,3,4,5,6,1,2,3,4,5,6,1,2,3,4,5,6,1,2,3,4,5,6,1,2,3,4,5,6,1,2,3,4,5,6]


start = time.time()

result1 = mylist[0]
result1 = mylist[1]
result1 = mylist[2]
result1 = mylist[3]
result1 = mylist[4]
result1 = mylist[5]
result1 = mylist[6]
result1 = mylist[7]
result1 = mylist[8]
result1 = mylist[9]
result1 = mylist[0]
result1 = mylist[1]
result1 = mylist[2]
result1 = mylist[3]
result1 = mylist[4]
result1 = mylist[5]
result1 = mylist[6]
result1 = mylist[7]
result1 = mylist[8]
result1 = mylist[9]
result1 = mylist[0]
result1 = mylist[1]
result1 = mylist[2]
result1 = mylist[3]
result1 = mylist[4]
result1 = mylist[5]
result1 = mylist[6]
result1 = mylist[7]
result1 = mylist[8]
result1 = mylist[9]
result1 = mylist[0]
result1 = mylist[1]
result1 = mylist[2]
result1 = mylist[3]
result1 = mylist[4]
result1 = mylist[5]
result1 = mylist[6]
result1 = mylist[7]
result1 = mylist[8]
result1 = mylist[9]
result1 = mylist[0]
result1 = mylist[1]
result1 = mylist[2]
result1 = mylist[3]
result1 = mylist[4]
result1 = mylist[5]
result1 = mylist[6]
result1 = mylist[7]
result1 = mylist[8]
result1 = mylist[9]
elapsed = time.time() - start
print(result1)
print(elapsed)

#getting from list is 0.00018 ms??? not much faster!?!?!?!

#what about numpy array?  exactly the same ...

import numpy as np
mylist = np.array([1,2,3,4,5,6,1,2,3,4,5,6,1,2,3,4,5,6,1,2,3,4,5,6,1,2,3,4,5,6,1,2,3,4,5,6])
mylist



start = time.time()

result1 = mylist[0]
result1 = mylist[1]
result1 = mylist[2]
result1 = mylist[3]
result1 = mylist[4]
result1 = mylist[5]
result1 = mylist[6]
result1 = mylist[7]
result1 = mylist[8]
result1 = mylist[9]
result1 = mylist[0]
result1 = mylist[1]
result1 = mylist[2]
result1 = mylist[3]
result1 = mylist[4]
result1 = mylist[5]
result1 = mylist[6]
result1 = mylist[7]
result1 = mylist[8]
result1 = mylist[9]
result1 = mylist[0]
result1 = mylist[1]
result1 = mylist[2]
result1 = mylist[3]
result1 = mylist[4]
result1 = mylist[5]
result1 = mylist[6]
result1 = mylist[7]
result1 = mylist[8]
result1 = mylist[9]
result1 = mylist[0]
result1 = mylist[1]
result1 = mylist[2]
result1 = mylist[3]
result1 = mylist[4]
result1 = mylist[5]
result1 = mylist[6]
result1 = mylist[7]
result1 = mylist[8]
result1 = mylist[9]
result1 = mylist[0]
result1 = mylist[1]
result1 = mylist[2]
result1 = mylist[3]
result1 = mylist[4]
result1 = mylist[5]
result1 = mylist[6]
result1 = mylist[7]
result1 = mylist[8]
result1 = mylist[9]
elapsed = time.time() - start
print(result1)
print(elapsed)

#getting from list is also 0.00018 ms each ... hmmm 
# i guess that makes sense ... 

#worp will have to fetch thousands+ of in-memory objects... 

