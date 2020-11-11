import boto3
import io
import pickle
import numpy
import time
#https://stackoverflow.com/a/37610132/8870055

print("hello")
# s3_client = boto3.client('s3')


# dynamodb_resource = boto3.resource('dynamodb')


# def read_table_item(table_name, pk_name, pk_value):
#     """
#     Return item read by primary key.
#     """
#     table = dynamodb_resource.Table(table_name)
#     response = table.get_item(Key={pk_name: pk_value})

#     return response


client = boto3.client(
    'dynamodb',
    aws_access_key_id='xxx',
    aws_secret_access_key='+xxx',
    region_name='us-east-1')


table_name = 'report-it-feedbacks-dev'



# response = client.get_item(
#     TableName=table_name,
#     Key={
#         'id': {
#             'S': '3b254060-defd-11e9-9de3-0d9853edce2c'
#         }
#     },
#     ConsistentRead=True | False,
#     ReturnConsumedCapacity='INDEXES' | 'TOTAL' | 'NONE',
#     ProjectionExpression='feedback'
# )

pk_name = 'id'
pk_value = '3b254060-defd-11e9-9de3-0d9853edce2c'
start = time.time()

response = client.get_item(TableName=table_name, Key={pk_name:{'S':str(pk_value)}})#this works!!!!

end = time.time()
print(end - start)


# table = client.Table('report-it-feedbacks-dev')



# pk_name = 'id'
# pk_value = '3b254060-defd-11e9-9de3-0d9853edce2c'

# # response = client.get_item(TableName='report-it-feedbacks-dev', Key={'id': '3b254060-defd-11e9-9de3-0d9853edce2c'})
# response = client.get_item(Key={pk_name: pk_value})



print(response)
