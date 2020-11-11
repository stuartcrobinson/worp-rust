import time

# fileName = "shootingrangewithjoel_111219_2.01.30pmEST.mp3"   # 320MB - 0.26 seconds
# fileName = "howfar.mp3"
fileName = "Archive.zip"    # 320MB - 0.6 seconds


start = time.time()

# response = client.get_item(TableName=table_name, Key={pk_name:{'S':str(pk_value)}}) #this works!!!!
with open(fileName, mode='rb') as file:  # b is important -> binary
    fileContent = file.read()
    # print(fileContent)

end = time.time()
print(end - start)
