oct 24:

worp_description_for_ross


🛸 WORP

worp is a barebones algolia competitor at a fraction of the price (initial goal is 10x less).  MVP = the first publicly available version. either v1.0.0 or v0.1.0 idk

NOMENCLATURE:

central                 refers to non-sharded data.  documents, locations, and meta are all "central" rocksdbs
shard                   a chunk of data intended to be able to be stored on a different machine.  invinds, sovs, and tivs get sharded.
                            a shard contains bits of all 3. they need to live together for queries.  concept is still a bit vague here but better than before.  some instances in code might
                            use "shard" to basically mean a rocksDB which i'm working on fixing
sovabydized             adj, refers to a field that has its own sovabydid
tivabydized
invinded                adj, refers to a field that has its own inverted index
_                       at the end of a fx that involves rocksdb stuff, underscore is for the version that accepts a rocksdb::DB reference
sortby, sortbys         a field by which you can sort your result documents
timeby, timebys         a field by which you can time-series-graph your result documents
rr                      rocksRoot, the dir containing the actualy rocksdb data
tt                      tokType ("PREFIX" or a string number for num words in phrase)
sov = sovabydid         "sorter values by docId"       - array of f32's indexed by docId
tiv = tivabydid         "timestamp values by docId"    - array of u64's indexed by docId
ii = invind             inverted index
locs                    locations - stored in rocksdb object per collection's field+docId+token
document = doc          single json object.  "record" in algolia i think
field =                 an item in a document.  "attribute" in algolia
table = collection =    a single group of documents (i cant decide between table vs collection so far)
index =                 an index for a specific field in a specific collection.  stored in a discrete db on an ec2
project =               1 user/email/pwd per project
worp =                  the company name and the Search-As-A-Service product name
worpdrive =             internally, this is the specific rust engine powering rust.  maybe in the future, "worp" would be renamed to
                            "worpdrive" like if there were other worp offerings like worptail
worptail = worp comet = log search service
worp orbit or gravity   a "KIP"/eDiscovery service
inuse                   in use -- currently being used.  re: a vintage that we shouldnt' delete cos it's "in use"
pid = "project id"      eg f7W6dC3  (formerly: "etsy" or "npr", but these are now called "project names")
scribe                  a ferret process - rename to "squirrel" ?  the only process allowed to access squirrel dbs
vintner                 a ferret process - 1 vintner per machine.  watches for changes to any vintage in the dirs and updates rocksroots map accordingly
bookkeeper              a ferret process - DEPRECATED
shardName = shardNumber a shard is a chunk of indexed data that, in theory, can be located on an external machine.
                            initially used "shardNumber" cos named "1", "2", etc.  but time-based shards
                            for worptail and customer query analytics have special time-based names
species                 ferret, squirrel, owl, etc? different agents that run on different ec2s (burrow, nest, roost, respectively) in the future distributed version.
                            so right now, that everything's running on a single machine, these terms a bit vague
ferret                  a ferret "ferrets" things out.  ferrets run queries in QUERY_MODE.
squirrel                a squirrel "squirrels" things away.  responsible for modifying a collection.  indexing, creating, deleting, etc
owl                     the owl is very wise and answers complex questions.
                            in the future, queries will be first sent to the owl, who will then distribute the query among
                            different shards, each handled by its own ferret
burrow                  where a ferret lives.  nebulously refers to things used by a ferret.  is it the ec2 where a ferret lives in the future?  or the ferret's clxnsRoot ?
nest                    where a squirrel lives.  nebulously refers to "
roost                   where an owl lives.  nebulously refers "
-------------------------------------------------------- non-MVP stuff:
shard =                 this is a vague term in the worp code right now.  it's erroneously used to refer to any db instance.
                        also, historically a "shard" referred to a single index, only, but i just realized that a shard will need to contain chunks of multiple indexes

MVP features:

note: [bracketed items] are non-critical MVP features / things that can be put off

- website
  - signup/login
  - 1 collection per project
  - search collection, view results as html formatted, raw json, formatted object, [or table (like dynamodb)]
  - payments, upgrade, cancel
  - view usage stats: num docs, num requests
  - [basic query analytics: view the most common queries, and see a timescale graph per query - this can be fairly slow in MVP ]
  - delete a doc
  - insert a new doc
  - [change schema stuff like which fields can be sorted by, which fields are indexed, etc]

- search features
  - prefix search
  - phrase search
  - non-configurable tokenization
  - [fuzzy search]
  - [support chinese,arabic tokenization]

- search limitations (things NOT yet supported)
  - facet search
  - aggregates

- monetization
  - payment tiers, something like this:
    - community:  $p/mo  - up to nD docs and nR requests per month.  probably p,nD,nR =  $0,  20k,  200k
    - asteroid:   $p/mo  - up to nD docs and nR requests per month.  probably p,nD,nR =  $5,  50k,  500k
    - moon:       $p/mo  - up to nD docs and nR requests per month.  probably p,nD,nR =  $10, 100k, 1000k
    - planet:     $p/mo  - up to nD docs and nR requests per month.  probably p,nD,nR =  $15, 150k, 1500k

- backend
  - single ec2 per customer running the worp binary.
  - 2 endpoints per ec2 - one for queries, and another for collection modification requests (insert, delete, etc)


STRATEGY

goal is to quickly release a basic algolia competitor by december.  algolia has a reputation for being very expensive, and they just dramatically increased their prices, so it seems inevitable for a cheaper competitor to appear soon.  the goal is to announce Worp quickly to discourage someone else for trying to make a cheap algolia competitor.  at that point, the goal will be to quickly increase the feature offerings to discourage an established player like google or AWS from building their own cheaper search service once they see that it's possible.

previously, i'd been planning to spend a lot longer on this to get closer to algolia feature parity before releasing.  but i had a kind-of mentoring session chat w/ joe colopy, a tech investor (who founded and sold a company Bronto in durham i used to work for).  and he really stressed releasing as soon as possible, no matter how crappy it is.  he stressed that Bronto was really "duct-taped" together when he first released it, in really terrible shape, barely functional. and then once you have real customer traction you invest in code cleanup, refactoring.  and he strongly discouraged seeking funding.

so since i shouldn't/can't incur financial debt, the goal is to take on as much tech debt for the MVP as is feasible.  not-super-clean code, and manual obligations instead of beautiful automation.  b/c of his chat, for instance, i gutted the distributed capabilities of worp to require it to run on a single ec2 for now with a fairly small collection size.  which is sad but a good starting point.  and maybe instead of an automated way to monitor customer usage, i'll just have to check every few days to see if any customers are over their usage level, and manually pause their accounts if so.  stuff like that.

GO-TO-MARKET

i'm thinking of having a soft release by starting with google ads and direct tech newsletter promotions in eastern europe and asia.  since i think a lot of global tech startups (potential customers) are built by freelancers there rn.  and to start getting feedback before getting first impressions in US/Europe


LONG-TERM PLAN

Road Map:


- convert the MVP worp to a multi-ec2 distributed system to allow for 200k+ docs (maximum is around 16 M with current setup (using 3 bytes for docIds rn) )
- faceted and aggregated searches (do this b4 distributed/sharded search?)
- nested docs
- new product offering: worptail - tweak the worpdrive to optimize for indexing/storing/search of IT logs.  to compete with the ELK stack
   - grow/diverge worptail into a logging + security tool
     - parlay worptail with Shellfish Chitin - a shellfish addon that monitors app versions on servers for security, like Tanium
- grow worp into ecommerce search tool like algolia or yext
   - product recommendations
   - personalized search
- build demo worp search page with a huge index.  start with wikipedia and/or (yelp?) businesses
  - pipe dream - gradually grow this into a google/duckduckgo competitor


BUT WHAT ABOUT SHELLFISH ?

i didn't end up making it past the first round for the shellfish grant.  because i didn't have any customer traction yet, and also one of the judges thought i miscalculated the market size based on an Atlassian customer size analogy.  but i think this is a good thing, because if i'd gotten that MICRO grant for $10k, i would have had to commit to working on shellfish full time for the next year.  i'm still very excited about releasing and promoting shellfish, but i think Worp has a greater immediate potential of being a thriving jobs-creator.  and i think there's a reasonable chance of getting enough Worp customer traction within 5 months to apply to the "SEED" grant for $50k.  given that there's already an established market and demand for search-as-a-service, and algolia has such a strong reputation for being expensive. (versus shellfish which needs to kind of create a brand new market).  (and elasticsearch has a reputation for being fragile and complicated)

so i hope i dont seem too frenetic/flaky with shellfish, but my goal right now is to focus on worp and release a commercially viable Worp version ASAP.  and either promote shellfish afterwards (in a couple months), or in the meantime.


---------------------------------------------------------------------------------------------------------


oct 2

nomenclature:

ii  - invind = inverted index.  currently, a byte array where 3 consecutive bytes are for docId, and the next 2 are for the token's relevance in that doc
rdb - rocksdb

table = collection of documents
"document" - not "record" (like in algolia)



sept 16 2020

    VERSION 2.1 INSTRUCTIONS

updates since v2:
- bulk indexing to a single shard
- some new functions and refactors.  mostly made code more cluttered prob

- with bulk indexing, i confirmed that worp can search 100k docs of 5kb each in under 116 ms.  usually under 40 or 50 ms.
  - and with 50k documents it's considerably faster.  3x as fast for hard queries (30 ms vs 90 ms)


    /*
    VERSION 2.1 INSTRUCTIONS

    cargo run destroy_everything
    # ./ferretReset.sh # this is commented because i have the ferrets running in separate terminal windows so i can watch their output
    cargo run storeEndpoints http://0.0.0.0:3101 http://0.0.0.0:3102 http://0.0.0.0:3103 http://0.0.0.0:3104
    cargo run ci demo 100000
    cargo run i demo beginner starting point document
    cargo run cs demo 1
    cargo run ib demo 1 "/Users/stuartrobinson/repos/worp/worp-rust/docs_generator/hidden/100k_docs_5kb_each.txt" 100
    cargo run cs demo 2
    cargo run ib demo 2 "/Users/stuartrobinson/repos/worp/worp-rust/docs_generator/hidden/100k_docs_5kb_each.txt" 100
    cargo run cs demo 3
    cargo run ib demo 3 "/Users/stuartrobinson/repos/worp/worp-rust/docs_generator/hidden/100k_docs_5kb_each.txt" 100

    cargo run ql demo hello

    # NOTE: this stuff isn't working! and idk why. some git trouble.  that's why we're gutting git next
    # all this stuff used to work, like the beginning of yesterday.  for bulk indexing of a single index.
    cargo run pushIndex demo
    cargo run build__m_shard_urls
    cargo run read__m__shard_urls
    cargo run envFile
    cargo run initRemoteShards
    cargo run pullRemoteShards
    cargo run endpointsShardHealthCheck
    cargo run remoteShardsHealthCheck
    cargo run qr demo about

*/





---------------------------------------------------------------------------------------
old stuff from the spring 2020:

ROADMAP

v3:
- no mut static M_SHARDS_META - pass dbs to http worker
- hyper 12
- 3 separate agents handling execution

v4+:
- token position list - consider this while getting intersection
- implement schema, tags, multiple searchable text fields, facets, 2-byte docIds (works cos subshards)
- search analytics
- auto-balancing?  self-healing?  spot pricing termination listening/healing
see main.rs comments at bottom for more
this is getting complicated .... improve abstraction layers?
----------------------------------------------------------------
v3 goal:

- use hyper 12 instead of hyper 13 for server and httpGets
- remove M_SHARD_META static
    - pass around map instead.  it should have a "mdb" attribute that holds both the readonlydb and a normaldb.  so the singular rocks db access methods
            can use one or the other depending on constant NONMUTABLE flag QUERY_MODE
- implement selfreboot after db change (git pull etc)
- see sandbox repos for this stuff ^

- make main more clear -- split into 3 roles based on input params:
    - ferret -- QUERY_MODE = true -- handle shard queries, data update dance (copy, pull to copy, self reboot, forward requests to understudy as needed, etc )
    - oracle  --  QUERY_MODE = true -- handle queries, remote and local ?
    - dispatch -- QUERY_MODE = false - handle index creation, doc indexing, etc
    - switchboard - this just takes commads and passes them to appropriate agent.

- run these 3 entities as different servers.
    ORACLE
    - no rocksdb!!!!
    - oracle should ONLY handle the old "qr" request.  how does it know when to update its internal in-memory
        - ... wait ... shit.  we can't reboot the oracle
            but it's state needs to change based on how many shards an index has
        - but its only rocksdb is meta.  or is it.  does the oracle have any rocksdb????? NO
        - the oracle just needs:
                - map_indexes:
                    - keys: numShards, maxShardLen, map_shards
                - map_shards:
                    - keys: shardName, endpoints (ip + port), .... ?
            - map_indexes should be a singleton that can be updated by messages from Dispatch
                - so, oracle should start a server to listen for updates from dispatch
    - what abt all that yeoman stuff .... like build__m__shard_meta ... no, that should be in in-memory singleton now.  delete all that stuff
        - request fresh map_indexes copy from DISPATCH via http.  for specific indexes.  or for all indexes. variable request.
    FERRET
    - rocksdb::readonlydb ONLY
        - create on startup.  nonmutable.  pass to http worker
    - gets http requests from oracle
    - selfreboot after git pull commands
        - spawn understudy during git pull.  copy repo to other dir on machine.  start server there to handle requests.  pass requests from main ferret to understudy during main git pull etc
    DISPATCH
    - rocksdb::DB ONLY!
    - send info to ORACLE to update its map_indexes.  get the data from rocksdb



v 2.0 actual:

run these steps, it should work

cargo run destroy_everything
./ferretReset.sh
cargo run storeEndpoints http://0.0.0.0:3101 http://0.0.0.0:3102 http://0.0.0.0:3103 http://0.0.0.0:3104
cargo run ci indie3 3
cargo run i indie3 here
cargo run i indie3 here is
cargo run i indie3 here is a
cargo run i indie3 here is a doc
cargo run i indie3 here is a doc about
cargo run i indie3 here is a doc about ducks
cargo run pushIndex indie3
cargo run ql indie3 about
cargo run build__m_shard_urls
cargo run read__m__shard_urls
cargo run envFile
cargo run initRemoteShards
cargo run pullRemoteShards          #not needed really
cargo run zclean                      #not needed really
cargo run endpointsShardHealthCheck
cargo run remoteShardsHealthCheck
cargo run qr indie3 about




v2: goal:

MAIN program indexes new documents into shards: shard_meta, shard1, shard2, shard3...

shard_master contains things not needed by ferrets such as the documents and metadata like "docIdGraveyard", "numDocs", "shardMaxLen" (and "3423")

numbered shard contains ONLY the ii.  key:  <subshard><token>, as in "xtennis" and "ytennis"
    - no need to put shard number in key -- shard number points to entire rocksdb

how to keep track of rocks db objects?
in map m_shardId_db.  key: shardId.  shardId = <index>|<shardNumber>

so, for index="toyota", shardIds are "toyota:1", "toyota:2", "toyota:meta", ...  (mandatory delimiters, in case an index name ends w/ a number)

when to initialize m_shardId_db?
1.  initialize on startup.  check ferret data folder.  initialize per any existing local repos.  might start empty
2.  update with new db per new index created

how to keep track of indexes and their shard lens?
- dynamodb and meta shard

inputs:

z  <portnumber>

endpoints:
- query <query>
- refresh <shardId>
- load <shardId>
- delete <shardId>
- terminate -- shuts off the server

main:

ci <index> <shardlen>   create new index with given max shard length
i <index> <doc> -       index this new document
                        update all relevant ferrets
ib <index> <doc1> | <doc2> | <doc3> | <doc4> ...
                        index documents in bulk
                        update all relevant ferrets
d <index> <docId>       delete that document, store docId in graveyard
                        update all relevant ferrets
ql <index> <query>      run query locally
qr <index> <query>      run query remotely
cc                      clear filesystem cache
delete_index <index>    delete locally and remotely
g <index>               display docId graveyard


implementation:

ci

create a new local folder at indexroot and open a new rocksdb::DB connection to it, and store all this info in m_shardId_db.
initialize new index with "meta" shard

i

determine the new doc's docId, then get the shardNumber, then use m_shardId_db to store it at the right place
might need to create a new shard (new dir, new repo, new rocksdb::DB, and update m_shardId_db)














v1 - creates rocksdb, input docs by bulk or individually, tokenize.  use "z" to run binary for ferret.  give it port number maybe?