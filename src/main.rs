#![allow(warnings)]
#![allow(non_camel_case_types)]
#![allow(while_true)]                 // 'while true' is faster than 'loop'
#![allow(non_snake_case)]
#![allow(unused_must_use)]
mod multiIiIntersection;
mod tokenizing;
#[macro_use]
extern crate lazy_static;
mod stateManagement;
mod worpError;
mod h;
use h::*;
mod dirStuff;
mod global;
use global::*;

extern crate rayon;
mod rocksdb_tools;
use rocksdb_tools::removeRocksdbLogsOld;
use crate::tokenizing::tokenize;
use crate::tokenizing::splitKeep_by_everything_except_apostrophes;
use crate::stateManagement::*;
use crate::rocksdb_tools::*;

mod ferret;
use ferret::*;
extern crate queues;
extern crate argon2;


fn main() {
  // std::process::exit(1);

  println!("hello");
  let args = getArgs();
  println!("args: {:?}",args);

  if args.len() == 0 {panic!("command me bro");}

  if args[0] != "ferret" &&  args[0] != "dc" {
    // why?
    resetStaticMaps_clxnsRoot(SQUIRREL_COLLECTIONS_ROOT, false);
  }
  println!("----------------------------------------------------------");

  if args[0] == "removeRocksdbLogsOld "       { removeRocksdbLogsOld(".");    }

  /* tokenizing testing */
  if args[0] == "t"                          { println!("{:?}", tokenize(&args[1], vec![1], false)); }
  if args[0] == "sk"                         { println!("{:?}", splitKeep_by_everything_except_apostrophes(&args[1])); }
  if args[0] == "gp"                         { println!("{:?}", getPrefixesForToken(&args[1])); }
  if args[0] == "findCookedToks"             { findCookedToks(&args[1]);    }

  if args[0] == "ii"                         { displayInvind(&args[1], &args[2], &args[3], &args[4], args[5].parse().unwrap(), &args[6]);    }
  // if args[0] == "cnd"                        { println!("collection: {}, numDocs: {}", &args[1], countNumDocsForCollection(&args[1]));    }
  if args[0] == "ld"                         { println!("{:?}", listDocs(SQUIRREL_COLLECTIONS_ROOT, &args[1],  &args[2], DEFAULT_VINTAGE));                           }
  if args[0] == "lk"                         { println!("{:?}", listNKeysForRocksRoot(&args[1], &args[2]));                                  }
  if args[0] == "lr"                         { for p in getRocksDbPaths(){ println!("{}", p); }                                                                 }
  if args[0] == "map"                        { displayMap();                               }
  if args[0] == "schema"                     { println!("{}", serde_json::to_string_pretty(&readSchema(SQUIRREL_COLLECTIONS_ROOT, &args[1], &args[2], DEFAULT_VINTAGE, false)).unwrap());                  }
  if args[0] == "gv"                         { println!("{:?}", getValueFromRocksDb(&args[1], &args[2]));        }
  if args[0] == "rk"                         { println!("{:?}", readKey(&args[1], &args[2]));        }
  if args[0] == "eq"                         { println!("{}", expandQuery_fromStr(&args[1], SQUIRREL_COLLECTIONS_ROOT, DEFAULT_VINTAGE, false).unwrap());          }
  if args[0] == "destroy_everything"         { destroy_everything();                                                                   }

  if args[0] == "sort"                       { println!("{:?}", setFieldAsSorter(&args[1], &args[2], &args[3])); }
  if args[0] == "unsort"                     { println!("{:?}", unsetFieldAsSorter(&args[1], &args[2], &args[3])); }
  if args[0] == "time"                       { println!("{:?}", setFieldAsTimegrapher(&args[1], &args[2], &args[3])); }
  if args[0] == "untime"                     { println!("{:?}", unsetFieldAsTimegrapher(&args[1], &args[2], &args[3])); }

  if args[0] == "unindexPrefixesOnly"        { println!("{:?}", unindexPrefixesOnly(&args[1], &args[2], &args[3])); }
  if args[0] == "indexWithPrefixes"          { println!("{:?}", indexWithPrefixes(&args[1], &args[2], &args[3])); }

  if args[0] == "unindexFieldInvinds"        { println!("{:?}", unindexFieldInvinds(&args[1], &args[2], &args[3])); }
  if args[0] == "indexTokensWithoutPrefixes" { println!("{:?}", indexTokensWithoutPrefixes(&args[1], &args[2], &args[3])); }

  if args[0] == "mfa"                        { println!("{:?}", modifyFieldAttribute(&args[1], &args[2], &args[3], &args[4], args[5].parse().unwrap())); }

  if args[0] == "size"                       { println!("{:?}", getCollectionDataSize(SQUIRREL_COLLECTIONS_ROOT, &args[1], &args[2], DEFAULT_VINTAGE)); }
  if args[0] == "sortbys"                    { println!("{:?}", getSortBys(SQUIRREL_COLLECTIONS_ROOT, &args[1], &args[2], DEFAULT_VINTAGE, false)); }


  if args[0] == "cc"                         { createCollection(&args[1]); }
  if args[0] == "dc"                         { deleteCollection(&args[1], &args[2]); }
  if args[0] == "ic"                         { indexDocInCollection(&args[1], &args[2], &args[3]); }
  if args[0] == "dd"                         { deleteDocInCollection(&args[1], &args[2], args[3].parse().unwrap()); }
  if args[0] == "ib"                         { indexBulk(&args[1], &args[2], &args[3], args[4].parse().unwrap()); }
  if args[0] == "ibs"                        { indexBulkSingleDoc(&args[1], &args[2], &args[3]); }
  if args[0] == "qc"                         { queryClxLocal(&args[1], true); }
  if args[0] == "qcp"                        { queryClxLocal_parallel(&args[1], true); }

  if args[0] == "pnv"                        { publishNewVintage(&args[1], &args[2]); }    //put the ferret's clxnsRoot in .tmp/ferret/collections for now // /Users/stuartrobinson/repos/worp/worp-rust/.tmp

  if args[0] == "ferret"                     { println!("{:?}", startFerret(args[1].parse().unwrap()));    }                     //, &args[2]
  if args[0] == "hash"                       { println!("{:?}", hashPasswordArgon2(&args[1]) );               }                     //, &args[2]


  //see commands.sh commands.fish
}


#[cfg(test)]
mod tests {
  use crate::global::createCollection;
  use crate::global::indexDocInCollection;
  use crate::global::queryClxLocal;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn do_stuff() -> Result<(), Box<dyn std::error::Error>> {

    let pid = "w1";
    let collection = "c1";

    let ccBody = r#"{
      "pid": "w1",
      "collection": "c1",
      "do_log_queries": true,
      "global_secret": "my secret globalSecret"
    }"#;
    createCollection(ccBody)?;

    indexDocInCollection(pid, collection, r#"{"num": 1, "quote": "Reality continues to ruin my life." }"#);

    let queryBody = r#"{
      "pid": "w1",
      "collection": "c1",
      "queries": [{ "query": "rea" }],
      "do_highlights_tagged": true,
      "highlight_pre_tag": "🟢",
      "highlight_post_tag": "🟥"
    }"#;

    queryClxLocal(queryBody, false);

    Ok(())
  }


}