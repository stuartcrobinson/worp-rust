import json
import multiprocessing
import boto3
import time
num_of_cpu = multiprocessing.cpu_count()
print num_of_cpu

client = boto3.client('dynamodb')

def lambda_handler(event, context):
    
    pk_name = 'id'
    pk_value = '400'
    table_name = 'test'
    start = time.time()
    
    response = client.get_item(TableName=table_name, Key={pk_name:{'N':str(pk_value)}}) #this works!!!!
    
    elapsed = time.time() - start

    # TODO implement
    return {
        'statusCode': 200,
        'body': json.dumps('Hello from Lambda! '+ str(multiprocessing.cpu_count()) + ' ' + str(elapsed))
    }


######################################################################

# WARM !!!!! possible to warm lambdas??? first dynamodb connection is slow... must be warmed. i think this possible.
#
# 3gb    400kb   80 ms
# 128mb  400kb   125 ms
# 128mb  100kb   50 ms
# 3gb    100kb   15 ms
# 
# 8 100kb in parallel:
# 3gb   40 ms
# 2gb   50 ms
# 1gb   50 ms
# 500mb 50 ms
# 128mb 200 ms
# 
# 

