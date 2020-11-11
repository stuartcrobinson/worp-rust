# TODO begins_with test not implemented yet
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


from boto3.dynamodb.conditions import Key, Attr


table_name = 'report-it-feedbacks-dev'
pk_name = 'id'
pk_value = '16014a70-d4a0-11e9-b645-0d1a403e468a'

# https://boto3.amazonaws.com/v1/documentation/api/latest/reference/services/dynamodb.html
# table = dynamodb.Table(table_name)
dynamodb = boto3.resource(
    'dynamodb',
    aws_access_key_id=aws_access_key_id,
    aws_secret_access_key=aws_secret_access_key,
    region_name='us-east-1'
)
table = dynamodb.Table(table_name)

# https://stackoverflow.com/questions/34171563/how-do-i-query-aws-dynamodb-in-python
response = table.get_item(Key={pk_name: pk_value})


response = table.query(
    IndexName='order_number-index',
    KeyConditionExpression=Key('order_number').eq(myordernumber))


# table_name = 'report-it-feedbacks-dev'
# pk_name = 'id'
# pk_value = '16014a70-d4a0-11e9-b645-0d1a403e468a'

# kwargs = dict(
#     ProjectionExpression='#id',
#     ExpressionAttributeNames={"#id": "_id"})

# if len(search_terms) > 0:
#     kwargs['FilterExpression'] = reduce(
#         lambda x, y: x & y,
#         [Attr('tags').contains(arg) for arg in search_terms])

# if begins_with:
#     if 'FilterExpression' in kwargs:
#         kwargs['FilterExpression'] = kwargs[
#             'FilterExpression'] & Key('_id').begins_with(begins_with)

#     else:
#         kwargs['FilterExpression'] = Key(
#             '_id').begins_with(begins_with)

# while True:
#     res = self._table.scan(**kwargs)
#     for r in res['Items']:
#         yield r['_id']
#     if 'LastEvaluatedKey' in res:
#         kwargs['ExclusiveStartKey'] = res['LastEvaluatedKey']
#     else:
#         break 












# table_name = 'report-it-feedbacks-dev'
# pk_name = 'id'
# pk_value = '16014a70-d4a0-11e9-b645-0d1a403e468a'

# # table_name = 'test'
# # pk_name = 'id'
# # pk_value = '1'

# start = time.time()

# response = client.get_item(TableName=table_name, Key={pk_name:{'S':str(pk_value)}}) #this works!!!!
# end = time.time()
# print(response)
# print(end - start)
