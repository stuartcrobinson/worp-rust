#machine 1
#https://www.codementor.io/dushyantbgs/deploying-a-flask-application-to-aws-gnva38cf0
from flask import Flask
app = Flask(__name__)

@app.route('/')
def hello_world():
    return 'Hello, World!'

if __name__ == "__main__":
    app.run(host="0.0.0.0", port=80)

#now get ipv4 public ip = 54.208.112.71 (from aws console UI)


#machine 2
#https://stackoverflow.com/questions/645312/what-is-the-quickest-way-to-http-get-in-python

import urllib2
import time

start = time.time()

# response = client.get_item(TableName=table_name, Key={pk_name:{'S':str(pk_value)}})#this works!!!!
# contents = urllib2.urlopen("http://54.208.112.71/hello").read()
contents = urllib2.urlopen("http://0.0.0.0:80/").read()
# contents = urllib2.urlopen("http://ec2-54-208-112-71.compute-1.amazonaws.com/hello").read()
end = time.time()
print(contents)
print(end - start)


#flask and falcon have same latency for single request ec2 to ec2 - 8 to 10 ms. but then it was 11 to 15 ms!!! 
#on t3.micro and also m5ad.large (10 gbps network speed). latency unchanged. 
# 
# t3.micro-t3.micro
# 100kb 18 ms
# 400kb 26 ms
# 
# t3.micro-m5ad.large
# 400kb 23

# NOTE - connections within a placement group arent faster ...


#python3:


import urllib.request
import time

start = time.time()

# response = client.get_item(TableName=table_name, Key={pk_name:{'S':str(pk_value)}})#this works!!!!
# contents = urllib2.urlopen("http://54.208.112.71/hello").read()
contents = urllib.request.urlopen("http://0.0.0.0:80/").read()
# contents = urllib2.urlopen("http://ec2-54-208-112-71.compute-1.amazonaws.com/hello").read()
end = time.time()
print(contents)
print(end - start)

#2 ms on the same machine


