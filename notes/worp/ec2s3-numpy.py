import boto3
import io
import pickle
import numpy
import time

print("hello")
# s3_client = boto3.client('s3')

my_array = numpy.random.randn(10)

filename = 'a_1'

# upload without using disk
my_array_data = io.BytesIO()
pickle.dump(my_array, my_array_data)
my_array_data.seek(0)
s3_client.upload_fileobj(my_array_data, 'stuartmisc', filename)


# download without using disk
my_array_data2 = io.BytesIO()

start = time.time()


s3_client.download_fileobj('stuartmisc', filename, my_array_data2)

end = time.time()
print(end - start)


my_array_data2.seek(0)
my_array2 = pickle.load(my_array_data2)


print('len(my_array2):')
print(len(my_array2))

# check that everything is correct
numpy.allclose(my_array, my_array2)

print(my_array2.shape)

