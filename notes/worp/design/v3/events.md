
*** ####################################  FROM USER: INDEX EDIT REQUEST   ####################################### ***

user query:
    - before User queries, she requests an oracle endpoint from the capitol.
    - during query, User messages the Oracle endpoint.  Oracle knows where all the ferrets are because the capitol keeps her updated
        - capitol is the SINGLE SOURCE OF TRUTH (SSOT) for zergligns locations.  NOT ddb!  only capitol reads the ferrets_table
    - oracle sends the query to ALL the ferrets which each hold a shard of the index.  ferrets return data, oracle generates page1 results

user updates the index (add_doc for example):
    0- if add_doc, estimate disk space needed for request
        - if librarian exists AND doesnt have enough space:
            - increase volume size: https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/requesting-ebs-volume-modifications.html
    1- add_doc request sent to permanent Capitol endpoint
        - request ID generated, stored to INDEX_UPDATES_TABLE📖🧨
            - this is so User can query Capitol and ask if the job's done yet or not
        - capitol responds with a request ID.
    2- the Capitol knows where all the librarians are.  cos LIBRARIANS_TABLE👩‍🏫🧨
        - sends the add_doc request to the librarian for that index
            - include index current total size (from INDEXES_TABLE📘🧨) (👩‍🏫 will compare this to updated size)
    3- librarian👩‍🏫 completes the request on local data, then git-pushes data in the necessary shards to git when complete.
        - measure the new total index size and update INDEXES_TABLE📘🧨 if it's different by more than 5% or something
        - how does she know where to push stuff?  LIBRARYMANAGER tells her on startup via file (passes path as command line arg)
        - messages the Capitol when the job is finished (including the job ID)
        - then, Capitol tells all the chaperones of ferrets of those index shards to update their respective ferrets.
            - a chaperones hatches a duplicate ferret with freshly-pulled data and retires the original
        - (why doenst librarian just tell all the chaperones directly... why bother the capitol?)
    4- what if no librarian exists?
        - capitol check all librarymanagers' spare capacity (by querying each)
            - how much space does it need? check INDEXES_TABLE for total size of index
            - finds a library?
                5- messages that LIBRARYMANAGER👩‍💼, tells her to hire new LIBRARIAN 👩‍🏫 for the index
                    - LIBRARYMANAGER👩‍💼 queries REPOS_TABLE🗃🧨 to get git urls per shard
                        - saves this info and the Capitol🏰 endpoint to file for 👩‍🏫 to read (maybe too long for command line args)
                        - 👩‍🏫 doesn't need to worry about this table.
                        - 👩‍🏫 DOES need to know the Capitol🏰 endpoint
                - waits to get messaged by LIBRARIAN 👩‍🏫 once she's ready
                - capitol🏰 updates the LIBRARIANS_TABLE👩‍🏫🧨 with fresh new librian
                - sends her👩‍🏫 the request
                - goto step 3
            - doesnt find a library?
                - starts a new LIBRARY📚 spotEC2
                    - what size disk? 1.5x index size (minimum is 1GB) (get index size from INDEXES_TABLE📘🧨)
                - waits to get messaged by new LIBRARYMANAGER 👩‍💼 when she's ready
                    - how does 👩‍💼LIBRARYMANAGER know the Capitol endpoint? --> 💐🧨 MISC_TABLE
                - goto step 5
        - go to step 3

🚜machines
    📚library
    🏰Capitol

🦜species
    👩‍💼LIBRARYMANAGER
    👩‍🏫librarian
    🏰Capitol <---- wtf shouldn't this be the 🧠Overmind ?

🧨ddb tables
    🗃🧨REPOS_TABLE
    👩‍🏫🧨LIBRARIANS_TABLE
    📘🧨INDEXES_TABLE
    📖🧨INDEX_UPDATES_TABLE
    💐🧨MISC_TABLE

1.  write code to create all the ddb tables
2.  learn packer    https://www.packer.io/intro/getting-started/provision.html http://blog.shippable.com/build-aws-amis-using-packer ?
2.  write dummy librarymanager rust server
2.  create simple library📚 aws AMI:
        - auto-starts rust program 👩‍💼LIBRARYMANAGER:
            - gets capitol's hello_from_new_library_manager endpoint from MISC_TABLE💐🧨,
            - messages capitol with self's ip address and port
2.  create capitol ec2
3.  write code for capitol to use to create a new 📚library spot-ec2 using a pre-saved aws library AMI


*** ####################################  FROM USER: QUERY   ####################################### ***

assume: capitol, oracle, and index zergforce already exists (1 ec2 per index ferret)
