import boto3
import time
import json
#https://stackoverflow.com/a/37610132/8870055

with open('/creds.json', 'r') as myfile:
    creds = json.loads(myfile.read())

aws_access_key_id=str(creds['aws']['user1']['access-key'])
aws_secret_access_key=str(creds['aws']['user1']['secret-key'])

client = boto3.client(
    'dynamodb',
    aws_access_key_id=aws_access_key_id,
    aws_secret_access_key=aws_secret_access_key,
    region_name='us-east-1')


# table_name = 'report-it-feedbacks-dev'
# pk_name = 'id'
# pk_value = '16014a70-d4a0-11e9-b645-0d1a403e468a'

table_name = 'test'
pk_name = 'id'
pk_value = '10'

start = time.time()

response = client.get_item(TableName=table_name, Key={pk_name:{'S':str(pk_value)}}) #this works!!!!
end = time.time()
print(response)
print(end - start)

'''

a lot of these values had huge download times for first time ... dynamodb must be caching.... 
have to expect 45 ms for fresh download D: so downloading 10 of these things in parallel would be like 250+ ms :( :( :( :( 

need to wait and try this test tomorrow or after several hours

1   kb: 10 ms (to 12)
10  kb: 10 ms
50  kb: 13 ms - 42 ms (first time)
100 kb: 13 ms - 17 ms
150 kb: 15 ms - 32 ms
200 kb: 21 ms - 30
400 kb: 25 ms - 40 ms --  starts at 60 ms (19 ms w/ 10gbps network)

39 ms --> 15 ms
30 ms --> 8 ms
42 ms --> 14 ms


'''


##########################################################################################
import boto3
import time
import multiprocessing
num_of_cpu = multiprocessing.cpu_count()
print num_of_cpu

# PARALLEL - batch_get_item about same speed as using Pool.  doing 10 requests in parallel takes a little more than half the time of doing sequentially.  for 100kb values.  

table_name = 'test'
pk_name = 'id'
start = time.time()

response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1000')}}) #this works!!!!
response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1001')}}) #this works!!!!
response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1002')}}) #this works!!!!
response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1003')}}) #this works!!!!
response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1004')}}) #this works!!!!
response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1005')}}) #this works!!!!
response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1006')}}) #this works!!!!
response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1007')}}) #this works!!!!
# response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1008')}}) #this works!!!!
# response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1009')}}) #this works!!!!

end = time.time()
print(end - start)


#################################


# table_name = 'test'
# pk_name = 'id'
# start = time.time()

# response = client.batch_get_item(
#             RequestItems={
# 'test': Keys=[
#     {        pk_name: {'N': str('1001')}    },
#     {        pk_name: {'N': str('1002')}    },
#     {        pk_name: {'N': str('1003')}    },
#     {        pk_name: {'N': str('1004')}    },
#     {        pk_name: {'N': str('1005')}    },
#     {        pk_name: {'N': str('1006')}    },
#     {        pk_name: {'N': str('1007')}    },
#     {        pk_name: {'N': str('1008')}    },
#     {        pk_name: {'N': str('1009')}    },
#     {        pk_name: {'N': str('1010')}    }
# ])  # this works!!!!


import boto3
import json
import decimal
from boto3.dynamodb.conditions import Key, Attr
from botocore.exceptions import ClientError
import time
import boto3
import time
import multiprocessing
num_of_cpu = multiprocessing.cpu_count()
print num_of_cpu



with open('/creds.json', 'r') as myfile:
    creds = json.loads(myfile.read())

aws_access_key_id=str(creds['wasabi']['root']['access-key'])
aws_secret_access_key=str(creds['wasabi']['root']['secret-key'])

client = boto3.client(
    'dynamodb',
    aws_access_key_id=aws_access_key_id,
    aws_secret_access_key=aws_secret_access_key,
    region_name='us-east-1')

# response = client.get_item(TableName=table_name, Key={pk_name:{'N':str('1000')}}) #this works!!!!

# Helper class to convert a DynamoDB item to JSON.
class DecimalEncoder(json.JSONEncoder):
    def default(self, o):
        if isinstance(o, decimal.Decimal):
            if o % 1 > 0:
                return float(o)
            else:
                return int(o)
        return super(DecimalEncoder, self).default(o)

###### just 1

pk_name = 'id'
pk_value = '400'
table_name = 'test'
start = time.time()

response = client.get_item(TableName=table_name, Key={pk_name:{'N':str(pk_value)}}) #this works!!!!

end = time.time()
print(end - start)

# 44 ms to 17 ms
# note -- fetching ONE dynamodb object makes fetching OTHER objects fast!
##############



start = time.time()
try:
    response = client.batch_get_item(
        RequestItems={
            'test': {
                'Keys': [
                    {'id': {'N':'1000'}},
                    {'id': {'N':'1001'}},
                    {'id': {'N':'1002'}},
                    {'id': {'N':'1003'}},
                    {'id': {'N':'1004'}},
                    {'id': {'N':'1005'}},
                    {'id': {'N':'1006'}},
                    {'id': {'N':'1007'}},
                ],            
                'ConsistentRead': True            
            }
        },
        ReturnConsumedCapacity='TOTAL'
    )
except ClientError as e:
    print(e.response['Error']['Message'])
else:
    end = time.time()
    item = response['Responses']
    print("BatchGetItem succeeded:")
    # print(json.dumps(item, indent=4, cls=DecimalEncoder))

print(end - start)

#cold: 67 ms to 57 ms -- 10 in parallel
#cold: 77 ms to 36 ms (w 8 vCPUs and 10gbps)

############


start = time.time()
try:
    response = client.batch_get_item(
        RequestItems={
            'test': {
                'Keys': [
                    {'id': {'N':'1000'}},
                    {'id': {'N':'1001'}},
                    {'id': {'N':'1002'}},
                    {'id': {'N':'1003'}},
                ],            
                'ConsistentRead': True            
            }
        },
        ReturnConsumedCapacity='TOTAL'
    )
except ClientError as e:
    print(e.response['Error']['Message'])
else:
    end = time.time()
    # item = response['Responses']
    # print("BatchGetItem succeeded:")
    # print(json.dumps(item, indent=4, cls=DecimalEncoder))

print(end - start)





# try:
#     response = client.batch_get_item(
#         RequestItems={
#             'test': {
#                 'Keys': [
#                     {'id': '1000'},
#                     {'id': '1001'},
#                     {'id': '1002'},
#                     {'id': '1003'},
#                     {'id': '1004'},
#                     {'id': '1005'},
#                     {'id': '1006'},
#                     {'id': '1007'},
#                     {'id': '1008'},
#                     {'id': '1009'},
#                     {'id': '1010'},
#                 ],            
#                 'ConsistentRead': True            
#             }
#         },
#         ReturnConsumedCapacity='TOTAL'
#     )
# except ClientError as e:
#     print(e.response['Error']['Message'])
# else:
#     end = time.time()
#     item = response['Responses']
#     print("BatchGetItem succeeded:")
#     print(json.dumps(item, indent=4, cls=DecimalEncoder))

# print(end - start)



# ######################################################
# id0 = '1000'
# id1 = '1001'
# id2 = '1002'
# id3 = '1003'
# id4 = '1004'
# id5 = '1005'
# id6 = '1006'
# id7 = '1007'
# id8 = '1008'
# id9 = '1009'
# id10 = '1010'

# response = client.batch_get_item(
#         RequestItems={
#             'test': {
#                 'Keys': [
#                     {'id': id0},
#                     {'id': id1},
#                     {'id': id2},
#                     {'id': id3},
#                     {'id': id4},
#                     {'id': id5},
#                     {'id': id6},
#                     {'id': id7},
#                     {'id': id8},
#                     {'id': id9},
#                     {'id': id10},
#                 ],            
#                 'ConsistentRead': True            
#             }
#         },
#         ReturnConsumedCapacity='TOTAL'
#     )