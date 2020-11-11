import boto3
import json

import time


client = boto3.client(
    'lambda',
    aws_access_key_id='AKIATDTCLTRFIE6APLJ4',
    aws_secret_access_key='+xxx',
    region_name='us-east-1')

event_to_send = 'what'

start = time.time()

res = client.invoke(
    FunctionName='lambda_latency_test_worker',
    InvocationType='Event',
    Payload=bytes(json.dumps(event_to_send))
)


end = time.time()
# print(content)
print(end - start)

print(res['Payload'].read().decode("utf-8"))


res = client.invoke(
    FunctionName='lambda_latency_test_worker',
    InvocationType='Event'
)
print(res['Payload'].read())

t = res['Payload']
j = t.read()
print(j)


with open('file_{}.txt'.format(138)) as f:
    content = f.readlines()
