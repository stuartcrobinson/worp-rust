import urllib.request
import time
import requests
import io

import boto3

s3 = boto3.resource('s3',
                    endpoint_url='https://s3.wasabisys.com',
                    aws_access_key_id='8IPYAZW9AJIA6BKMHBN0',
                    aws_secret_access_key='xxx',
                    region_name='us-east-1')


receiver = io.BytesIO()

start = time.time()

# with open('FILE_NAME', 'wb') as f:
#     s3.download_fileobj(bucket, objectname, f)

s3.download_file(bucket, objectname, 'FILE_NAME')


end = time.time()
print(end - start)


# using direct http ---- maybe sdk is faster????
# url = 'https://s3.us-east-2.wasabisys.com/stuartstatsperformeast2/fgh-2.wav'
url = 'https://s3.us-east-2.wasabisys.com/stuartstatsperformeast2/end1.43mi-3.wav'
start = time.time()
myfile = requests.get(url)
end = time.time()
print(end - start)
open('fgh-2', 'wb').write(myfile.content)


def download_file_with_client(endpoint, access_key, secret_key, bucket_name, key, local_path):
    client = boto3.client('s3', endpoint_url=endpoint, aws_access_key_id=access_key, aws_secret_access_key=secret_key)
    client.download_file(bucket_name, key, local_path)
    print('Downloaded frile with boto3 client')

endpoint_url='https://s3.wasabisys.com'
access_key = '8IPYAZW9AJIA6BKMHBN0'
secret_key = 'xxx'
bucket_name = 'stuartstatsperformeast2'
# key = 'fgh-2.wav'
key = 'end1.43mi-3.wav'
local_path = 'wasabi'


endpoint_url='https://s3.us-east-1.amazonaws.com'
access_key = 'AKIATDTCLTRFIE6APLJ4'
secret_key = '+xxx'
bucket_name = 'stuartmisc'
key = 'a_5'
# key = 'your-file4.pkl'
local_path = 'aws'

start = time.time()
download_file_with_client(endpoint_url, access_key, secret_key, bucket_name, key, local_path)
end = time.time()
print(end - start)



# import boto3
# import os
# import json

# with open('/creds.json', 'r') as myfile:
#     creds = json.loads(myfile.read())
