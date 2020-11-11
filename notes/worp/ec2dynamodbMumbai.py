import boto3
import time
# https://stackoverflow.com/a/37610132/8870055

print("hello")


client = boto3.client(
    'dynamodb',
    aws_access_key_id='xxx',
    aws_secret_access_key='+xxx',
    region_name='ap-south-1')

table_name = 'Employee'


# pk_name = 'id'
# pk_value = '3b254060-defd-11e9-9de3-0d9853edce2c'
pk_name = 'empID'
pk_value = 400000
start = time.time()

response = client.get_item(TableName=table_name,
                           Key={
                               pk_name: {'N': str(pk_value)}
                           })  # this works!!!!

end = time.time()
print(end - start)




print(response)
