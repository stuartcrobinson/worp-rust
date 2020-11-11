# dc = delete collection
./target/debug/worp-rust dc c1

# cc = create collection
./target/debug/worp-rust cc c1  '{
                    "max_docs_per_shard": 150000,
                    "default_num_results_per_page": 10,
                    "fields": [
                      {
                        "name": "num",
                        "sortThisGuy": true
                      }
                    ]
                  }'


# index documents into the collection.  ic = index in collection

./target/debug/worp-rust ic c1 '{"num": 1, "quote": "Reality continues to ruin my life." }'
./target/debug/worp-rust ic c1 '{"num": "2", "quote": "It\'s not denial. I\'m just selective about the reality I accept." }'
./target/debug/worp-rust ic c1 '{"num": 3, "quote": "Sometimes when I\'m talking, my words can\'t keep up with my thoughts. I wonder why we think faster than we speak. Probably so we can think twice." }'
./target/debug/worp-rust ic c1 '{"num": 4, "quote": "You know, Hobbes, some days even my lucky rocket ship underpants don\'t help." }'
./target/debug/worp-rust ic c1 '{"num": 5, "quote": "I\'m killing time while I wait for life to shower me with meaning and happiness." }'
./target/debug/worp-rust ic c1 '{"num": 6, "quote": "People think it must be fun to be a super genius, but they don\'t realize how hard it is to put up with all the idiots in the world." }'
./target/debug/worp-rust ic c1 '{"num": 7, "quote": "You can\'t just turn on creativity like a faucet. You have to be in the right mood. What mood is that? Last-minute panic." }'
./target/debug/worp-rust ic c1 '{"num": 8, "quote": "I wish I had more friends, but people are such jerks. If you can just get most people to leave you alone, you\'re doing good. If you can find even one person you really like, you\'re lucky. And if that person can also stand you, you\'re really lucky." }'
./target/debug/worp-rust ic c1 '{"num": 9, "quote": "The surest sign that intelligent life exists elsewhere in the universe is that it has never tried to contact us." }'
./target/debug/worp-rust ic c1 '{"num": 10, "quote": "There\'s never enough time to do all the nothing you want." }'
./target/debug/worp-rust ic c1 '{"num": 11, "quote": "You know what\'s weird? Day by day, nothing seems to change, but pretty soon...everything\'s different." }'
./target/debug/worp-rust ic c1 '{"num": 12, "quote": "Dad, how do soldiers killing each other solve the world\'s problems?" }'
./target/debug/worp-rust ic c1 '{"num": 13, "quote": "In my opinion, we don\'t devote nearly enough scientific research to finding a cure for jerks." }'
./target/debug/worp-rust ic c1 '{"num": 14, "quote": "As far as I\'m concerned, if something is so complicated that you can\'t explain it in 10 seconds, then it\'s probably not worth knowing anyway." }'
./target/debug/worp-rust ic c1 '{"num": 15, "quote": "You know, sometimes kids get bad grades in school because the class moves too slow for them. Einstein got D\'s in school. Well guess what, I get F\'s!!!" }'
./target/debug/worp-rust ic c1 '{"num": 16, "quote": "We\'re so busy watching out for what\'s just ahead of us that we don\'t take time to enjoy where we are." }'
./target/debug/worp-rust ic c1 '{"num": 17, "quote": "I\'m a misunderstood genius. What\'s misunderstood? Nobody thinks I\'m a genius." }'
./target/debug/worp-rust ic c1 '{"num": 18, "quote": "That\'s the difference between me and the rest of the world! Happiness isn\'t good enough for me! I demand euphoria!" }'
./target/debug/worp-rust ic c1 '{"num": 19, "quote": "Who was the guy who first looked at a cow and said \'I think I’ll drink whatever comes out of these when I squeeze ’em?" }'
./target/debug/worp-rust ic c1 '{"num": 20, "quote": "If people sat outside and looked at the stars each night, I\'ll bet they\'d live a lot differently. " }'
./target/debug/worp-rust ic c1 '{"num": 21, "quote": "When life gives you lemons, chunk it right back." }'
./target/debug/worp-rust ic c1 '{"num": 22, "quote": "Weekends don\'t count unless you spend them doing something completely pointless." }'
./target/debug/worp-rust ic c1 '{"num": 23, "quote": "I go to school, but I never learn what I want to know." }'
./target/debug/worp-rust ic c1 '{"num": 24, "quote": "I find my life is a lot easier the lower I keep my expectations." }'
./target/debug/worp-rust ic c1 '{"num": 25, "quote": "I\'m learning skills I will use for the rest of my life by doing homework...procrastinating and negotiation." }'
./target/debug/worp-rust ic c1 '{"num": 26, "quote": "I think hiccup cures were really invented for the amusement of the patient\'s friends." }'
./target/debug/worp-rust ic c1 '{"num": 26, "quote": "I think hiccup one two three." }'

# sanity check -- qc = query collection

./target/debug/worp-rust qc '
                  {
                    "collection_name": "c1",
                    "do_highlight_map": true,
                    "do_highlight_tagged": true,
                    "highlight_pre_tag": "✅",
                    "highlight_post_tag": "🛑",
                    "min_highlight_context": 20,
                    "max_total_snippets_length": 500,
                    "queries": [
                      {
                        "query": "reality",
                        "fields": ["*"]
                      }
                    ]
                  }'

./target/debug/worp-rust ferret 3131