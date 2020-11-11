find ~/repos/worp/worp-rust -type f -name '.DS_Store' -delete
find ./.. -type f -name '.DS_Store' -delete
export    RUST_BACKTRACE 1
cargo build
cargo run  dc c1
cargo run  dc c1_hiddenQueryCollection

cargo run  dc w1 c1
cargo run  dc w1 c1_hiddenQueryCollection

cargo run  cc '{
                    "pid": "w1",
                    "collection ": "c1",
                    "do_log_queries": true,
                    "global_secret" "doesnt matter here.  only gets read from endpoint"
                  }'

cargo run ps w1 c1
cargo run ps w1 c1_hiddenQueryCollection
# cargo run  ic c1_hiddenQueryCollection '{"_timestamp": 1603985111111, "query": "inserted" }'


cargo run  ic w1 c1 '{"num": 1, "quote": "Reality continues to ruin my life." }'
cargo run  ic w1 c1 '{"num": "2", "quote": "It\'s not denial. I\'m just selective about the reality I accept." }'
cargo run  ic w1 c1 '{"num": 4, "quote": "You know, Hobbes, some days even my lucky rocket ship underpants don\'t help." }'
cargo run  ic w1 c1 '{"num": 5, "quote": "I\'m killing time while I wait for life to shower me with meaning and happiness." }'


cargo run  qc '{
                  "pid": "w1",
                  "collection ": "c1",
                  "queries": [{ "query": "rea" }],
                  "do_highlights_tagged": true,
                  "highlight_pre_tag": "🟢",
                  "highlight_post_tag": "🟥",
                  "sort_by": [
                    {
                      "name": "num",
                      "is_descending": true
                    }
                  ]
                }'


# you have to use this do_log_query_for_analytics param or else it'll try to index a query document at c1_hiddenQueryCollection_hiddenQueryCollection
cargo run  qc '{
                  "pid": "w1",
                  "collection ": "c1_hiddenQueryCollection",
                  "queries": [{ "query": "rea" }],
                  "do_log_query_for_analytics": false
                }'



########################################################################################################################################################################################


# /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt

cargo run ib w1 c1 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt 10

# some combination of these to test indexing/unindexing
sh commands.sh
export    RUST_BACKTRACE 1

cargo run schema w1 c1
#test indexing
cargo run qc '{
  "pid": "w1",
  "collection ": "c1",
  "queries": [{ "query": "reality" }],
  "do_highlights_tagged": true,
  "highlight_pre_tag": "🟢",
  "highlight_post_tag": "🟥"
}'
cargo run unindexFieldInvinds w1 c1 quote
cargo run indexTokensWithoutPrefixes w1 c1 quote
#test prefixing
cargo run qc '{
  "pid": "w1",
  "collection ": "c1",
  "queries": [{ "query": "rea" }],
  "do_highlights_tagged": true,
  "highlight_pre_tag": "🟢",
  "highlight_post_tag": "🟥"
}'
cargo run qc '{
  "pid": "w1",
  "collection ": "c1",
  "queries": [{ "query": "reality" }],
  "do_highlights_tagged": true,
  "highlight_pre_tag": "🟢",
  "highlight_post_tag": "🟥"
}'
cargo run unindexPrefixesOnly w1 c1 quote
cargo run indexWithPrefixes w1 c1 quote
#test timing
cargo run qc '{
  "pid": "w1",
  "collection ": "c1_hiddenQueryCollection",
  "queries": [{ "query": "ruin" }],
  "do_highlights_tagged": true,
  "highlight_pre_tag": "🟢",
  "highlight_post_tag": "🟥"
}'
cargo run untime w1 c1_hiddenQueryCollection _timestamp
cargo run time w1 c1_hiddenQueryCollection _timestamp
#test sorting
cargo run  qc '{
  "pid": "w1",
  "collection ": "c1",
  "queries": [{ "query": "rea" }],
  "do_highlights_tagged": true,
  "highlight_pre_tag": "🟢",
  "highlight_post_tag": "🟥",
  "sort_by": [
    {
      "name": "num",
      "is_descending": true
    }
  ]
}'
cargo run unsort w1 c1 num
cargo run sort   w1 c1 num
#test bulk upload
cargo run ib w1 c1 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt 10
#test delete doc
cargo run ld w1 c1
cargo run dd w1 c1 3
cargo run qc '{
  "pid": "w1",
  "collection ": "c1",
  "queries": [{ "query": "lucky" }],
  "do_highlights_tagged": true,
  "highlight_pre_tag": "🟢",
  "highlight_post_tag": "🟥"
}'




# cargo run qc '{
#                                     "pid": "w1",
#                                     "collection ": "c1_hiddenQueryCollection",
#                                     "queries": [{ "query": "rea" }],
#                                     "do_highlights_tagged": true,
#                                     "highlight_pre_tag": "🟢",
#                                     "highlight_post_tag": "🟥"
#                                   }'
# cargo run qc '{
#                                     "pid": "w1",
#                                     "collection ": "c1_hiddenQueryCollection",
#                                     "queries": [{ "query": "ruin" }],
#                                     "do_highlights_tagged": true,
#                                     "highlight_pre_tag": "🟢",
#                                     "highlight_post_tag": "🟥"
#                                   }'
# cargo run untime w1 c1_hiddenQueryCollection _timestamp
# cargo run time w1 c1_hiddenQueryCollection _timestamp
# cargo run unindexPrefixesOnly w1 c1 quote
# find ./.. -type f -name '.DS_Store' -delete                 # this is for watching the rocksdir in Finder to see its size change
# cargo run unindexFieldInvinds w1 c1 quote
# cargo run indexTokensWithoutPrefixes w1 c1 quote
# cargo run qc '{
#                                     "pid": "w1",
#                                     "collection ": "c1",
#                                     "queries": [{ "query": "reality" }],
#                                     "do_highlights_tagged": true,
#                                     "highlight_pre_tag": "🟢",
#                                     "highlight_post_tag": "🟥"
#                                   }'
# cargo run indexWithPrefixes w1 c1 quote
# cargo run qc '{
#                                     "pid": "w1",
#                                     "collection ": "c1",
#                                     "queries": [{ "query": "rea" }],
#                                     "do_highlights_tagged": true,
#                                     "highlight_pre_tag": "🟢",
#                                     "highlight_post_tag": "🟥"
#                                   }'


# # cargo run  qc '
# #                   {
# #                     "collection ": "c1_hiddenQueryCollection",
# #                     "queries": [{ "query": "rea" }],
# #                     "do_log_query_for_analytics": false
# #                   }'


# # ########################################################################################################################################################################################
# # ########################################################################################################################################################################################
# # ########################################################################################################################################################################################

# #   v0.0.16 cleanup, w servers

# #   find ~/repos/worp/worp-rust -type f -name '.DS_Store' -delete
# #   find ./.. -type f -name '.DS_Store' -delete
# #   export    RUST_BACKTRACE 1
# #   cargo run dc c1
# #   cargo run cc c1 '{
# #                       "max_docs_per_shard": 5,
# #                       "default_num_results_per_page": 10,
# #                       "fields": [
# #                         {
# #                           "name": "s1",
# #                           "sortThisGuy": true,
# #                           "doIndexPrefixes": true
# #                         },
# #                         {
# #                           "name": "s2",
# #                           "sortThisGuy": true,
# #                           "doIndexPrefixes": false
# #                         }
# #                       ]
# #                     }'

# #   cargo run ib c1 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt 200


# #   cargo run ic c1 '{ "s1":"111",  "s2":"111",   "f1":" w5 gregor mendel cheated",  "f2":"oh hi5 greg", "f3":"this is a test gregory!!" }'
# #   cargo run ic c1 '{ "s1":"211",  "s2":"211",   "f1":" w5 gregor the great",  "f2":"oh hi5", "f3":"this is a test" }'
# #   cargo run ic c1 '{ "s1":"311",  "s2":"311",   "f1":" w5 gregor is cool",  "f2":"oh hi5", "f3":"this is a test" }'
# #   cargo run ic c1 '{ "s1":"411",  "s2":"411",   "f1":" w5 gregor hey man",  "f2":"oh hi5", "f3":"this is a test" }'
# #   cargo run qc c1 '
# #                 {
# #                   "queries": [
# #                     {
# #                       "query": "11",
# #                       "fields": ["*"]
# #                     }
# #                   ]
# #                 }'
# # cargo run ferret 3131 "/Users/stuartrobinson/repos/worp/worp-rust/.tmp/ferret/collections"
# # http://localhost:3131/query/c1
# #                                     {
# #                                       "queries": [
# #                                         {
# #                                           "query": "gregory",
# #                                           "fields": ["*"]
# #                                         }
# #                                       ]
# #                                     }


# #   cargo run dc c2
# #   cargo run cc c2 '{
# #                       "max_docs_per_shard": 150000,
# #                       "default_num_results_per_page": 10,
# #                       "fields": [
# #                         {
# #                           "name": "s1",
# #                           "sortThisGuy": true,
# #                           "doIndexPrefixes": true
# #                         },
# #                         {
# #                           "name": "s2",
# #                           "sortThisGuy": true,
# #                           "doIndexPrefixes": false
# #                         }
# #                       ]
# #                     }'
# #   cargo run ib c2 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/100_rows_5kb_each.txt 100
# #   cargo run ib c2 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/100_rows_5kb_each.txt 100
# #   cargo run ib c2 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/100_rows_5kb_each.txt 100
# #   cargo run ib c2 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/100_rows_5kb_each.txt 100


# #   cargo run ic c1 '{ "s1":"5",  "s2":"5",   "f1":" w5  that is island tonight time",                "f2":"oh hi5"          }'
# #   cargo run ld c1
# #   cargo run lk "c1_\$_f2:1" 20000
# #   cargo run lk "c1_\$_f1:1" 20000





# #   cargo run qc c1 '
# # {
# # "num_results_per_page": 8,
# # "page_number": 1,
# # "sort_by": [
# #   {
# #     "name": "s1",
# #     "is_descending": false
# #   }
# # ],
# # "queries": [
# #   {
# #     "query": "gregor",
# #     "fields": ["f1"],
# #     "prefixLast": true,
# #     "collection": "c1"
# #   },
# #   {
# #     "query": "oh",
# #     "fields": ["f2"]
# #   }
# # ],
# # "worp_id": "fowiefiauwhe7fyawf"
# # }'








# #   cargo run qc c1 '{
# #                       "num_results_per_page":   "10",
# #                       "page_number":            "1",
# #                       "f1":                     "time is"
# #                     }'
# #   cargo run ib c1 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt 10
# #   cargo run ib c1 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt 500
# #   cargo run ib c1 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt 100
# #   cargo run ib c1 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt 100
# #   cargo run ib c1 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt 100
# #   cargo run ib c1 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt 10000
# #   cargo run ib c1 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt 10000
# #   cargo run ib c1 /Users/stuartrobinson/repos/worp/worp-rust/.resources/index_seeders/tinyBulk.txt 10000

# #   */

# cargo run qc '{
#   "pid": "w1",
#   "collection ": "c1_hiddenQueryCollection",
#   "queries": [{ "query": "ruin" }],
#   "do_highlights_tagged": true,
#   "highlight_pre_tag": "🟢",
#   "highlight_post_tag": "🟥"
# }'