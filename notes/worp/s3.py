import boto3
import time
import json
#https://stackoverflow.com/a/37610132/8870055

with open('/creds.json', 'r') as myfile:
    creds = json.loads(myfile.read())

aws_access_key_id=str(creds['aws']['user1']['access-key'])
aws_secret_access_key=str(creds['aws']['user1']['secret-key'])

client = boto3.client(
    's3',
    aws_access_key_id=aws_access_key_id,
    aws_secret_access_key=aws_secret_access_key,
    region_name='us-east-1')

bucket_name = 'stuartmisc'
key = 'a_5'

start = time.time()
response = client.download_file(bucket_name, key, '/tmp/hello.txt')
elapsed = time.time() - start
print(elapsed)

########################################################################################

bucket_name='seabiscuitblacklist'
start = time.time()
response = client.list_objects_v2(Bucket=bucket_name, Prefix='B')
elapsed = time.time() - start
print(response)
print(elapsed)


# 30 to 200 ms.  200, 100 ms <-- first and second times.  then faster

########################################################################################
# LAMBDA S3

bucket_name = 'stuartmisc'
# key = 'your-file4.pkl'
# key = 'your-file4.pkl'
# key = 'your-file3.pkl'
key = 'nptest-100000.pkl' #2 MB - 300 MS LAMBDA

client = boto3.client('s3')


def lambda_handler(event, context):
    # TODO implement

    start = time.time()
    # response = client.download_file(bucket_name, key, '/tmp/hello.txt')
    # s3_response_object = s3_client.get_object(Bucket=BUCKET_NAME_STRING, Key=FILE_NAME_STRING)
    s3_response_object = client.get_object(Bucket=bucket_name, Key=key)
    elapsed = time.time() - start
    # print(elapsed)

    return {
        'statusCode': 200,
        'body': json.dumps('Hello from Lambda! ' + str(elapsed))
    }

# #2 MB - 300 MS LAMBDA

########################################################################################
