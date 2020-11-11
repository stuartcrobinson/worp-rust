
'''
# install pip:

# https://pip.pypa.io/en/stable/installing/
curl https://bootstrap.pypa.io/get-pip.py -o get-pip.py
python get-pip.py
pip install boto3
'''

import time
import io
import boto3
import json

with open('/creds.json', 'r') as myfile:
    creds = json.loads(myfile.read())

aws_access_key_id=str(creds['wasabi']['root']['access-key'])
aws_secret_access_key=str(creds['wasabi']['root']['secret-key'])

s3 = boto3.client('s3',
                  # endpoint_url='https://s3.eu-central-1.wasabisys.com/',
                  endpoint_url='https://s3.wasabisys.com/',
                  aws_access_key_id=aws_access_key_id,
                  aws_secret_access_key=aws_secret_access_key,
                  # region_name='eu-central-1')
                  region_name='us-east-1')


s3.create_bucket(Bucket='deleteme2345')  #test connection


# bucket_name = 'amsterdam-stuart'
bucket_name = 'va-stuart'
key = 'size_1mb.txt'

start = time.time()
metadata = {}
metadata = s3.head_object(Bucket=bucket_name, Key=key)
elapsed = time.time() - start
print(metadata)
print(elapsed)


# https://www.peterbe.com/plog/fastest-way-to-find-out-if-a-file-exists-in-s3
start = time.time()
response = s3.list_objects_v2(Bucket=bucket_name, Prefix='size_314')
elapsed = time.time() - start
print(response)
print(elapsed)

#NOTE: list_objects_v2 returns within 10 ms !!!! for single-list object responses.
# #   head_object first request is 40 ms, then 10 ms afterwards
# check these speeds for s3 metadata responses ....
# so .... empty wasabi files could be used to store metadata like hive metastore analogy.  where files are, how many subfiles they have, whethere thye're currently locked


start = time.time()
response = s3.get_object(Bucket=bucket_name, Key=key)
# response = s3.download_file(Bucket=bucket_name, Key=key, Filename='/tmp/asdf.txt')
elapsed = time.time() - start
print(elapsed)


# each download executed about 10 times

# eu-central-1 ################################################################################################

# download_file
# ambsterdam wasabi, packet c1.small.x86 (2 x 1Gbps) https://www.packet.com/cloud/servers/c1-small/
# 314mb:    2000 ms
# 1mb:      70 ms - 425 ms   
# 400kb:    40 ms - 270 ms  
# 100kb:    25 ms - 40 ms
# 10kb:     25 ms - 70 ms
# 1kb:      25 ms - 40 ms
# 3b:       25 ms - 220 ms (first)

# get_object
# ambsterdam wasabi, amsterdam packet c1.small.x86 (2 x 1Gbps) https://www.packet.com/cloud/servers/c1-small/
# 314mb:    70 ms - 250 ms (first)
# 1mb:      45 ms - 70 ms (191 first, 388 ms once)
# 400kb:    38 ms - 45 ms (475 ms first)
# 100kb:    35 ms - 60 ms (272 ms first)
# 10kb:     47 ms - 79 ms
# 1kb:      35 ms - 50 ms (70 ms first)
# 3b:       35 ms   40 ms (170 ms sometimes)

# us-east-1 ################################################################################################

# download_file
# ashburn, va wasabi, ashburn, va packet c1.small.x86 (2 x 1Gbps) https://www.packet.com/cloud/servers/c1-small/
# 314mb:    3800 ms - 8200 ms 
# 1mb:      70 ms - 200 ms
# 400kb:    42 ms - 75 ms (2600 ms first)
# 100kb:   
# 10kb:    
# 1kb:     
# 1b:       17 ms - 35 ms (128 ms first)


# get_object
# ashburn, va wasabi, ashburn, va packet c1.small.x86 (2 x 1Gbps) https://www.packet.com/cloud/servers/c1-small/
# 314mb:    75 ms - 110 ms (180 ms first)
# 1mb:      55 ms - 85 ms (300 ms once)
# 400kb:    45 ms - 70 ms (4800 ms once.  maybe a throttle setting???)
# 100kb:    
# 10kb:     
# 1kb:      
# 1b:       45 ms - 65 ms

# eu-central-1 ################################################################################################

# amsterdam wasabi, amsterdam packet c2.large.arm 2x10Gbps https://www.packet.com/cloud/servers/n2-xlarge/

# download_file
# ashburn, va wasabi, ashburn, va packet c1.small.x86 (2 x 1Gbps) https://www.packet.com/cloud/servers/c1-small/
# 314mb:    4300 ms
# 1mb:      100 ms or 150 ms (sometimes 200 - 300 ms) 
# 400kb:    80 ms - 160 ms
# 100kb:   
# 10kb:    
# 1kb:     
# 1b:       30 ms - 60 ms (120 ms sometimes)

# get_object
# ashburn, va wasabi, ashburn, va packet c1.small.x86 (2 x 1Gbps) https://www.packet.com/cloud/servers/c1-small/
# 314mb:    48 ms - 110 ms
# 1mb:      90 ms - 200 ms
# 400kb:    70 ms - 90 ms (300, 400 ms sometimes)
# 100kb:   
# 10kb:    60 ms - 90 ms (350 ms sometimes)
# 1kb:     
# 1b:      65 ms - 110 ms
