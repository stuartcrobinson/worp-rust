# https://www.machinelearningplus.com/python/parallel-processing-python/

#this was for downloading dynamodb parallel, stupid cos boto3 already supports

import time

start = time.time()

results = []
for row in data:
    results.append(howmany_within_range(row, minimum=4, maximum=8))

print(results[:10])

end = time.time()
print(end - start)


##############################


start = time.time()

# Step 1: Init multiprocessing.Pool()
pool = mp.Pool(mp.cpu_count())

# Step 2: `pool.apply` the `howmany_within_range()`
results = [pool.apply(howmany_within_range, args=(row, 4, 8)) for row in data]

# Step 3: Don't forget to close
pool.close()

print(results[:10])

end = time.time()
print(end - start)


###################################

start = time.time()

pool = mp.Pool(mp.cpu_count())

results = pool.starmap(howmany_within_range, [(row, 4, 8) for row in data])

pool.close()

print(results[:10])

end = time.time()
print(end - start)


#############


def fibonacci_sequence_of(num):
    first_number = 0
    second_number = 1
    num = int(num)
    if num == 0:
        print("Fibonacci of {} is {}".format(num, num))
    elif num == 1:
        print("Fibonacci of {} is {}".format(num, num))
    else:
        for i in range(2, num):
            new_number = first_number + second_number
            first_number = second_number
            second_number = new_number
        print("Fibonacci of {} is {}".format(num, num))


if __name__ == '__main__':
    input_number = input(
        "Provide comma-seperated-values for multiple values \nFabonacci of : ")
    input_values = []
    input_values = input_number.split(",")
    toc = time.time()
    for i in input_values:
        fibonacci_sequence_of(i)
    tic = time.time()
    time_taken = round((tic-toc)*1000, 1)
    print("It takes {} milli-seconds to calculate the fibonacci of {} concurrently".format(time_taken, input_number))

#######

# importing libraries-
import time
from multiprocessing import Pool

if __name__ == '__main__':
    input_number = input(
        "Provide comma-seperated-values for multiple values \nFabonacci of : ")
    input_values = []
    input_values = input_number.split(",")
    toc = time.time()
  # Making a pool object-
    pool = Pool()
  # Providing numerical values in parellel for computation using .map function
  # .map is a function that is gonna take a function and a list of something(numbes here) in interval and is going to map all those into the processors of our machine
    result = pool.map(fibonacci_sequence_of, input_values)
    tic = time.time()
    time_taken = round((tic-toc)*1000, 1)
    print("It takes {} milli-seconds to calculate the fibonacci of {} in parellel ".format(time_taken, input_number))
  # Waiting for this process to finish running then close-
    pool.close()
    pool.join()



######

import multiprocessing
num_of_cpu = multiprocessing.cpu_count()
print num_of_cpu

######
######
######


# ec2 w/ 4 vCPUs

99999,88888,77777,78787

#It takes 5.2 milli-seconds to calculate the fibonacci of 987 in parellel
#It takes 445.5 milli-seconds to calculate the fibonacci of 99999,88888,77777,78787 concurrently



99999,88888,77777,78787,99999,88888,77777,78787,99999,88888,77777,78787
# It takes 355.0 milli-seconds to calculate the fibonacci of 99999,88888,77777,78787,99999,88888,77777,78787,99999,88888,77777,78787 in parellel
# It takes 1335.9 milli-seconds to calculate the fibonacci of 99999,88888,77777,78787,99999,88888,77777,78787,99999,88888,77777,78787 concurrently



# ec2 w/ 1 vCPUs

99999,88888,77777,78787
# It takes 511.6 milli-seconds to calculate the fibonacci of 99999,88888,77777,78787 in parellel
# It takes 450.5 milli-seconds to calculate the fibonacci of 99999,88888,77777,78787 concurrently



99999,88888,77777,78787,99999,88888,77777,78787,99999,88888,77777,78787
# It takes 1345.6 milli-seconds to calculate the fibonacci of 99999,88888,77777,78787,99999,88888,77777,78787,99999,88888,77777,78787 in parellel
# It takes 1341.4 milli-seconds to calculate the fibonacci of 99999,88888,77777,78787,99999,88888,77777,78787,99999,88888,77777,78787 concurrently




###################
#batch_get_item not that great ... trying w/ real Pool



import boto3
import json
import decimal
from boto3.dynamodb.conditions import Key, Attr
from botocore.exceptions import ClientError
import time

client = boto3.client(
    'dynamodb',
    aws_access_key_id='AKIATDTCLTRFIE6APLJ4',
    aws_secret_access_key='+xxx',
    region_name='us-east-1')
table_name = 'test'
pk_name = 'id'
pk_value = '1006'
# start = time.time()

# response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1000')}}) #this works!!!!


import time
from multiprocessing import Pool


def downloadWithId(num):
    response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('100'+str(num))}}) #this works!!!!
    print('finished ' + str(num))

if __name__ == '__main__':
    toc = time.time()
  # Making a pool object-
    pool = Pool()
  # Providing numerical values in parellel for computation using .map function
  # .map is a function that is gonna take a function and a list of something(numbes here) in interval and is going to map all those into the processors of our machine
    input_values = [0,1,2,3,4,5,6,7,8,9]
    result = pool.map(downloadWithId, input_values)
    tic = time.time()
    time_taken = round((tic-toc)*1000, 1)
    print("It takes {} milli-seconds to calculate the fibonacci of {} in parellel ".format(time_taken, input_values))
  # Waiting for this process to finish running then close-
    pool.close()
    pool.join()


