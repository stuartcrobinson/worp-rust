extern crate walkdir;
use walkdir::WalkDir;
use crate::stateManagement::*;
use crate::dirStuff::*;
use crate::h::st;
use crate::h::prettyStringHashMap;
use crate::h::getCurrYearMonthString;
// use crate::global::displayMap;
use std::collections::HashSet;
use std::collections::HashMap;
// use std::collections::BTreeMap;
use std::iter::FromIterator;
// use rocksdb::prelude::*;
use crate::getDocId;
use rocksdb::{prelude::*, IteratorMode};
use std::error::Error;
use crate::worpError::werr;


//TODO use this!! "old" files are building up
pub fn removeRocksdbLogsOld(root: &str) -> Result<(), walkdir::Error> {
    // https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
    for entry in WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok()) {
        let f_name_str = &entry.file_name().to_string_lossy().to_string();

        //this isn't the full path
        if f_name_str.contains("LOG.old."){
            std::fs::remove_file(entry.path().display().to_string());  //but this is
        }
    }
    Ok(())
}

pub fn openDb(rocksRoot: &str) -> rocksdb::DB {
  // let map = M_ROCKSROOT_DB.lock().unwrap();

  //whats' the point of using this map here?  why not just embed the rocksdb file path into the "shardId" (now rocksId)
  //should we use rocksId at all?  what about rocksPath? that feels like it would be more sane ...

  // println!("openDb(shardId: &str) shardId: {}", shardId);
  // let rocksRoot: String = map.get(shardId).unwrap().rocksRoot.to_string();

  match rocksdb::DB::open_default(&rocksRoot){
    Ok(_db) => {return _db;},
    Err(e)   => {
      println!("openDb: retrying afer error: {}", e);
      match rocksdb::DB::open_default(&rocksRoot) {       // this is to avoid weird Corruption .... log file found ... error.
        Ok(_db) => {return _db;},
        Err(e)   => {
          panic!("wtf error: {}", e);
        }
      }
    }
  }
}

pub fn writeString(rocksRoot: &str, key: &str, value: &str) {
  // query mode is always false for writing!!
  let isQueryMode = false;
  unsafe {
    if QUERY_MODE && isQueryMode {
      let map = M_ROCKSROOT_DB.read().unwrap();
      let db = map.get(rocksRoot).unwrap().db.as_ref().unwrap();
      db.put(key, value).unwrap();
    } else {
      let db = &openDb(rocksRoot);
      db.put(key, value).unwrap();
    }
  }
}

// pub fn writeBytes(rocksRoot: &str, key: &str, value: Vec<u8>) {
//   unsafe {
//     if QUERY_MODE {
//       let map = M_ROCKSROOT_DB.read().unwrap();
//       let db = map.get(rocksRoot).unwrap().db.as_ref().unwrap();
//       db.put(key, value).unwrap();
//     } else {
//       let db = &openDb(rocksRoot);
//       db.put(key, value).unwrap();
//     }
//   }
// }

/* should combine these using generics or whatever it's called */

pub fn writeString_(db: &rocksdb::DB, key: &str, value: &str)     {
  db.put(key, value).unwrap();
}

pub fn  writeBytes_(db: &rocksdb::DB, key: &str, value: &Vec<u8>) {
  db.put(key, value).unwrap();
}

pub fn readKey(rocksRoot: &str, key: &str) -> Vec<u8> {
  // println!("rocksRoot:  {}", rocksRoot);
  unsafe {
    if QUERY_MODE {
      let map = M_ROCKSROOT_DB.read().unwrap();
      let db = map.get(rocksRoot);
      match db {
        Some(x) => readKey_(x.db.as_ref().unwrap(), key),
        None    => Vec::new(),
      }
    } else {
      let db = &openDb(rocksRoot);
      readKey_(db, key)                                             // it has to be like this even tho so WET not DRY
    }
  }
}

pub fn readKey_qm(rocksRoot: &str, key: &str, isQueryMode: bool) -> Vec<u8> {
  // println!("rocksRoot:  {}", rocksRoot);
  unsafe {
    if QUERY_MODE && isQueryMode{
      let map = M_ROCKSROOT_DB.read().unwrap();
      let db = map.get(rocksRoot);
      match db {
        Some(x) => readKey_(x.db.as_ref().unwrap(), key),
        None    => Vec::new(),
      }
    } else {
      let db = &openDb(rocksRoot);
      readKey_(db, key)                                             // it has to be like this even tho so WET not DRY
    }
  }
}

pub fn readKey_(db: &rocksdb::DB, key: &str) -> Vec<u8> {
  // TODO shouldn't this fail if key not found .... ???????????? idk
  match db.get(key) {
    Ok(Some(value)) =>  { value.to_vec() },
    Ok(None) =>         { Vec::new() },
    Err(e) => panic!("readKey operational problem encountered: {}", e),
  }
}

pub fn readString_(db: &rocksdb::DB, key: &str) -> String {
  let value = readKey_(db, key);
  String::from_utf8(value.to_vec()).unwrap()
}

pub fn readString(rocksRoot: &str, key: &str) -> String {
  let value = readKey(rocksRoot, key);
  String::from_utf8(value.to_vec()).unwrap()
}

pub fn readString_qm(rocksRoot: &str, key: &str, isQueryMode: bool) -> String {
  // let value = readKey(rocksRoot, key);
  let value = readKey_qm(rocksRoot, key, isQueryMode);
  String::from_utf8(value.to_vec()).unwrap()
}


pub fn assertKeyIsUnused_(key: &str, db: &rocksdb::DB) {
  match db.get(&key) {
      Ok(Some(_value)) =>  { panic!(format!("assertKeyIsUnused_ FAILED! key was found! {}", &key)) },
      Ok(None) =>         {  },
      Err(e) => panic!("assertKeyIsUnused_ operational problem encountered: {}", e),
  }
}

pub fn listDocs(clxnsRoot: &str, pid: &str, collection: &str, vintage: &str) {

  let rocksRoot = &getDir_rocksRoot_docs(collection, clxnsRoot, pid, vintage);
  // getObjectsAsStrings_(db: &rocksdb::DB)

  unsafe {
    if QUERY_MODE {
      let map = M_ROCKSROOT_DB.read().unwrap();
      let db = map.get(rocksRoot).unwrap().db.as_ref().unwrap();
      listObjectsAsStrings_(&db)
    } else {
      let db = &openDb(rocksRoot);
      listObjectsAsStrings_(&db)
    }
  }
}


pub fn listObjectsAsStrings_(db: &rocksdb::DB)  {
  println!("in listDocs: {:?}", &db);
  let mut count = 0;
  for (key, value) in db.iterator(IteratorMode::Start) {
    count += 1;
    if count > 1000{
      break;
    }
    println!("{}) {}: {:?}", count, String::from_utf8(key.to_vec()).unwrap(), String::from_utf8(value.to_vec()).unwrap());                             //trash this
  }
}

pub fn getAllObjectsAsStrings(rocksRoot: &str, isQueryMode: bool) -> HashMap<String, serde_json::Value> {
  println!("in getAllObjectsAsStrings, rocksRoot: {:?}", &rocksRoot);

  unsafe {
    if QUERY_MODE && isQueryMode {
      // displayMap();
      let map = M_ROCKSROOT_DB.read().unwrap();
      let db = map.get(rocksRoot).unwrap().db.as_ref().unwrap();
      getAllObjectsAsStrings_(&db)
    } else {
      let db = &openDb(rocksRoot);
      getAllObjectsAsStrings_(&db)
    }
  }
}

pub fn getAllObjectsAsStrings_(db: &rocksdb::DB) -> HashMap<String, serde_json::Value>  {
  // println!("in getAllObjectsAsStrings_: {:?}", &db);
  let mut allObjects: HashMap<String,  serde_json::Value> = HashMap::new();
  let mut count = 0;
  for (key, value) in db.iterator(IteratorMode::Start) {
    count += 1;
    if count > 1000{
      break;
    }
    let keyStr = String::from_utf8(key.to_vec()).unwrap();
    let valueStr = String::from_utf8(value.to_vec()).unwrap();
    // println!("in getAllObjectsAsStrings_: valueStr: {:?}", valueStr);


    match serde_json::from_str(&valueStr) {
      Ok(v)   => { allObjects.insert(keyStr, v); },
      Err(_e)  => { allObjects.insert(keyStr, serde_json::json!(valueStr)); }
    };

    // let valueValue: serde_json::Value = serde_json::from_str(&valueStr).unwrap();
    // allObjects.insert(keyStr, valueValue);
  }
  // println!("in getAllObjectsAsStrings_: allObjects {:?}", &allObjects);

  allObjects
}

fn listKeys(db: &rocksdb::DB, n: usize) {
  println!("in listKeys: {:?} {:?}", &db, n);

  let mut count = 0;
  for (key, _value) in db.iterator(IteratorMode::Start) {
    count += 1;
    if count > n{
      break;
    }
    println!("{}", String::from_utf8(key.to_vec()).unwrap());
  }
}


pub fn listNKeysForRocksRoot(rocksRoot: &str, nStr: &str) {
  let n: usize = nStr.parse().unwrap();
  let db = &openDb(rocksRoot);
  return listKeys(db, n);
}

pub fn getValueFromRocksDb(rocksRoot: &str, key: &str) -> (String, String, Vec<u8>){
  let bytes = readKey(rocksRoot, key);
  // (st(rocksRoot), st(key), String::from_utf8(value).unwrap())
  (st(rocksRoot), st(key), bytes)
}


pub fn countObjects(db: &rocksdb::DB) -> usize{
  db.iterator(IteratorMode::Start).count()                                          //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.count
}

// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////
// ////////////////////////////////////////////////////////////////////////////////////////////////////  reading/writing specific things  ///////////////////////////////////


/// this can assume default params cos only squirrel is writing. ferret just reads
pub fn writeSchema(pid: &str, collectionName: &str, schema: &WorpdriveSchema) {
  println!("writeSchema: pid c schema: {} {} {:?}", pid, collectionName, schema);
  let schemaStr = serde_json::to_string(schema).unwrap();
  println!("writeSchema: pid c schStr: {} {} {}", pid, collectionName, schemaStr);
  let rocksRoot = &getDir_rocksRoot_meta(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE);
  writeString(rocksRoot, KEY_SCHEMA, &schemaStr);
}

pub fn writeSchema_(schema: &WorpdriveSchema, db: &rocksdb::DB) {
  let schemaStr = serde_json::to_string(schema).unwrap();
  writeString_(db, KEY_SCHEMA, &schemaStr);
}


pub fn readSchema(clxnRoot: &str, pid: &str, collection: &str, vintage: &str, isQueryMode: bool) -> WorpdriveSchema {
  let rocksRoot = &getDir_rocksRoot_meta(collection, clxnRoot, pid, vintage);
  let schemaStr = readString_qm(rocksRoot, KEY_SCHEMA, isQueryMode);
  parseSchemaStr(&schemaStr)
}


pub fn readSchema_(metaDb: &rocksdb::DB) -> WorpdriveSchema {
  let schemaStr = readString_(metaDb, KEY_SCHEMA);
  parseSchemaStr(&schemaStr)
}

pub fn getAllMetaJsonPrettyStr(clxnsRoot: &str, pid: &str, collectionName: &str, vintage: &str, isQueryMode: bool) -> Result<String, Box<dyn Error>> {
  let rocksroot = &getDir_rocksRoot_meta(collectionName, clxnsRoot, pid, vintage);
  if !exists(rocksroot) {
    return Err(werr(&format!("rocksroot DNE: {}", rocksroot)));
  }
  let allObjects: HashMap<String,  serde_json::Value> = getAllObjectsAsStrings(rocksroot, isQueryMode);
  let asString = prettyStringHashMap(allObjects);
  Ok(asString)
}

pub fn getAllDocsJsonPrettyStr(clxnsRoot: &str, pid: &str, collectionName: &str, vintage: &str, isQueryMode: bool) -> String {
  let rocksroot = &getDir_rocksRoot_docs(collectionName, clxnsRoot, pid, vintage);
  let allObjects: HashMap<String,  serde_json::Value> = getAllObjectsAsStrings(rocksroot, isQueryMode);
  let asString = prettyStringHashMap(allObjects);
  asString
}


/// just writes marco polo to the meta rdb  (should it be in all rdbs????? no.)
pub fn writeMarcoPolo(pid: &str, collectionName: &str){
  let rocksRoot = &getDir_rocksRoot_meta(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE);
  writeString(rocksRoot, KEY_MARCO, KEY_POLO);
  // assert_eq!(KEY_POLO, readString(rocksRoot, KEY_MARCO));
}

pub fn readSovabydid(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str, shardName: &str, fieldName: &str) -> Vec<f32>{
  let sovabydidRocksRoot = &getDir_rocksRoot_sovabydid(collection, clxnsRoot, pid, vintage, shardName);
  let key = fieldName;
  let bytes = &readKey(sovabydidRocksRoot, key);
  // println!("read bytes in readSovabydid: {:?}", bytes);
  // println!("sovabydidRocksRoot: {}, key {}", sovabydidRocksRoot, key);
  if bytes.len() > 0 {
    bincode::deserialize(bytes).unwrap()
  } else {
    Vec::new()
  }
}

pub fn readTivabydid(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str, shardName: &str, fieldName: &str) -> Vec<u64>{
  let tivabydidRocksRoot = &getDir_rocksRoot_tivabydid(collection, clxnsRoot, pid, vintage, shardName);
  let key = fieldName;
  let bytes = &readKey(tivabydidRocksRoot, key);
  if bytes.len() > 0 {
    bincode::deserialize(bytes).unwrap()
  } else {
    Vec::new()
  }
}

pub fn readSovabydid_(fieldName: &str, db: &rocksdb::DB) -> Vec<f32>{
  let key = fieldName;
  let bytes = &readKey_(db, key);
  if bytes.len() > 0 {
    bincode::deserialize(bytes).unwrap()
  } else {
    Vec::new()
  }
}

pub fn readTivabydid_(fieldName: &str, db: &rocksdb::DB) -> Vec<u64>{
  let key = fieldName;
  let bytes = &readKey_(db, key);
  if bytes.len() > 0 {
    bincode::deserialize(bytes).unwrap()
  } else {
    Vec::new()
  }
}

pub fn writeSovabydid(sovabydid: &Vec<f32>, fieldName: &str, collection: &str, pid: &str, shardName: &str) {
  let rocksRoot = getDir_rocksRoot_sovabydid(collection, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, shardName);
  let db = &openDb(&rocksRoot);
  writeSovabydid_(sovabydid, fieldName, db);
}

pub fn deleteSovabydid(fieldName: &str, collection: &str, pid: &str, shardName: &str) {
  let rocksRoot = getDir_rocksRoot_sovabydid(collection, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, shardName);
  let db = &openDb(&rocksRoot);
  let key = fieldName;
  db.delete(key);
}

pub fn writeSovabydid_(sovabydid: &Vec<f32>, fieldName: &str, sovabydidsShardRdb: &rocksdb::DB) {
  let key = fieldName;
  let value: Vec<u8> = bincode::serialize(&sovabydid).unwrap();
  writeBytes_(sovabydidsShardRdb, key, &value);
}

pub fn writeTivabydid(tivabydid: &Vec<u64>, fieldName: &str, collection: &str, pid: &str, shardName: &str) {
  let rocksRoot = &getDir_rocksRoot_tivabydid(collection, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, shardName);
  createDirMaybe(rocksRoot);
  let db = &openDb(rocksRoot);
  writeTivabydid_(tivabydid, fieldName, db);
}

// TODO dont delete - dont we need this?
// pub fn deleteTivabydid(fieldName: &str, collection: &str, pid: &str, shardName: &str) {
//   let rocksRoot = &getDir_rocksRoot_tivabydid(collection, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, shardName);
//   let db = &openDb(rocksRoot);
//   let key = fieldName;
//   db.delete(key);
// }

pub fn writeTivabydid_(tivabydid: &Vec<u64>, fieldName: &str, tivabydidsShardRdb: &rocksdb::DB) {
  let key = fieldName;
  let value: Vec<u8> = bincode::serialize(&tivabydid).unwrap();
  writeBytes_(tivabydidsShardRdb, key, &value);
}

pub fn readDoc(clxnsRoot: &str, pid: &str, collection: &str, vintage: &str, docId: usize) -> String {
  let key = &format!("{}", docId);
  let rr = &getDir_rocksRoot_docs(collection, clxnsRoot, pid, vintage);
  readString(rr, key)
}

pub fn readDoc_(db: &rocksdb::DB, docId: usize) -> String {
  let key = &format!("{}", docId);
  // let rr = &getDir_rocksRoot_docs(collection, clxnsRoot, pid, vintage);
  readString_(db, key)
}

pub fn unwriteDoc_(db: &rocksdb::DB, docId: usize) {
  let key = &format!("{}", docId);
  db.delete(key);
}

pub fn writeDoc_(docId: usize, doc: &str, docsDb: &rocksdb::DB, doassertKeyIsUnused_: bool) {
  let key = format!("{}", docId);
  if doassertKeyIsUnused_ {
    assertKeyIsUnused_(&key, docsDb);
  }
  writeString_(docsDb, &key, doc);
}


pub fn writeNumDocs_(numDocs: usize,  metaDb: &rocksdb::DB) {
  metaDb.put(KEY_NUM_DOCS, format!("{}", numDocs)).unwrap();
}

pub fn writeNumDocs(pid: &str, collectionName: &str, numDocs: usize) {
  let rocksRoot = &getDir_rocksRoot_meta(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE);
  writeString(rocksRoot, KEY_NUM_DOCS, &numDocs.to_string());
}

pub fn readNumDocs_(metaDb: &rocksdb::DB) -> usize{
  readString_(metaDb, KEY_NUM_DOCS).parse().unwrap()
}

pub fn readNumDocs(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str) -> usize {
  let metaRocksRoot = &getDir_rocksRoot_meta(collection, clxnsRoot, pid, vintage);
  readString(metaRocksRoot, KEY_NUM_DOCS).parse().unwrap()
}


pub fn readInvind_(token: &str, isPrefix: bool, db: &rocksdb::DB) -> Vec<u8>{
  let key = if isPrefix { format!("_$_pre_{}", token) } else { format!("{}", token) };
  // println!("key: {}", key);
  readKey_(db, &key)
}

// pub fn readInvindVecs(queryTokens: &Vec<String>, indexName: &str, shardName: &str, doMergeLastTokenWithItsPrefixInvind: bool, vintage: &str) -> Vec<Vec<u8>> {
//   let key = if isPrefix { format!("_$_pre_{}", token) } else { format!("{}", token) };
//   readKey_(db, &key)
// }

/// read inverted index binary arrays for given tokens.  each sorted by increasing docIds
pub fn readInvindBas(tokens: &Vec<String>, fieldName: &str, shardName: &str, doMergeLastTokenWithItsPrefixInvind: bool, clxnsRoot: &str, pid: &str, collection: &str, vintage: &str) -> Vec<Vec<u8>>{
  let rocksRoot = &getDir_rocksRoot_invind(collection, clxnsRoot, pid, vintage, shardName, fieldName);
  let mut bas: Vec<Vec<u8>> = Vec::new(); //bas for "binary arrays"

  for token in tokens {
    let invind = readInvindVec(token, false, rocksRoot);                          // this happens in query mode so accessing the db should be super quick.  time cost is just unlocking the map
    // println!("invind for {} {} {:?} -- {:?}", token, subShard, db, invind);
    bas.push(invind);
  }
  if doMergeLastTokenWithItsPrefixInvind {
    let basLen = &bas.len();
    let mut lastInvind = bas[basLen - 1].to_vec();
    let lastToken = &tokens[tokens.len()-1];
    // println!("lastToken: {}", lastToken);
    let lastTokenPrefixInvind = readInvindVec(&lastToken, true, rocksRoot);

    // println!("lastTokenPrefixInvind: {:?}", lastTokenPrefixInvind);
    // println!("lastInvind len before extend with lastToken: {:?}", lastInvind.len());
    // lastInvind.extend(lastTokenPrefixInvind);

    lastInvind = mergeInvinds(lastInvind, lastTokenPrefixInvind);

    // println!("lastInvind len after extend with lastToken: {:?}", lastInvind.len());
    // println!("basLen: {:?}", basLen);

    bas[basLen - 1] = lastInvind.to_vec();
  }
  bas
}

pub fn incrementBillingMetricReads(pid: &str, collectionName: &str) {
  let rocksRoot = getDir_rocksRoot_meta(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE);
  let key = "billingMetrics";
  let db = &openDb(&rocksRoot);
  let mut billingMetrics = readString_(db, key);
  let mut bmmap : HashMap<&str, BillingMetricsMonthObj> =  match serde_json::from_str(&billingMetrics) {
    Ok(map) => map,
    Err(_e) => HashMap::new()
  };
  let monthKey = &getCurrYearMonthString();
  bmmap.entry(monthKey).or_insert(BillingMetricsMonthObj {reads: 0, writes: 0}).reads += 1;
  writeString_(db, key, &serde_json::to_string_pretty(&bmmap).unwrap());
}

pub fn incrementBillingMetricWrites(pid: &str, collectionName: &str) {
  let rocksRoot = getDir_rocksRoot_meta(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE);
  let key = "billingMetrics";
  let db = &openDb(&rocksRoot);
  let mut billingMetrics = readString_(db, key);
  let mut bmmap : HashMap<&str, BillingMetricsMonthObj> =  match serde_json::from_str(&billingMetrics) {
    Ok(map) => map,
    Err(_e) => HashMap::new()
  };
  let monthKey = &getCurrYearMonthString();
  bmmap.entry(monthKey).or_insert(BillingMetricsMonthObj {reads: 0, writes: 0}).writes += 1;
  writeString_(db, key, &serde_json::to_string_pretty(&bmmap).unwrap());
}

pub fn getDocIds(clxnsRoot: &str, pid: &str, collection: &str, vintage: &str, isQueryMode: bool) -> Vec<usize> {
  let rocksRoot = &getDir_rocksRoot_docs(collection, clxnsRoot, pid, vintage);

  unsafe {
    if QUERY_MODE && isQueryMode {
      let map = M_ROCKSROOT_DB.read().unwrap();
      let db = map.get(rocksRoot).unwrap().db.as_ref().unwrap();
      getKeysAsIntsSortedIncreasing_(&db)
    } else {
      let db = &openDb(rocksRoot);
      getKeysAsIntsSortedIncreasing_(&db)
    }
  }
}

pub fn getKeysAsIntsSortedIncreasing_(db: &rocksdb::DB) -> Vec<usize> {
  let mut keys: Vec<usize> = Vec::new();
  for (key, _value) in db.iterator(IteratorMode::Start) {
    let keyString = String::from_utf8(key.to_vec()).unwrap();
    keys.push(keyString.parse().unwrap());
  }
  keys.sort();
  keys
}

pub fn deleteAllPrefixes(db: &rocksdb::DB) -> usize {
  let mut count = 0;
  for (key, _value) in db.iterator(IteratorMode::Start) {
    let keyString = String::from_utf8(key.to_vec()).unwrap();
    if keyString.contains("_$_pre_"){
      db.delete(keyString);
      count += 1;
    }
  }
  count
}


/// why is this here ...
/// a and b are sorted by docId \
/// merge them, keeping their ordering. randomly dropping a docId if the other has it. (this is bad, it should merge their relavences or soemthing)
pub fn mergeInvinds(a: Vec<u8>, b: Vec<u8>) -> Vec<u8>{
  let mut merged: Vec<u8> = vec![0; a.len() + b.len()];                     // do this instead of incremental pushing.  big as possible, then shrink down to return

  let mut ai = 0;
  let mut bi = 0;
  let mut mi = 0;


  while mi < merged.len() && ai < a.len() && bi < b.len() {
    let aDocId = getDocId!(a, ai);
    let bDocId = getDocId!(b, bi);

    if aDocId < bDocId {
      merged[mi + 0] = a[ai + 0];
      merged[mi + 1] = a[ai + 1];
      merged[mi + 2] = a[ai + 2];
      merged[mi + 3] = a[ai + 3];
      merged[mi + 4] = a[ai + 4];
      ai += 5;
      mi += 5;
    }
    else if  bDocId < aDocId {
      merged[mi + 0] = b[bi + 0];
      merged[mi + 1] = b[bi + 1];
      merged[mi + 2] = b[bi + 2];
      merged[mi + 3] = b[bi + 3];
      merged[mi + 4] = b[bi + 4];
      bi += 5;
      mi += 5;
    }
    else if  bDocId == aDocId {         //TODO this just picks b randomly.  but it should merge their relavences or something. or at least pick the bigger one
      merged[mi + 0] = b[bi + 0];
      merged[mi + 1] = b[bi + 1];
      merged[mi + 2] = b[bi + 2];
      merged[mi + 3] = b[bi + 3];
      merged[mi + 4] = b[bi + 4];
      ai += 5;
      bi += 5;
      mi += 5;
    }
  }
  if ai < a.len() && getDocId!(a, ai) > 0 {
    while ai < a.len() && mi < merged.len() {
      merged[mi + 0] = a[ai + 0];
      merged[mi + 1] = a[ai + 1];
      merged[mi + 2] = a[ai + 2];
      merged[mi + 3] = a[ai + 3];
      merged[mi + 4] = a[ai + 4];
      ai += 5;
      mi += 5;
    }
  } else if bi < b.len() && getDocId!(b, bi) > 0{
    while bi < b.len()  && mi < merged.len() {
      merged[mi + 0] = b[bi + 0];
      merged[mi + 1] = b[bi + 1];
      merged[mi + 2] = b[bi + 2];
      merged[mi + 3] = b[bi + 3];
      merged[mi + 4] = b[bi + 4];
      bi += 5;
      mi += 5;
    }
  }

  merged[0..mi].to_vec()                   //cut the fat
}

pub fn readInvindVec(token: &str, isPrefix: bool, rocksRoot: &str) -> Vec<u8>{
  let key = if isPrefix { format!("_$_pre_{}", token) } else { format!("{}", token) };
  readKey(rocksRoot, &key)
}


pub fn writeInvind_(token: &str, isPrefix: bool, ii: &Vec<u8>, db: &rocksdb::DB) {
  let key = if isPrefix { format!("_$_pre_{}", token) } else { format!("{}", token) };
  writeBytes_(db, &key, ii);
}


pub fn appendInvind_(token: &str, isPrefix: bool, invind: &Vec<u8>, db: &rocksdb::DB) {
  let mut existingInvind = readInvind_(token, isPrefix, db);
  existingInvind.extend(invind);
  writeInvind_(token, isPrefix, &existingInvind, db);
}


pub fn writeLocations_(cookedTok: &str, isPrefix: bool, docId: usize, fieldName: &str, locations: &HashSet<u16>, db: &rocksdb::DB) {
  // do locations need to be sorted????  --> not rn.  getting sorted during querying.  prob a trivial query-time cost
    let key = if isPrefix {
      format!("{}_$_{}_$_{}_$_prefix", docId, fieldName, cookedTok)
    } else {
      format!("{}_$_{}_$_{}", docId, fieldName, cookedTok)
    };
    //have to convert vec<u16> to vec<u8> right? or does rocksdb to that automatically?  or unnecessary?
    // https://docs.rs/rocksdb/0.5.1/rocksdb/struct.DB.html#method.put
    // ^ looks like it has to be u8

    let mut locations_vec: Vec<&u16> = Vec::from_iter(locations);
    locations_vec.sort();
    locations_vec.reverse();    // for highlighting //  //this *should* be redundant cos done during querying too.  should it b in one or the other? which?

    let locations_u8: Vec<u8> = bincode::serialize(&locations_vec).unwrap();
    // println!("writeLocations_: key, locations: {}, {:?}", key, locations_u8);

    db.put(key, &locations_u8).unwrap();
}

pub fn deleteLocations_(docId: usize, fieldName: &str, cookedTok: &str, isPrefix: bool, db: &rocksdb::DB) {  //for entire field.  but to do that, we have to delete each field-tok-docId combo object.  cos all fields' locations rae combined in same rocksdb
  let key = if isPrefix {
    format!("{}_$_{}_$_{}_$_prefix", docId, fieldName, cookedTok)
  } else {
    format!("{}_$_{}_$_{}", docId, fieldName, cookedTok)
  };
  db.delete(key);
}

/// locations will only be read from squirrel. (for now, until we get secondary distributed ferrets i think)
pub fn readLocations(clxnsRoot: &str, pid: &str, collection: &str, fieldName: &str,  docId: usize, cookedTok: &str, isPrefix: bool, vintage: &str) -> Vec<u16> {
  let key = if isPrefix {
    format!("{}_$_{}_$_{}_$_prefix", docId, fieldName, cookedTok)
  } else {
    format!("{}_$_{}_$_{}", docId, fieldName, cookedTok)
  };

  let rocksRoot = &getDir_rocksRoot_locs(collection, clxnsRoot, pid, vintage);
  let bytes = readKey(rocksRoot, &key);
  // println!("rocksRoot {}, readLocations bytes: {:?}", rocksRoot, bytes);
  // println!("cargo run gv {} '{}'", rocksRoot, key);
  if bytes.len() == 0 {
    Vec::new()
  }
  else { bincode::deserialize(&bytes).unwrap() }
}
