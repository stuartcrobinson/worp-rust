#!/bin/bash

#############################################################################################################################
########################################################## v0.0.24 ##########################################################
#############################################################################################################################

WORP_BIN_LOCATION=./target/debug/worp-rust

WORP_CMD=$WORP_BIN_LOCATION
# WORP_CMD="cargo run"

#############################################################################################################################


find ./.. -type f -name '.DS_Store' -delete
find ./.. -type f -name 'LOG.old.*' -delete

if [ -d "target/debug" ]
then
    export RUST_BACKTRACE = 1
    cargo build
fi

$WORP_CMD  dc c1
$WORP_CMD  dc c1_hiddenQueryCollection

$WORP_CMD  dc w1 c1
$WORP_CMD  dc w1 c1_hiddenQueryCollection

$WORP_CMD  cc '{
                    "pid": "w1",
                    "collection": "c1",
                    "do_log_queries": true,
                    "global_secret" "my secret globalSecret"
                  }'

$WORP_CMD ic w1 c454 '{"num": 1, "quote": "Reality continues to ruin my life." }'
$WORP_CMD ic w1 c1 '{"num": "2", "quote": "It'\''s not denial. I'\''m just selective about the reality I accept." }'
$WORP_CMD ic w1 c1 '{"num": 4, "quote": "You know, Hobbes, some days even my lucky rocket ship underpants don'\''t help." }'
$WORP_CMD ic w1 c1 '{"num": 5, "quote": "I'\''m killing time while I wait for life to shower me with meaning and happiness." }'
$WORP_CMD ic w1 c1_hiddenQueryCollection '{"_timestamp": 1603101899205, "query": "ruin" }'
$WORP_CMD ic w1 c1_hiddenQueryCollection '{"_timestamp": 1603202899205, "query": "ruin" }'
$WORP_CMD ic w1 c1_hiddenQueryCollection '{"_timestamp": 1603202899205, "query": "ruin" }'
$WORP_CMD ic w1 c1_hiddenQueryCollection '{"_timestamp": 1603303899205, "query": "ruin" }'
$WORP_CMD ic w1 c1_hiddenQueryCollection '{"_timestamp": 1603404899205, "query": "ruin" }'
$WORP_CMD ic w1 c1_hiddenQueryCollection '{"_timestamp": 1603404899205, "query": "ruin" }'
$WORP_CMD ic w1 c1_hiddenQueryCollection '{"_timestamp": 1603404899205, "query": "ruin" }'
$WORP_CMD ic w1 c1_hiddenQueryCollection '{"_timestamp": 1603707899205, "query": "ruin" }'
$WORP_CMD ib w1 c1 .resources/index_seeders/tinyBulk.txt 10

$WORP_CMD qc '{
                  "pid": "w1",
                  "collection": "c1",
                  "queries": [{ "query": "rea" }],
                  "do_highlights_tagged": true,
                  "highlight_pre_tag": "🟢",
                  "highlight_post_tag": "🟥"
                }'

$WORP_CMD ld w1 c1
$WORP_CMD ld w1 c1_hiddenQueryCollection

# $WORP_CMD ferret 3131

