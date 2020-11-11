import copy
import uuid
import re
import pprint
import requests
'''
- writing documents to worp
-- write to wasabi
---- get signed POST wasabi url via a lambda.  input: # of docs.  output: POST url template
-- scan wasabi for new documents
---- via prefix search
-- index documents.  PER DOC:
---- download orig
---- re-upload orig with diff name
---- split by punctuation conf
---- lemmatize
------- IN MEMORY: add stems to ix stems list
---- list all fullwords
------- IN MEMORY: add fullwords to ix fullwords list
---- measure relevance of all stems and fullwords
---- IN MEMORY: insert each fullword and stem (token) and their relev into respective I.I.:  <tok>_docID_relev <-- sorted by docId
---- store tok positions to positions objects:  start_<tok>_<docId> and stop_<tok>_<docId>  <-- list of positions in the doc
-------- saves storage space ?  ... no ... but sometimes we might ONLY need start positions (for quoted multiword string query)
----
-- write index files to ddb



QUESTION:  start with single-dynamodb object for II?  small quickstart?

or start with parallel lambda set intersection approach ....


https://www.schrodinger.com/kb/1842
env variable:
To set an environment variable, enter the following command:
launchctl setenv variable "value"
To find out if an environment variable is set, use the following command:
launchctl getenv variable
To clear an environment variable, use the following command:
launchctl unsetenv variable

python?
https://able.bio/rhett/how-to-set-and-get-environment-variables-in-python--274rgt5
import os
# Set environment variables
os.environ['API_USER'] = 'username'
os.environ['API_PASSWORD'] = 'secret'
# Get environment variables
USER = os.getenv('API_USER')
PASSWORD = os.environ.get('API_PASSWORD')

'''

###########################################################################################################
# ---- get signed POST wasabi url via a lambda.  input: # of docs.  output: POST url template

import boto3
import os
import json

with open('/creds.json', 'r') as myfile:
    creds = json.loads(myfile.read())

s3 = boto3.client('s3',
                  endpoint_url='https://s3.wasabisys.com',
                  aws_access_key_id=str(creds['wasabi']['root']['access-key']),
                  aws_secret_access_key=str(creds['wasabi']['root']['secret-key']))

response = s3.generate_presigned_post(
    Bucket='bucket-from-code',
    Key='uploads/image.jpg',
    Conditions=[
        ['content-length-range', 1, 1048579]
    ]
)

print(json.dumps(response, indent=2))

# ------------------
# use presigned url to upload an obj
# https://boto3.amazonaws.com/v1/documentation/api/latest/guide/s3-presigned-urls.html

# # Demonstrate how another Python program can use the presigned URL to upload a file
# myfile = {'file': 'here is file contents?'}
# http_response = requests.post(
#     response['url'], data=response['fields'], files=myfile)
# # If successful, returns HTTP status code 204
# print('File upload HTTP status code: '+str(http_response.status_code))

# ---------------
# -- write to wasabi


# Demonstrate how another Python program can use the presigned URL to upload a file
object_name = 'esInLambda.txt'
with open(object_name, 'rb') as f:
    print(f.read())
    files = {'file': (object_name, f)}
    # http_response = requests.post(response['url'], data=response['fields'], files=files)
    http_response = requests.post(response['url'], data=response['fields'], files={
                                  'file': ('hi', 'sup2')})
# If successful, returns HTTP status code 204


# ---------------
# above not working.  try using creds to upload any file ????
# working now.  had to delete ~/.aws

# s3.create_bucket(Bucket='worptest')
# s3.put_object(Body='more_binary_data', Bucket='stuarttestdec9', Key='my/key/including/anotherfilename.txt')


###########################################################################################################
# -- scan wasabi for new documents
# ---- via prefix search
# search for docs w/ prefix (not using delimiter here, but we could)
result = s3.list_objects(Bucket='bucket-from-code', Prefix='uploads')

'''
now we need to do this part:

-- index documents.  PER DOC:
---- download orig
---- re-upload orig with diff name
---- stop/stem orig
---- list all stems
'''

# first, upload a bunch of docs to use for searching

pp = pprint.PrettyPrinter(indent=4)

with open('/creds.json', 'r') as myfile:
    creds = json.loads(myfile.read())

s3 = boto3.client('s3',
                  endpoint_url='https://s3.wasabisys.com',
                  aws_access_key_id=str(creds['wasabi']['root']['access-key']),
                  aws_secret_access_key=str(creds['wasabi']['root']['secret-key']))


bucket_name = 'worptest'
# s3.create_bucket(Bucket=bucket_name)


def write_obj(key, contents):
    s3.put_object(Body=contents, Bucket=bucket_name, Key=key)


def read_obj(key):
    # https://stackoverflow.com/questions/37087203/retrieve-s3-file-as-object-instead-of-downloading-to-absolute-system-path
    s3_response_object = s3.get_object(Bucket=bucket_name, Key=key)
    object_content = s3_response_object['Body'].read()
    return object_content


def get_keys_with_prefix(prefix):
    result = s3.list_objects(Bucket=bucket_name, Prefix=prefix)
    return [str(x['Key']) for x in result['Contents']]


write_obj('todo/file10', 'file10')
write_obj('asdf/file20', 'file20')
write_obj('todo/file30', 'file30')
write_obj('asdf/file40', 'file40')
write_obj('todo/file50', 'file50')

keys = get_keys_with_prefix('todo')

for key in keys:
    print(read_obj(key))

###########################################################################################################
# ---- split by punctuation conf
# NO -- lemmatize instead.  stop/lem
# https://blog.algolia.com/natural-languages-in-search/

# steps:
# 0. assert protected_punctuation_conf not contain smartquotes
# 0. replace all smartquotes w/ dumbquotes ONLY IF protected_punctuation_conf contains a quote (single or double)
# 0. rename all protected punctuation ($ --> s8ei3r8weiuo89i)
# 1. split and remove all remaining non-word-or-num chars
# 1b. put back protected punctuation
# 2. now handle each "word" separately
#       iterate through token creation: split?, splitkeep?, remove? <-- do same for all protected punctuation in same sweep

#### PYTHON3 ###

import uuid
import re
import copy


def put_back_punctuation(st, m__uid_punc):
    for uuid in m__uid_punc.keys():
        st = st.replace(uuid, m__uid_punc.get(uuid))
    return st



# "_and" means that both will happen
m__directive_protectedPunctuation = {
    'splitkeep': ['.', '%', '@', '€'],
    'splitkeep_and_keep': ['$'],
    'keep_and_remove': ["'"],
    'split_and_remove': [","]
}

# List of string
smartQuotes = ['‘',  '’', '“', '”']


# 0. assert protected_punctuation_conf not contain smartquotes
for v in m__directive_protectedPunctuation.values():
    if any(smartQuote in v for smartQuote in smartQuotes):
        raise Exception(
            'smart quote found in m__directive_protectedPunctuation')

protected_punctuation_list = []
for v in m__directive_protectedPunctuation.values():
    protected_punctuation_list += v

# 0. rename all protected punctuation ($ --> s8ei3r8weiuo89i)
punct_uuid_dict = [{p: uuid.uuid4().hex} for p in protected_punctuation_list]

m__punc_uid = {p: uuid.uuid4().hex for p in protected_punctuation_list}
m__uid_punc = {v: k for k, v in m__punc_uid.items()}

m__directive_protectedPunctuation__escaped = copy.deepcopy(m__directive_protectedPunctuation)

# escape these special chars: https://stackoverflow.com/a/32212181/8870055
# .^$*+-?()[]{}\|
regex_chars_to_escape = '.^$*+-?()[]{}\|'

for d, p_list in m__directive_protectedPunctuation__escaped.items():
    p_list = ['\\' + p if p in regex_chars_to_escape else p for p in p_list]
    m__directive_protectedPunctuation__escaped[d] = p_list

splitkeep_regex = '(' + \
    '|'.join(m__directive_protectedPunctuation__escaped['splitkeep']) + ')'
splitkeep_and_keep_regex = '(' + \
    '|'.join(
        m__directive_protectedPunctuation__escaped['splitkeep_and_keep']) + ')'
split_and_remove_regex = '|'.join(
    m__directive_protectedPunctuation__escaped['split_and_remove'])



doc = "here is a S.m.A.R.t 'sample-sentence' that ”will”, test, peoples' abilities who won't be 20% best for $23,234 lakersfan@gmail.com"

# 0. replace all smartquotes w/ dumbquotes
doc = doc.replace('“', '"').replace(
    '”', '"').replace('‘', "'").replace('’', "'")

doc = doc.lower()

# 0. rename all protected punctuation ($ --> s8ei3r8weiuo89i)
for p in protected_punctuation_list:
    doc = doc.replace(p, m__punc_uid.get(p))

# 1. split and remove all remaining non-word-or-num chars
docToks0 = re.split('\W', doc)

# 1b. put back protected punctuation
docToks0 = [put_back_punctuation(tok, m__uid_punc) for tok in docToks0]


# 2. now handle each "word" separately
#       iterate through token creation: split?, splitkeep?, remove? keep?<-- do same for all protected punctuation in same sweep

def regexEscape(st):
    if st in regex_chars_to_escape:
        return '\\'+st
    return st


def get_subtoks(tok0):
    toks_out = []
    # p punctuation, d directive
    toks_out1 = set(re.split(splitkeep_regex, tok0))
    #
    toks_out2 = []
    for tok in toks_out1:
        split_result = re.split(split_and_remove_regex, tok)
        split_joined = [''.join(split_result)]
        toks_out2 += split_result + split_joined
    toks_out2 = list(set(toks_out2))
    #
    m__tokmod_toklit = dict()
    msyn__toklit_tokmod = dict()
    p_list = m__directive_protectedPunctuation['keep_and_remove']
    for p in p_list:
        for tok in toks_out2:
            if p in tok:
                # toks_out3 += [tok.replace(p, '')]
                m__tokmod_toklit[tok.replace(p, '')] = tok
                msyn__toklit_tokmod[tok] = tok.replace(p, '')
    toks_out3 = list(set(list(m__tokmod_toklit.keys()) + toks_out2))
    #
    toks_out4 = []
    for tok in toks_out3:
        ar = re.split(splitkeep_and_keep_regex, tok)
        toks_out4 += ar
        #now if any of these new split tokens were derived from a modified token, we need to associate the original literal token with these new toks
        if tok in m__tokmod_toklit.keys():
            for splitTok in ar:
                m__tokmod_toklit[splitTok] = m__tokmod_toklit[tok]
    returner=  set(toks_out3 + toks_out4)
    if '' in returner:
        returner.remove('')
    return list(returner), m__tokmod_toklit, msyn__toklit_tokmod #synonyms like "arent":"aren't"

get_subtoks("asdfasdf@oiuro'iuoi.compound%percent$123,456")
get_subtoks("$234,566")

#need to distinguish literal tokens from modified tokens
#removed: modified tokens
def tokenize(doc):
    # 0. replace all smartquotes w/ dumbquotes
    doc = doc.replace('“', '"').replace(
        '”', '"').replace('‘', "'").replace('’', "'")
    doc = doc.lower()
    # 0. rename all protected punctuation ($ --> s8ei3r8weiuo89i)
    for p in protected_punctuation_list:
        doc = doc.replace(p, m__punc_uid.get(p))
    # 1. split and remove all remaining non-word-or-num chars
    docToks0 = re.split('\W', doc)
    # 1b. put back protected punctuation
    docToks0 = [put_back_punctuation(tok, m__uid_punc) for tok in docToks0]
    subtoks = []
    m__tokmod_toklit = dict()
    msyn__toklit_tokmod = dict()
    for tok in docToks0:
        subtoks_, m__tokmod_toklit_, msyn__toklit_tokmod_ = get_subtoks(tok) #asdfasdf
        subtoks += subtoks_
        m__tokmod_toklit.update(m__tokmod_toklit_)
        msyn__toklit_tokmod.update(msyn__toklit_tokmod_)
    subtoks = set(subtoks)
    return list(subtoks), m__tokmod_toklit, msyn__toklit_tokmod

tokens, m__tokmod_toklit, msyn__toklit_tokmod = tokenize("one two dog ran over won't the hill jumped quickly quietly $123,345 20% puppy@mail.com")

###########################################################################################
# ---- lemmatize
# https://blog.algolia.com/natural-languages-in-search/
# how to lemmatize in python??
# spacy
# 

import spacy
nlp = spacy.load('en')

doc = "one two dog aren't ran arent one over the hill ran ran ran jumped two quickly quietly $123,345 one one one 20% puppy@mail.com"
for spacy_token in nlp(doc):
    print(spacy_token, spacy_token.lemma, spacy_token.lemma_)

from collections import Counter

def get_tokens_and_lemmas(doc):
    #tokens is a set
    tokens, m__tokmod_toklit, msyn__toklit_tokmod = tokenize(doc) 
    m__lemma_orig = {}
    for tok in tokens:
        spdoc = nlp(tok)
        for sptoken in spdoc:
            if str(sptoken) != sptoken.lemma_:
                m__lemma_orig[sptoken.lemma_] = tok
                # print(tok, sptoken, sptoken.lemma, sptoken.lemma_)
    return tokens, m__lemma_orig, m__tokmod_toklit, msyn__toklit_tokmod



###########################################################################
# ---- measure relevance of all stems and fullwords
# hmmmmmm how to get relevance of lemmatized version ......
# should we get relevance BEFORE lemmatizing??? and then apply OG relevance to the lemma?  yeah.
# for non-lemma tokens, just count num occurrences and divide by num chars in document. not the best way -- but easiest and good enough?

import math

doc = "one two dog aren't ran arent won't can't one over the hill ran ran ran jumped two quickly quietly $123,345 one one one 20% puppy@mail.com"
tokens, m__lemma_orig, m__tokmod_toklit, msyn__toklit_tokmod = get_tokens_and_lemmas(doc)

m__literaltoken_count = {}
for tok in tokens:
    count = doc.count(tok)
    #if tok=="aren't", but "arent" is in doc too, add occurrences of each for both tok references
    if tok in msyn__toklit_tokmod:
        modtok = msyn__toklit_tokmod[tok]
        count += doc.count(modtok)
    m__literaltoken_count[tok] = count

m__literaltoken_count

m__modifiedtoken_count = {}
for tokmod, toklit in m__tokmod_toklit.items():
    count1 = doc.count(toklit)
    count2 = doc.count(tokmod)
    m__modifiedtoken_count[tokmod] = count1 + count2

m__modifiedtoken_count

lemma_count_factor = 1 # if "ran" is in the doc 10 times, but you search for "run", it'll b like "run" is there 10*factor times. 

m__token_count = m__literaltoken_count.copy()
m__token_count.update(m__modifiedtoken_count)

m__lemma_count = {}
for lemma,orig in m__lemma_orig.items():
    count = math.ceil(m__token_count[orig] * lemma_count_factor)
    m__lemma_count[lemma] = count
    print(lemma, orig, count)


m__literaltoken_count
m__modifiedtoken_count
m__token_count
m__lemma_count

#what a mess

#### now make m__tok_positions
#### where positions is an array of tuples (start,stop) positions in input documnet

def find_all(a_str, sub):
    # https://stackoverflow.com/a/4665027/8870055
    start = 0
    while True:
        start = a_str.find(sub, start)
        if start == -1: return
        yield start
        start += len(sub) # use start += 1 to find overlapping matches

def get_start_positions(substring, doc):
    return list(find_all(doc, substring))

m__tok_positions = {}
#note - this is currently inefficient bc storing positions for both "arent" and "aren't" (mod and lit toks)
for tok in tokens:
    positions = set(get_start_positions(tok, doc))
    if tok in m__modifiedtoken_count:
        positions += set(get_start_positions(m__modifiedtoken_count[tok], doc))
    m__tok_positions[tok] = get_start_positions(tok, doc)

#
#
# this is very complicated right now ... adding lemmas at this point is stupid i think. 
# lets remove lemmas entirely.  otherwise "be" will highlight "arent", "isn't", "are", etc.
# here are two searches w/ and w/out lemmas used for "excited"
# https://www.google.com/search?sxsrf=ACYBGNRoaw4vxXFsI_fYq_Tf9SGhjtz1tQ%3A1577521051990&source=hp&ei=mw8HXsGfOuzH_Qa1qrDIDw&q=excited&oq=excited&gs_l=psy-ab.3..0i70i249j0l9.957.1729..1830...1.0..0.103.516.6j1......0....1..gws-wiz.......35i39j0i131.WxJwQhLxBxY&ved=0ahUKEwiB2bXv89fmAhXsY98KHTUVDPkQ4dUDCAg&uact=5
# https://hn.algolia.com/?dateRange=all&page=0&prefix=false&query=excited&sort=byPopularity&type=story
# need to control TYPE of lemmas ... types of parts of speech.  nouns plural
# check out google aren't vs arent
# 


###########################################################################
# ---- IN MEMORY: insert each tok and their relev into respective I.I.:  <tok>_docID_relev <-- sorted by docId
# <tok>_docID_relev could be a parquet file.  cols: docId,relevance
# still sorted by docId? 
# ignore parquet for now.  ?
# https://dzone.com/articles/how-to-be-a-hero-with-powerful-parquet-google-and
# parquet filters: https://stackoverflow.com/questions/48714803/how-to-read-parquet-file-with-a-condition-using-pyarrow-in-python ?
# 

'''
TODO - we need to keep track of which tokens are literal tokens vs modified tokens ("aren't" vs "arent")
        - because each literal token needs to have start/stop locations.  and each modified token needs to point to its literal token locations. 
        - NOTE - some modified tokens might ALSO be literal tokens.  searching for this modified token should light up both literal and lemma matches
'''
