#![allow(non_snake_case)]
// #![allow(dead_code)]
#![allow(unused_must_use)]
use crate::multiIiIntersection::{multiIiIntersection};
use std::collections::HashMap;
use std::option::Option;
use crate::tokenizing::*;
use crate::rocksdb_tools::*;
// #[macro_use]

use crate::h::*;

use crate::getDocId;
use rocksdb::prelude::*;

use crate::stateManagement::*;
use std::{  fs::File,  io::{self, BufRead, BufReader},  path::Path,};

// use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io::Write;
use std::cmp::max;
use std::cmp::min;
use crate::rocksdb_tools::removeRocksdbLogsOld;

use crate::dirStuff::*;
// use crate::worpError::WError;

extern crate rayon;
use rayon::prelude::*;

use chrono::prelude::*;
// use std::{mem, thread};

extern crate fs_extra;
use fs_extra::dir::get_size;

use std::cmp::Ordering;
use std::error::Error;

use crate::worpError::werr;
use binary_heap_plus::BinaryHeap;



pub fn displayMap() {
    let map = M_ROCKSROOT_DB.read().unwrap();
    println!("in displayMap, M_ROCKSROOT_DB:");
    for k in map.keys() {
      println!("{}, {:?}", k, map.get(k));
    }
}

/// - starts rocksdb dbs if isQueryMode (TODO kill QUERY_MODE !!!!)
/// this could be named better
fn updateDbsMap<'a>(rocksRoot: &str, isQueryMode: bool) {
  let mut map = M_ROCKSROOT_DB.write().unwrap();
  // before we make new rocksdb object, we need to make sure there isn't one already    // this should remove the entire entry for shardID    // is this necessary?
  //TODO comment out this removal stuff!! i dont think its necessary!!! it'll just get overwritten by new value right?!?!?!?! what'st he point of deleting here ...
  match map.remove(rocksRoot){
    Some(mut dbstuffOld) => {dbstuffOld.db = None;},
    None => {}
  }
  std::fs::create_dir_all(&rocksRoot);        //in case we dont open rocksbd rn

  let mut dbstuff = DbStuff{ db: None };
  unsafe {
    if QUERY_MODE && isQueryMode {
      let db: Option<rocksdb::DB>;
      match rocksdb::DB::open_default(&rocksRoot) {
        Ok(_db) => {db = Some(_db);},
        Err(e)   => {
          println!("retrying afer error: {}", e);
          match rocksdb::DB::open_default(&rocksRoot) {       // this is to avoid weird Corruption .... log file found ... error.  //TODO does this *actually* help anything??
            Ok(_db) => {db = Some(_db);},
            Err(e)   => {
              panic!("wtf error: {}", e);
            }
          }
        }
      }
      dbstuff.db = db;
    } else {
      //if not QUERY_MODE, then we're never going to use this map, so it *should* jsut have empty values
    }
  }
  map.insert(st(&rocksRoot), dbstuff);
}

/// just rewritten! does it still work?
pub fn removeVintageFromDbsMap(collectionName: &str, clxnsRoot: &str, pid: &str, vintage: &str) {
  let mut map = M_ROCKSROOT_DB.write().unwrap();

  map.remove(&getDir_rocksRoot_meta(collectionName, clxnsRoot, pid, vintage));
  map.remove(&getDir_rocksRoot_docs(collectionName, clxnsRoot, pid, vintage));
  map.remove(&getDir_rocksRoot_locs(collectionName, clxnsRoot, pid, vintage));

  for s in &getShards(clxnsRoot, pid, collectionName, vintage) {

    map.remove(&getDir_rocksRoot_sovabydid(collectionName, clxnsRoot, pid, vintage, s));
    map.remove(&getDir_rocksRoot_tivabydid(collectionName, clxnsRoot, pid, vintage, s));

    let invindsParentDir = &getDir_invindsParent(collectionName, clxnsRoot, pid, vintage, s);
    for f in &getDirsLast(invindsParentDir) {
      map.remove(&getDir_rocksRoot_invind(collectionName, clxnsRoot, pid, vintage, s, f));
    }
  }
}

pub fn addRocksDbsToStaticMap(collectionName: &str, clxnsRoot: &str, pid: &str, vintage: &str, isQueryMode: bool) {

  if !vintage.ends_with("_busy") {

    updateDbsMap(&getDir_rocksRoot_meta(collectionName, clxnsRoot, pid, vintage), isQueryMode);
    updateDbsMap(&getDir_rocksRoot_docs(collectionName, clxnsRoot, pid, vintage), isQueryMode);
    updateDbsMap(&getDir_rocksRoot_locs(collectionName, clxnsRoot, pid, vintage), isQueryMode);

    //ok what are we rly doing here ... ? opening/preparing all possible rocks dbs.  so we just need to iterate thorugh all the shards of all the invinds and sovabydids
    let shardsRoot = &getDir_shards(&collectionName, clxnsRoot, pid, vintage);
    createDirMaybe(shardsRoot);
    for shardName in getDirsLast(shardsRoot){

      let sovabydidsRocksRoot = &getDir_rocksRoot_sovabydid(collectionName, clxnsRoot, pid, vintage, &shardName);
      updateDbsMap(sovabydidsRocksRoot, isQueryMode);

      let tivabydidsRocksRoot = &getDir_rocksRoot_tivabydid(collectionName, clxnsRoot, pid, vintage, &shardName);
      updateDbsMap(tivabydidsRocksRoot, isQueryMode);

      let invindsParentDir = &getDir_invindsParent(collectionName, clxnsRoot, pid, vintage, &shardName);
      for fieldName in getDirsLast(invindsParentDir) {
        let invindRocksRoot = &getDir_rocksRoot_invind(collectionName, clxnsRoot, pid, vintage, &shardName, &fieldName);
        updateDbsMap(invindRocksRoot, isQueryMode);
      }
    }
  }

}

/// insert rdb cnxn to each existing vintage, even if it's old.  NOTE THIS IS BAD.  might cause downtime cos map gets deleted before re-adding eveyrthing.  could be down for 50 ms.
/// need to update it so it doesn't delete old stuff each time.  just remove M_ROCKSROOT_DB.read().unwrap().clear();  ??? must be a raeason it's ther ...
/// v20 done !?!?!?!
pub fn resetStaticMaps_clxnsRoot(clxnsRoot: &str, isQueryMode: bool) -> Result<(), Box<dyn Error>>{
  //TODO need a way to delete old unused rdb connections after deleting that library version // oh maybe this does that: M_ROCKSROOT_DB.read().unwrap().clear(); ?
  createDirMaybe(clxnsRoot);
  M_ROCKSROOT_DB.write().unwrap().clear();

  for pid in getProjects(clxnsRoot)? {
    // println!("reset looping pid: {}", pid);
    for c in getProjectCollections(clxnsRoot, &pid)? {
      // println!("reset looping c: {}", c);
      for v in getCollectionVintages( &c, clxnsRoot, &pid)? {
        // println!("reset looping v: {}", v);
        addRocksDbsToStaticMap(&c, clxnsRoot, &pid, &v, isQueryMode);
      }
    }
  }
  Ok(())
}

pub fn deleteCollection(pid: &str, collectionName: &str) -> Result<String, Box<dyn Error>>{
  deleteCollection_(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid);
  deleteCollection_(collectionName, FERRET_COLLECTIONS_ROOT, pid);
  Ok(format!("deleted {}/{}", pid, collectionName))
}

pub fn deleteCollection_(collectionName: &str, clxnsRoot: &str, pid: &str) {
  println!("deleteCollection_ c, cxsRoot, pid: {}, {}, {}", collectionName, clxnsRoot, pid);

  let collectionParent = &getDir_collectionParent(collectionName, clxnsRoot, pid);
  println!("collectionParent:, {}", collectionParent);

  std::fs::remove_dir_all(collectionParent);
  println!("deleted collection {} by deleting dir {}", collectionName, collectionParent);
}


/// the ones in the map
pub fn getRocksDbPaths() -> Vec<String> {
  let mut rocksDbPaths: Vec<String> = Vec::new();

  let map = M_ROCKSROOT_DB.read().unwrap();
  for k in map.keys(){
    let shardId = format!("{}", &k);
    rocksDbPaths.push(shardId);
  }
  rocksDbPaths
}

pub fn getShardNumberForDocId(maxShardLen: usize, docId: usize) -> usize {

  let quotient = docId as f32 / maxShardLen as f32;
  quotient.ceil() as usize
}

pub fn collectionExists_clxnsRoot(clxName: &str, clxnsRoot: &str, pid: &str, vintage: &str) -> bool {
  // Path::new(&getCollectionVintageRoot_(&clxName, clxnsRoot, vintage)).exists()
  Path::new( &getDir_collectionVintage(&clxName, clxnsRoot, pid, vintage) ).exists()
}

/// TODO rename this ! not creating a new shard, it's creating a new rocksdb i think
///
/// this opens an rdb connection only if it doens't exist yet so it should be okay
pub fn createAndLoadNewRocksDbMaybe(rocksRoot: &str, isQueryMode: bool){
    let mut doMakeNewShard = false;
    {                                                               // to kill off map, end the lock.  needed for update part later
        // let shardId = &getShardId(indexName, shardName);
        let map = M_ROCKSROOT_DB.read().unwrap();
        if !map.contains_key(rocksRoot){
            doMakeNewShard = true;
        }
    }
    if doMakeNewShard {
      updateDbsMap(&rocksRoot, isQueryMode);
    }
}

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
  BufReader::new(File::open(filename)?).lines().collect()
}

fn getMapForShard<'a>(m__fieldShardTok__m_tt_ii: &'a HashMap<(String, String, String), HashMap<String, Vec<u8>>> , shardNumber: &str) ->  HashMap<(&'a str, &'a str), &'a HashMap<String, Vec<u8>>> {
  let mut m__fieldTok___m_tt_ii: HashMap<(&str, &str), &HashMap<String, Vec<u8>>> = HashMap::new();

  for ((field, shard, tok), m) in m__fieldShardTok__m_tt_ii {
    if shard == shardNumber {
      m__fieldTok___m_tt_ii.insert((field, tok), m);
    }
  }

  m__fieldTok___m_tt_ii
}

// getMapForField(&m__fieldCookedtokTuple_ii, &field);
fn getMapForField<'a>(m__fieldTokTuple_ii: &'a HashMap<(&str, &str), &HashMap<String, Vec<u8>>> , field: &str) ->  HashMap<&'a str, &'a HashMap<String, Vec<u8>>> {
  let mut m__tok__m_tt_ii: HashMap<&'a str, &'a HashMap<String, Vec<u8>>>  = HashMap::new();

  for ((f, tok), m_tt_ii) in m__fieldTokTuple_ii {
    if *f == field {
      m__tok__m_tt_ii.insert(tok, m_tt_ii);
    }
  }
  m__tok__m_tt_ii
}
pub fn indexBulkSingleDoc(pid: &str, collectionName: &str, doc: &str){
  let docs: Vec<String> = vec![doc.to_string()];
  let amount = 123;
  indexBulk_docs(pid, collectionName, docs, amount);
  publishNewVintage(pid, collectionName);   //a "vintage" by definition is stored by/with/for a ferret, i think
}

// pub fn indexBulk_str(pid: &str, collectionName: &str, docsStr: &str, maxRows: usize) -> Result<String, Box<dyn Error>> {
//   let lines: Vec<&str> = docsStr.lines().collect();
//   indexBulk_docs(pid, collectionName, lines, maxRows)
// }

pub fn indexBulk(pid: &str, collectionName: &str, fileWithDocPerLine: &str, maxRows: usize) -> Result<String, Box<dyn Error>> {
  let path = fileWithDocPerLine;
  let lines = lines_from_file(path).expect("Could not load lines");
  println!("read {} docs from {}", lines.len(), path);
  let result = indexBulk_docs(pid, collectionName, lines, maxRows);
  publishNewVintage(pid, collectionName);   //a "vintage" by definition is stored by/with/for a ferret, i think
  result
}

#[allow(dead_code)]
pub fn indexBulk_ndjson(pid: &str, collectionName: &str, ndjsonStr: &str)  -> Result<String, Box<dyn Error>>  {
  let lines: Vec<String> = ndjsonStr.lines().map(str::to_string).collect();                                                       //https://stackoverflow.com/questions/37547225/split-a-string-and-return-vecstring
  println!("inputted {} docs", lines.len());
  indexBulk_docs(pid, collectionName, lines, usize::MAX)
}

pub fn indexBulk_docs(pid: &str, collectionName: &str, docStrs: Vec<String>, maxRows: usize) -> Result<String, Box<dyn Error>>  {
  let mut m_docId_doc: BTreeMap<usize, String> = BTreeMap::new();

  let docIds = getKeysAsIntsSortedIncreasing_(&openDb(&getDir_rocksRoot_docs(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE)));
  let maxDocId = docIds.last().unwrap();

  let numDocsToIndex = min(maxRows, docStrs.len());

  let mut prevDocId = maxDocId;
  let mut currDocId;
  for i in 0..numDocsToIndex {                                               /* generate invinds (prefix and normal (and phrases?)) and write to maps for this doc */
    let docJsonStr = &docStrs[i];
    currDocId = prevDocId + 1;
    m_docId_doc.insert(currDocId, st(docJsonStr));
    prevDocId = &currDocId;
  }

  let isFieldReindexing = false;

  /* allow indexing for the following: */

  let allowSort = true;
  let allowTimeindexing = true;
  let allowPrefix = true;
  let allowSearch = true;

  indexBulk_docsMap(pid, collectionName, m_docId_doc, isFieldReindexing, allowSort, allowTimeindexing, allowPrefix, allowSearch)
}

pub fn indexBulk_docsMap(pid: &str, collectionName: &str, m_docId_doc: BTreeMap<usize, String>, isFieldReindexing: bool, allowSort: bool, allowTimeindexing: bool, allowPrefix: bool, allowSearch: bool) -> Result<String, Box<dyn Error>>  {

  if allowPrefix && !allowSearch { panic!("allowPrefix, allowSearch:  {}, {}", allowPrefix, allowSearch); }

  let v = DEFAULT_VINTAGE;
  let r = SQUIRREL_COLLECTIONS_ROOT;
  let c = collectionName;

  let mut m__fieldShardTok__m_tt_ii:   HashMap<(String, String, String),  HashMap<String, Vec<u8>>>      = HashMap::new();
  let mut m_fieldShard_sov:            HashMap<(String, String),          Vec<f32>>                      = HashMap::new();
  let mut m_fieldShard_tiv:            HashMap<(String, String),          Vec<u64>>                      = HashMap::new();
  let mut m__tokFieldDocid__m_tt_locs: HashMap<(String, String, usize),   HashMap<String, HashSet<u16>>> = HashMap::new();
  let mut shardsToWriteTo:             HashSet<String>                                                   = HashSet::new();
  let mut allFoundFieldNames:          HashSet<String>                                                   = HashSet::new();

  let metaDb = &openDb(&getDir_rocksRoot_meta(c, r, pid, v));
  let docsDb = &openDb(&getDir_rocksRoot_docs(c, r, pid, v));
  let locsDb = &openDb(&getDir_rocksRoot_locs(c, r, pid, v));

  let mut schema = readSchema_(metaDb);
  let mut schema_was_changed = false;
  for field in &schema.fields {                                               /* load pre-existing sovabydids and tivabydids  */
    let f = &field.name;
    for s in &getShards(r, pid, c, v) {
      let sovabydid = readSovabydid(c, r, pid, v, s, f);
      let tivabydid = readTivabydid(c, r, pid, v, s, f);
      if sovabydid.len() > 0 {
        m_fieldShard_sov.insert((st(f), st(s)), sovabydid);
      }
      if tivabydid.len() > 0 {
        m_fieldShard_tiv.insert((st(f), st(s)), tivabydid);
      }
    }
  }
  let mut count = 0;
  for (docId, docJsonStr) in &m_docId_doc {                                   /* generate invinds (prefix and normal (and phrases?)) and write to maps for this doc */
    count += 1;
    let docJson = getDocJson(&docJsonStr);
    if count % 100 == 0 { print!("{} ({})..., ", count, docId); io::stdout().flush(); }
    let shardName = &getShardName(*docId, &docJson, &schema);
    shardsToWriteTo.insert(st(shardName));
    for (fieldName, fieldValue) in docJson {
      allFoundFieldNames.insert(st(&fieldName));

      if !schemaContainsField(&schema, &fieldName) {
        schema = updateSchemaWithNewField(schema, &fieldName);
        schema_was_changed = true;
      }
      let fields = &schema.fields;

      let isForSorting      = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].sortThisGuy              && allowSort;
      let isForTimegraphing = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].is_graphable_timestamp   && allowTimeindexing;
      let isForPrefixes     = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].doIndexPrefixes          && allowPrefix;
      let isForSearching    = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].searchMe                 && allowSearch;

      indexBulk_worker(schema.max_docs_per_shard, &fieldName, &fieldValue, *docId, &shardName,  isForSorting,
                                                                                                isForTimegraphing,
                                                                                                isForPrefixes,
                                                                                                isForSearching,
                                                                                                &mut m__fieldShardTok__m_tt_ii,
                                                                                                &mut m_fieldShard_sov,
                                                                                                &mut m_fieldShard_tiv,
                                                                                                &mut m__tokFieldDocid__m_tt_locs );
    }
  }
  if schema_was_changed {                                                        /* write schema if changed   */
    writeSchema_(&schema, metaDb);
  }
  if allowSearch {                                                                /* write [append] invinds (normal and prefix) to shards */
    for shardName in &shardsToWriteTo {
      let m__fieldTok__m_tt_ii:  HashMap<(&str, &str), &HashMap<String, Vec<u8>>>  =  getMapForShard(&m__fieldShardTok__m_tt_ii,  &shardName);
      for field in &allFoundFieldNames {
        let m__tok__m_tt_ii:  HashMap<&str, &HashMap<String, Vec<u8>>>  =  getMapForField(&m__fieldTok__m_tt_ii, field);
        let iiRr = getDir_rocksRoot_invind(c, r, pid, v, &shardName, field);
        createAndLoadNewRocksDbMaybe(&iiRr, false);
        let iiDb = &openDb(&iiRr);
        for (tok, m_tt_ii) in m__tok__m_tt_ii {
          for (tokType, ii) in m_tt_ii{
            // appendInvind_new(tok, tokType == "PREFIX", ii, &iiDb);                                                      // we can append b/c bulk indexing always uses bigger docIds than any existing docId
            appendInvind_(tok, tokType == "PREFIX", ii, &iiDb);                                                      // we can append b/c bulk indexing always uses bigger docIds than any existing docId
          }
        }
      }
    }
    println!("writing locations to locsDb: {:?}", locsDb);                        /* write locations to central locsDb */                   //would this ever happen with field re-indexing? ... no?
    for ((tok, field, docId), m_tt_locs) in m__tokFieldDocid__m_tt_locs {
      for (tokType, locations) in m_tt_locs {
        // writeLocations_new(&tok, tokType == "PREFIX", docId, &field, &locations, locsDb);
        writeLocations_(&tok, tokType == "PREFIX", docId, &field, &locations, locsDb);
      }
    }
  }
  if allowSort || allowTimeindexing {      //redundant. for viz consistency
    for s in &shardsToWriteTo {                                                   /* write updated sovabydids and tivabydids */               //we could open the dbs here ... trivial savings for bulk fx tho
      for f in &allFoundFieldNames {
        let key = &(st(f), st(s));                                                                                  //what if we find a new field along the way? then it gets added to the schema earlier, and new sov and tiv get added to their maps // but maybe not every field is represented in every shard.  that's okay
        if allowSort {
          // println!("allowSort: {}, key: {:?}", allowSort, key);
          match m_fieldShard_sov.get(key) { Some(sov) => writeSovabydid(&sov, f, c, pid, s), None => { println!("none TODO STAERT HEREJERSELFKSLKFSLDKFJ"); }};
        }
        if allowTimeindexing {
          match m_fieldShard_tiv.get(key) { Some(tiv) => writeTivabydid(&tiv, f, c, pid, s), None => {}};
        }
      }
    }
  }
  if !isFieldReindexing {
    println!("writing documents to owlDataDb: {:?}", docsDb);                     /* write docs                */
    for (docId, docJsonStr) in &m_docId_doc {
      // writeDoc_new(docId, docJsonStr, docsDb, true);
      writeDoc_(*docId, docJsonStr, docsDb, true);
    }
    let countedNumDocs = countObjects(docsDb);                                    /* update total docs in meta */
    writeNumDocs_(countedNumDocs, metaDb);
  }

  publishNewVintage(pid, collectionName);   //a "vintage" by definition is stored by/with/for a ferret, i think
  Ok(format!("bulk-indexed {} docs", m_docId_doc.len()))
}

pub fn indexBulk_worker(maxDocsPerShard: usize,
                        fieldName: &str,
                        fieldValue: &Value,
                        docId: usize,
                        shardNumber: &str,
                        isForSorting: bool,
                        isForTimegraphing: bool,
                        doIndexPrefixes: bool,
                        isForSearching: bool,
                        m__fieldShardTok__m_tt_ii:   &mut HashMap< (String, String, String), HashMap<String, Vec<u8>>       >,
                        m_fieldShard_sov:            &mut HashMap< (String, String),         Vec<f32>                       >,
                        m_fieldShard_tiv:            &mut HashMap< (String, String),         Vec<u64>                       >,
                        m__tokFieldDocid__m_tt_locs: &mut HashMap< (String, String, usize),  HashMap<String, HashSet<u16>>  >   ) {

  let key = &(st(fieldName), st(shardNumber));                                                                                  //what if we find a new field along the way? then it gets added to the schema earlier, and new sov and tiv get added to their maps // but maybe not every field is represented in every shard.  that's okay

  // println!("here -0 key: {:?}, isForSorting: {}", key, isForSorting);
  if isForSorting {
    // println!("here 000");
    let sortValue: f32 = if fieldValue.is_number() { fieldValue.as_f64().unwrap() as f32} else {fieldValue.as_str().unwrap().parse().expect(&format!("value unable to parse as float: {}", fieldValue))};

    // println!("here 00001 sortValue: {:?}", sortValue);
    if !m_fieldShard_sov.contains_key(key) {

      // println!("here 001");
      m_fieldShard_sov.insert(key.clone(), vec![f32::NAN; maxDocsPerShard]);    // this is it.  no need to be so big right??? not but doesn't matter rn https://trello.com/c/S3VsWya7/147-dont-make-sovabydid-so-big
    }

    // println!("here 002");
    let sov = m_fieldShard_sov.get_mut(key).unwrap();

    // println!("here 003 sov: {:?}", sov);
    sov[docId] = sortValue;
    // println!("here 004 sov: {:?}", sov);
  }
  if isForTimegraphing {
    let timestamp: u64 = if fieldValue.is_number() { fieldValue.as_u64().unwrap() as u64} else {fieldValue.as_str().unwrap().parse().expect(&format!("value unable to parse as timestamp: {}", fieldValue))};
    if !m_fieldShard_tiv.contains_key(key) {    //m_fieldShard_tiv contains an empty tivabydid?!?!?!?!?!?!
      // println!("not here {:?}", key);
      m_fieldShard_tiv.insert(key.clone(), vec![0; maxDocsPerShard]);
    }
    // else {println!("found! {:?}", key);}
    let tiv = m_fieldShard_tiv.get_mut(key).unwrap();
    // println!("tiv::::! {:?}", tiv);
    tiv[docId] = timestamp;
  }

  if isForSearching && fieldValue.is_string() {
    let phraseTokWordLengths: Vec<usize> = vec![1,2,3];
    let m__toksType__num_mCookedtokLocations = tokenize(&fieldValue.as_str().unwrap(), phraseTokWordLengths, doIndexPrefixes).0;    //includes prefixes as tokType "PREFIX" (if doIndexPrefixes == true)

    for (tokType, (numTotalRawToks, m_cookedTok_locations)) in m__toksType__num_mCookedtokLocations {

      for (cookedTok, locations) in &m_cookedTok_locations {
        let count = locations.len();
        let relevance = getRelevance(numTotalRawToks, count);
        let newDocIdRelBytes = getBytesFromDocIdAndRelevance(docId, relevance);

        {
          let key = (st(fieldName), st(shardNumber), st(cookedTok));
          m__fieldShardTok__m_tt_ii                                             //TODO expand this into a map of a map of a map of a map... (for field, shard, tok, tokType)
                                              .entry(key)                                  //or maybe should add tokType to tuple????  idfk
                                              .or_insert_with(HashMap::new)
                                              .entry(tokType.to_string())
                                              .or_insert_with(Vec::new)
                                              .extend(newDocIdRelBytes.to_vec());
        }
        {
          let key = (st(cookedTok), st(fieldName), docId);
          m__tokFieldDocid__m_tt_locs
                                              .entry(key)
                                              .or_insert_with(HashMap::new)
                                              .entry(tokType.to_string())
                                              .or_insert_with(HashSet::new)
                                              .extend(locations);
        }
      }
    }
  }
}

/// locations should be a set, not vec (?)
pub fn convertToPrefixes(m_cookedTok_locations:  &HashMap<String, HashSet<u16>>) -> HashMap<String, HashSet<u16>> {
  let mut m_prefix_locations: HashMap<String, HashSet<u16>>  = HashMap::new();

  for (cookedTok, locations) in m_cookedTok_locations {
    let prefixes = getPrefixesForToken(&cookedTok);
    for prefix in prefixes {
      m_prefix_locations.entry(prefix)
                        .or_insert_with(HashSet::new)
                        .extend(locations);
    }
  }
  m_prefix_locations
}

// // TODO re-implement - remember to delete the locations too? and to update numDocs ?
pub fn unindexDocFieldValue_worker(pid: &str, collection: &str, fieldName: &str, fieldValue: &Value, docId: usize, isForSorting: bool, isForTimegraphing: bool, doIndexPrefixes: bool, isForSearching:bool, shardName: &str) {

  if isForSorting {
    deleteSovabydid(fieldName, collection, pid, shardName);
  }
  if isForTimegraphing {
    writeTivabydid(&Vec::new(), fieldName, collection, pid, shardName);
  }

  if isForSearching && fieldValue.is_string() {

    let invindsRocksRoot = getDir_rocksRoot_invind(collection, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, &shardName, fieldName);
    let iiDb = &openDb(&invindsRocksRoot);
    let locationsRocksRoot = getDir_rocksRoot_locs(collection, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE);
    let locsDb = &openDb(&locationsRocksRoot);

    let phraseTokWordLengths: Vec<usize> = vec![1,2,3];

    let m__toksType__num_mCookedtokLocations = tokenize(&fieldValue.as_str().unwrap(), phraseTokWordLengths, doIndexPrefixes).0;

    for (tokType, (_numRawToks, m_cookedTok_locations)) in m__toksType__num_mCookedtokLocations {

      for (cookedTok, _locations) in &m_cookedTok_locations {                                           //possible to optimize by not actually getting locations for delete fx?
        let mut ii = readInvind_(cookedTok, tokType == "PREFIX", iiDb);
        // println!("cookedTok: {}", cookedTok);
        // println!("init ii: {:?}", ii);

        let iiLen = ii.len();                                                                                //for performance (?)
        let mut i = 0;
        while i < iiLen {
          let other_docId = getDocId!(ii, i) as usize;
          if other_docId == docId {
            ii.drain(i..(i+5));
            break;
          }
          i += 5;
        }
        // println!("post ii: {:?}", ii);
        writeInvind_(&cookedTok, tokType == "PREFIX", &ii, iiDb);
        // let doubleCheckIi = readInvind_(&cookedTok, tokType == "PREFIX", iiDb);
        // println!("read ii: {:?}", doubleCheckIi);
        // println!("   iiDb: {:?}", iiDb);

        deleteLocations_(docId, fieldName, cookedTok, tokType == "PREFIX", locsDb);
      }
    }
  }
}

/// DO NOT DELETE
/// this is research to see how long it would take to highlight matches by using a single locations rdb object per doc: m__cookedTok_locations : HashMap<String, Vec<u16>> /
/// finding locations for 4 tokens including 1 prefix took 500 µs -- so page 1 for 10 docs would be around an extra 5 ms.  that seems worth it.  db size would be literally cut in half. (which is a big deal cos rememebr db x 3 for 1 squirrel and 2 copies for the ferrets)
pub fn findCookedToks(mystring: &str) {
  let start = std::time::Instant::now();

  let m__toksType__num_mCookedtokLocations = tokenize(mystring, vec![1], false).0;

  let awefasdf = m__toksType__num_mCookedtokLocations.get("1").unwrap();

  let mut m_cookedTok_locations = awefasdf.1.clone();

  println!("m_cookedTok_locations: {:?}", m_cookedTok_locations);

  // println!("m__toksType__num_mCookedtokLocations: {:?}", m__toksType__num_mCookedtokLocations);
  // println!("num_mCookedtokLocations: {:?}", num_mCookedtokLocations);

  //convert m__toksType__num_mCookedtokLocations to bytes to check len
  let asbytesDoc: Vec<u8> = bincode::serialize(mystring).unwrap();
  let asbytesMap: Vec<u8> = bincode::serialize(&m_cookedTok_locations).unwrap();

  println!("num bytes documStr: {:?}", asbytesDoc.len());
  println!("num bytes locs map: {:?}", asbytesMap.len());

  let dur =  start.elapsed();

  println!("took {:?}", dur);

  let mut queryCookedToks: Vec<String>= Vec::new();
  queryCookedToks.push(st("when"));
  queryCookedToks.push(st("mary"));
  queryCookedToks.push(st("england"));
  queryCookedToks.push(st("ev"));

  // now find all the start locations for these toks in the tokenized input str
  // find prefix matches for the last one

  let last = &queryCookedToks[queryCookedToks.len() - 1];

  let start2 = std::time::Instant::now();

  m_cookedTok_locations.retain(|key, _value| {  queryCookedToks.contains(key) || key.starts_with(last) });

  println!("took {:?}", start2.elapsed());
  println!("m_cookedTok_locations after retain: {:?}", m_cookedTok_locations);
}

pub fn indexDocFieldValue_worker(collection: &str, fieldName: &str, fieldValue: &Value, verbose: bool, docId: usize, isForSorting: bool,  isForTimegraphing:bool, doIndexPrefixes: bool, isForSearching: bool,
                                  shardName: &str, shardRdb: &rocksdb::DB, locsRdb: &rocksdb::DB, sovabydidsRdb: &rocksdb::DB, tivabydidsRdb: &rocksdb::DB){
  if isForSorting {                                                               /****** index the sorting value ******/  //TODO this should go after all the searchable fields have been indexed?  cos we need to add a value to the sorter field's sorter vec for EVERY docId, even when the sorter is missing.                                                                         /*  ***************** index the sorting value ***************** */
    let mut sovabydid = readSovabydid_(&fieldName, sovabydidsRdb);
    if sovabydid.len() < docId + 1 { sovabydid.resize(docId + 1, f32::NAN); }     // https://trello.com/c/ijgxkDlr/145-use-docid-offsets-everywhere dont use offsetId.  no reason to right now.  not using shards yet.  //resize the new sorter to ensure capacity for the this doc being indexed

    // support sorter as string or numeric ...
    if fieldValue.is_string() {
      let temp = fieldValue.as_str().unwrap();
      sovabydid[docId] = temp.parse::<f32>().unwrap();
    }
    else if fieldValue.is_number() {
      let temp = fieldValue.as_f64().unwrap();
      sovabydid[docId] = temp as f32;
    }
    else {
      panic!("unsupported sorter type collection {} field {} value {}", collection, fieldName, fieldValue);
    }
    writeSovabydid_(&sovabydid, &fieldName, sovabydidsRdb);
  }
  // println!("isForTimegraphing: {:?}", isForTimegraphing);
  if isForTimegraphing {

    let mut tivabydid = readTivabydid_(&fieldName, tivabydidsRdb);
    // println!("0 tivabydid: {:?}", tivabydid);
    if tivabydid.len() < docId + 1 { tivabydid.resize(docId + 1, 0); }
    // println!("1 tivabydid: {:?}", tivabydid);

    // fieldValue must be integer unix epoch time
    if fieldValue.is_number() {
      let temp = fieldValue.as_u64().unwrap();
      tivabydid[docId] = temp;
    }
    else {
      panic!("unsupported timestamp type, collection {} field {} value {}", collection, fieldName, fieldValue);
    }
    // println!("2 tivabydid: {:?}", tivabydid);
    writeTivabydid_(&tivabydid, &fieldName, tivabydidsRdb);
  }
  if isForSearching && fieldValue.is_string() {
    let phraseTokWordLengths: Vec<usize> = vec![1,2,3];

    let m__toksType__num_mCookedtokLocations = tokenize(&fieldValue.as_str().unwrap(), phraseTokWordLengths, doIndexPrefixes).0;

    for (tokType, (numRawToks, m_cookedTok_locations)) in m__toksType__num_mCookedtokLocations {

      for (cookedTok, locations) in &m_cookedTok_locations {
        let count = locations.len();
        let relevance = getRelevance(numRawToks, count);
        let newDocIdRelBytes = getBytesFromDocIdAndRelevance(docId, relevance);
        let mut ii =  readInvind_(cookedTok, tokType == "PREFIX", shardRdb);


        let iiLen = ii.len();                                                     //looks silly, but leave this here for performance reasons
        let mut i = 0;
        let mut didInsert = false;                                              //if docId exists: iterate until u find an otherDocId greater than docId. then slip this one in rihgt before it. else: put it at the end.
        while i < iiLen {                                                           //... should we binskip to find where to insert? prob not. overhead makes it slow irl
          let other_docId = getDocId!(ii, i) as usize;
          if other_docId > docId {
            ii.splice(i..i, newDocIdRelBytes.iter().cloned());                         //https://stackoverflow.com/questions/28678615/efficiently-insert-or-replace-multiple-elements-in-the-middle-or-at-the-beginnin
            didInsert = true;
            break;
          }
          i += 5;                                                           //this should b a constant somewhere? ... whatif we change the binary format.  but i think it's faster to not use a var here? maybe
        }
        if !didInsert {
          ii.append(&mut newDocIdRelBytes.to_vec());                      //then insert it at the end.  it belongs in this shard, and it must be bigger than all current docIds
        }

        writeInvind_(cookedTok, tokType == "PREFIX", &ii, shardRdb);

        let key = format!("{}", &cookedTok);
        //ignoring this stuff for v20 cos non-critical
        if verbose {println!("wrote docId, fieldName, shard, subShard, key:  {:<3} {:<8} {:<2} {:<10}   {:?}", docId, fieldName, shardName, key, readInvind_(cookedTok, doIndexPrefixes, shardRdb));}
        if verbose {println!();}
        // if verbose {println!("counted {} docs", countNumDocsForCollection(&collection));}

        // println!("about to write locations: {:?}", locations);
        writeLocations_(cookedTok, tokType == "PREFIX", docId, &fieldName, locations, locsRdb);
      }
    }
  }
}

pub fn createCollection_fromObj(schemaParsedUserInput_input: WorpdriveSchema_userInput) -> Result<String, Box<dyn Error>> {

  let schemaParsedUserInput = sanitizeSchema_userInput(schemaParsedUserInput_input);

  let schemaStr : String = serde_json::to_string(&schemaParsedUserInput).unwrap();
  // println!("schemaStr: {:?}", schemaStr);
  let schema = parseSchemaStr(&schemaStr);

  let collectionName = &schema.collection;
  let pid            = &schema.pid;

  let clxnsRoot = SQUIRREL_COLLECTIONS_ROOT;
  let vintage = DEFAULT_VINTAGE;

  let dataMainDir = &getDir_main(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE);

  if exists(dataMainDir) {
    return Err(werr(&format!("collection [{}] already exists", collectionName)));
  }

  //should these even happen?  we're creating a collection ... that's in the squirrel... which is NOT query mode .... so these dbs wont be in the map anyway ...
  // updateDbsMap(&getDir_rocksRoot_meta(collectionName, clxnsRoot, pid, vintage), isQueryMode);
  // updateDbsMap(&getDir_rocksRoot_docs(collectionName, clxnsRoot, pid, vintage), isQueryMode);
  // updateDbsMap(&getDir_rocksRoot_locs(collectionName, clxnsRoot, pid, vintage), isQueryMode);

  std::fs::create_dir_all(&getDir_rocksRoot_meta(collectionName, clxnsRoot, pid, vintage));
  std::fs::create_dir_all(&getDir_rocksRoot_docs(collectionName, clxnsRoot, pid, vintage));
  std::fs::create_dir_all(&getDir_rocksRoot_locs(collectionName, clxnsRoot, pid, vintage));

  //we dont do invinds or sovabydids yet (sharded stuff) -- that stuff's created on the fly when a new shard is created (cos it has to be cos doc size has grown)

  writeSchema(pid, collectionName, &schema);
  writeMarcoPolo(pid, collectionName);
  writeNumDocs(pid, collectionName, 0);
  //write billingMetrics - should this go here too? for sanity
  publishNewVintage(pid, collectionName);   //a "vintage" by definition is stored by/with/for a ferret, i think

  if schema.do_log_queries {
    createHiddenQueryCollection(pid, collectionName);
  }
  // schema
  Ok(serde_json::to_string_pretty(&schema).unwrap())
}

/// return schema
pub fn createCollection(schemaJsonStr: &str) -> Result<String, Box<dyn Error>> {
  println!("createCollection: schemaJsonStr: {}", schemaJsonStr);
  let schemaParsedUserInput: WorpdriveSchema_userInput = serde_json::from_str(&schemaJsonStr).unwrap();
  createCollection_fromObj(schemaParsedUserInput)
}

fn createHiddenQueryCollection(pid: &str, parentCollection: &str) -> Result<String, Box<dyn Error>> {

  let mut inputSchemaMap: HashMap<&str, Value> = HashMap::new();
  inputSchemaMap.insert("pid",         serde_json::json!(pid));
  inputSchemaMap.insert("collection",  serde_json::json!(format!("{}_hiddenQueryCollection", parentCollection)));
  inputSchemaMap.insert("is_worptail", serde_json::json!(true));

  let inputSchemaStr = serde_json::to_string(&inputSchemaMap).unwrap();

  println!("createHiddenQueryCollection inputSchemaStr: {}", inputSchemaStr);

  createCollection(&inputSchemaStr)
}

/// actually just returns a map for now.  later shoudl prob get a real json object.  for nested fields objects etc
///
/// /// https://github.com/serde-rs/json
pub fn getDocJson(docJsonStr: &str) -> BTreeMap<String, Value> {
  let docJson: BTreeMap<String, Value> = serde_json::from_str(docJsonStr).unwrap();
  docJson
}

pub fn sanitizeQueryUserInput(mut q: WorpdriveQueryParent_userInput, schema: &WorpdriveSchema ) -> WorpdriveQueryParent {
  let collectionName = &q.collection;
  // let worpId = &q.pid;

  if q.num_results_per_page         .is_none() { q.num_results_per_page           = Some(schema.default_num_results_per_page);            }
  if q.do_log_query_for_analytics   .is_none() { q.do_log_query_for_analytics     = Some(true);                                        }
  if q.page_number                  .is_none() { q.page_number                    = Some(1);                                                                        }
  if q.queries                      .is_none() { q.queries                        = Some(Vec::new());                                                          }
  if q.sort_by                      .is_none() { q.sort_by                        = Some(vec![SortByObject{name:st("_score"), is_descending: true}]);          }
  if q.fields_to_return             .is_none() { q.fields_to_return               = Some(vec![st("*")]);                                                       }
  if q.do_highlights_tagged         .is_none() { q.do_highlights_tagged           = Some(false);                                                                }
  if q.do_highlights_objects        .is_none() { q.do_highlights_objects          = Some(false);                                                                }
  if q.do_highlights_map            .is_none() { q.do_highlights_map              = Some(false);                                                                }
  if q.highlight_pre_tag            .is_none() { q.highlight_pre_tag              = Some(st("<em>"));                                                                }
  if q.highlight_post_tag           .is_none() { q.highlight_post_tag             = Some(st("</em>"));                                                                }
  if q.min_highlight_context        .is_none() { q.min_highlight_context          = Some(20);                                                                }
  if q.max_total_snippets_length    .is_none() { q.max_total_snippets_length      = Some(600);                                                                }

  let mut queries = q.queries.unwrap();
  if queries.len() == 0 {
    queries = vec![WorpdriveQueryChild_userInput{ query: None, fields: None, doPrefixLast: None, collection: None}];
  }

  /* there must be a better way */

  for i in 0..queries.len() {
    let qo = &queries[i];
    if qo.query.is_none() {
      queries[i] = WorpdriveQueryChild_userInput {
        query: Some(st("")),
        fields: if qo.fields.is_none() { None } else { Some(qo.fields.as_ref().unwrap().to_vec()) },
        doPrefixLast: qo.doPrefixLast,
        collection: if qo.collection.is_none() { None } else { Some(qo.collection.as_ref().unwrap().to_string())}
      };
    }
  }
  for i in 0..queries.len() {
    let qo = &queries[i];
    if qo.fields.is_none() {
      queries[i] = WorpdriveQueryChild_userInput {
        query: Some(qo.query.as_ref().unwrap().to_string()),
        fields: Some(vec![st("*")]),
        doPrefixLast: qo.doPrefixLast,
        collection: if qo.collection.is_none() { None } else { Some(qo.collection.as_ref().unwrap().to_string())}
      };
    }
  }
  for i in 0..queries.len() {
    let qo = &queries[i];
    if qo.doPrefixLast.is_none() {
      queries[i] = WorpdriveQueryChild_userInput {
        query: Some(qo.query.as_ref().unwrap().to_string()),
        fields: Some(qo.fields.as_ref().unwrap().to_vec()),
        doPrefixLast: Some(true),
        collection: if qo.collection.is_none() { None } else { Some(qo.collection.as_ref().unwrap().to_string())}
      };
    }
  }
  for i in 0..queries.len() {
    let qo = &queries[i];
    if qo.collection.is_none() {
      queries[i] = WorpdriveQueryChild_userInput {
        query: Some(qo.query.as_ref().unwrap().to_string()),
        fields: Some(qo.fields.as_ref().unwrap().to_vec()),
        doPrefixLast: qo.doPrefixLast,
        collection: Some(st(collectionName))
      };
    }
  }
  q.queries = Some(queries);

  let queryParentStr = serde_json::to_string(&q).unwrap();
  // println!("queryParentStr: {}", queryParentStr);
  serde_json::from_str(&queryParentStr).unwrap()                    // this should panic if sort_by values are incomplete i think. and if collection is missing
}



pub fn sanitizeSchema_userInput(mut s: WorpdriveSchema_userInput) -> WorpdriveSchema_userInput{

  if s.default_num_results_per_page      .is_none() { s.default_num_results_per_page           = Some(10);            }
  if s.do_log_queries                    .is_none() { s.do_log_queries                         = Some(false);            }
  if s.is_worptail                       .is_none() { s.is_worptail                            = Some(false);            }
  if s.max_docs_per_shard                .is_none() { s.max_docs_per_shard                     = Some(150000);            }
  if s.fields                            .is_none() { s.fields                                 = Some(Vec::new());            }

  let mut fields = s.fields.unwrap();

  // sanitize all the fields
  for i in 0..fields.len() {
    let fieldObj = &fields[i];
    if fieldObj.searchMe.is_none() {
      fields[i] = WorpdriveFieldObject_userInput {name: fieldObj.name.to_string(), searchMe: Some(true), sortThisGuy: fieldObj.sortThisGuy, doIndexPrefixes: fieldObj.doIndexPrefixes, is_graphable_timestamp: fieldObj.is_graphable_timestamp };
    }
  }
  for i in 0..fields.len() {
    let fieldObj = &fields[i];
    if fieldObj.sortThisGuy.is_none() {
      fields[i] = WorpdriveFieldObject_userInput {name: fieldObj.name.to_string(), searchMe: fieldObj.searchMe, sortThisGuy: Some(false), doIndexPrefixes: fieldObj.doIndexPrefixes, is_graphable_timestamp: fieldObj.is_graphable_timestamp };
    }
  }
  for i in 0..fields.len() {
    let fieldObj = &fields[i];
    if fieldObj.doIndexPrefixes.is_none() {
      fields[i] = WorpdriveFieldObject_userInput {name: fieldObj.name.to_string(), searchMe: fieldObj.searchMe, sortThisGuy: fieldObj.sortThisGuy, doIndexPrefixes: Some(true), is_graphable_timestamp: fieldObj.is_graphable_timestamp }; //defaults to true
    }
  }
  for i in 0..fields.len() {
    let fieldObj = &fields[i];
    if fieldObj.is_graphable_timestamp.is_none() {
      fields[i] = WorpdriveFieldObject_userInput {name: fieldObj.name.to_string(), searchMe: fieldObj.searchMe, sortThisGuy: fieldObj.sortThisGuy, doIndexPrefixes: fieldObj.doIndexPrefixes, is_graphable_timestamp: Some(false) }; //defaults to false
    }
  }

  // if is_worptail==true but none of the fields are is_graphable_timestamp, then we need to add a timestamp field

  let mut number_of_userDefined_timestamp_fields = 0;

  for i in 0..fields.len() {
    let  fieldObj = &fields[i];
    if fieldObj.is_graphable_timestamp.unwrap() == true {
      number_of_userDefined_timestamp_fields += 1;
    }
  }

  if number_of_userDefined_timestamp_fields > 1 {
    panic!("only 1 timestamp field supported right now sorry");
  }

  if s.is_worptail.unwrap() == true {
    if number_of_userDefined_timestamp_fields == 0 {
      let field =WorpdriveFieldObject_userInput {name: format!("{}timestamp", WORP_AUTO_FIELD_PREFIX), searchMe: Some(true), sortThisGuy: Some(false), doIndexPrefixes: Some(false), is_graphable_timestamp: Some(true) };
      fields.push(field);
    }
  }
  s.fields = Some(fields);

  s
}


pub fn deleteDocInCollection(pid: &str, collectionName: &str,  docId: usize) -> Result<String, Box<dyn Error>> {
  //delete sorters, tivs, location, prefixes...

  // basically we have to read the doc then tokenize it as tho we were gonna index it, but use the tokenized stuff to remove any traces of itself from existing dbs


  let metaRdb = &openDb(&getDir_rocksRoot_meta(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE));
  let docsRdb = &openDb(&getDir_rocksRoot_docs(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE));

  let docJsonStr = readDoc_(docsRdb, docId);
  if docJsonStr.len() == 0 {
    return Err(werr(&format!("document with _id [{}] not found", docId)));    //TODO we should throw an error from rocksdb code if object/key not found right ... ?  should go thorugh and change that once MVP locked down?
  }
  let docJson = getDocJson(&docJsonStr);

  let schema = readSchema_(metaRdb);
  // println!("schema:\n {}", serde_json::to_string_pretty(&schema).unwrap());

  let shardName = &getShardName(docId, &docJson, &schema);

  for (fieldName, fieldValue) in docJson {
    println!("deleteDocInCollection:: docId ({}), key ({}), value: {}", docId, fieldName, fieldValue);
    let fields = &schema.fields;

    let isForSorting      = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].sortThisGuy;
    let isForTimegraphing = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].is_graphable_timestamp;
    let isForPrefixes     = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].doIndexPrefixes;
    let isForSearching    = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].searchMe;

    unindexDocFieldValue_worker(pid, collectionName, &fieldName, &fieldValue, docId, isForSorting, isForTimegraphing, isForPrefixes, isForSearching, &shardName);
  }
  unwriteDoc_(docsRdb, docId);
  writeNumDocs_(readNumDocs_(metaRdb) - 1, metaRdb);
  publishNewVintage(pid, collectionName);
  Ok(format!("deleted doc with docId: {}", docId))
}

pub fn currentTimeMillis() -> u128 {

  use std::time::{SystemTime, UNIX_EPOCH};

  let start = SystemTime::now();
  let since_the_epoch = start
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards");
  let in_ms = since_the_epoch.as_secs() as u128 * 1000    +   since_the_epoch.subsec_millis() as u128;

  in_ms
}
///a "vintage" by definition is stored in a ferret's burrow
pub fn publishNewVintage(pid: &str, collectionName: &str){

  //TODO create dest dir if not exist?... wait .... it deos htat....

  //a "collection root" is the same as a vintageDir.  not to be confused with the collectionParentRoot which contains all the vintages for collection

  // get current collection root SOURCE_DIR
  // let sourcePath = format!("{}/", getCollectionVintageRoot(collectionName));    //ends with slach so rsync copies CONTENTS of the dir, instead of the dir
  let sourcePath = format!("{}/", getDir_collectionVintage(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE));    //ends with slach so rsync copies CONTENTS of the dir, instead of the dir

  let vintage = currentTimeMillis().to_string() + "_busy";

  // determine destination dir = new destination collection root: DESTINATION_DIR
  // let destinationDir = getCollectionVintageRoot_(collectionName, FERRET_COLLECTIONS_ROOT, &vintage);
  let destinationDir = getDir_collectionVintage(collectionName, FERRET_COLLECTIONS_ROOT, pid, &vintage);

  println!("create dir: destinationDir: {}", destinationDir);

  // create destinationDir
  createDirMaybe(&destinationDir);

  removeRocksdbLogsOld(&sourcePath);

  // copy CONTENTS OF sourceDir INTO destinationDIr
  println!("rsync:\n      source: \n{}\n      destination: \n{}", &sourcePath, &destinationDir);
  let mut child = std::process::Command::new("rsync") .arg("-ar")
                                      .arg(&sourcePath)
                                      .arg(&destinationDir)
                                      .spawn()
                                      .expect("in publishNewVintage: rsync command failed ");   //this isn't waiting for it to ifnish!!!!
  child.wait(); //yay

  let renameDirFrom = &destinationDir;
  let renameDirTo = &destinationDir.replace("_busy", "");
  std::process::Command::new("mv").args(&[renameDirFrom, renameDirTo]).spawn().expect("rename failed");     //https://users.rust-lang.org/t/moving-and-renaming-directory/44742/3?u=stuartcrobinson
}




/// TODO how too adapt this to ferret endpoint?  should it read a vintage schema, or squirrel's???  reading squirrel for now
/// irl - it has to be squirrel's!  cos we're prob gonna be writing to squirrel data anyway!  this result should be placed on the scribe's outgoing map
///       hmmmm so that map value needs to be Result not string
pub fn unsetFieldAsSorter(pid: &str, collectionName: &str,  fieldName: &str) -> Result<String, Box<dyn Error>> {
  // first, see if the field is already a sorter. if so, return an error with the right message
  let mut schema = readSchema(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE, false);

  let  fieldInSchema = schema.fields.iter().find(|f| f.name == fieldName);

  let mut isAlreadySorted = false;
  let mut isAlreadyInSchema = true;

  match fieldInSchema {
    Some(f) => isAlreadySorted = f.sortThisGuy,
    None    => isAlreadyInSchema = false
  };
  if !isAlreadySorted || !isAlreadyInSchema{
    return Err(werr(&format!("field {} isn't already a sorter",  fieldName)));
    // return Err(WError { message: format!("field {} isn't already a sorter",  fieldName)});
  }
  //field is in schema -- mark it as not a sorter
  for i in 0..schema.fields.len() {
    let  f = &schema.fields[i];
    if f.name == fieldName {
      let fNew: WorpdriveFieldObject = WorpdriveFieldObject {
        name: fieldName.to_string(),
        searchMe: f.searchMe,
        sortThisGuy: false,
        doIndexPrefixes: f.doIndexPrefixes,
        is_graphable_timestamp: f.is_graphable_timestamp
      };
      schema.fields[i] = fNew;
      break;                                                          //assuming only 1 entry for fieldName!  need to validate this elsewhere?
    }
  }

  for shardName in getShards(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE) {
    let rr = &getDir_rocksRoot_sovabydid(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, &shardName);
    // let sovDir = getParent(rr);
    std::fs::remove_dir_all(rr);
  }
  writeSchema(pid, collectionName, &schema);
  publishNewVintage(pid, collectionName);   //a "vintage" by definition is stored by/with/for a ferret, i think
  Ok(serde_json::to_string_pretty(&schema)?)
}

/// TODO how too adapt this to ferret endpoint?  should it read a vintage schema, or squirrel's???  reading squirrel for now
/// irl - it has to be squirrel's!  cos we're prob gonna be writing to squirrel data anyway!  this result should be placed on the scribe's outgoing map
///       hmmmm so that map value needs to be Result not string
pub fn setFieldAsSorter(pid: &str, collectionName: &str,  fieldName: &str) -> Result<String, Box<dyn Error>> {
  // first, see if the field is already a sorter. if so, return an error with the right message
  let mut schema = readSchema(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE, false);

  let  fieldInSchema = schema.fields.iter().find(|f| f.name == fieldName);

  let mut isAlreadySorted = false;
  let mut isAlreadyInSchema = true;

  match fieldInSchema {
    Some(f) => isAlreadySorted = f.sortThisGuy,
    None    => isAlreadyInSchema = false
  };
  if isAlreadySorted {
    return Err(werr(&format!("field {} is already a sorter",  fieldName)));
    // return Err(WError { message: format!("field {} is already a sorter",  fieldName)});
  }
  if !isAlreadyInSchema {                                //first, add the field to the schema if not there
    let f: WorpdriveFieldObject = WorpdriveFieldObject {
      name: fieldName.to_string(),
      searchMe: false,
      sortThisGuy: true,
      doIndexPrefixes: false,
      is_graphable_timestamp: false
    };
    schema.fields.push(f);
  }
  else {                                                       //else if it IS in the schema, update the schema to show it's a sorter now!
    for i in 0..schema.fields.len() {
      let  f = &schema.fields[i];
      if f.name == fieldName {
        // println!("found field {}", fieldName);
        // println!("before f obj: {:?}", f);
        let fNew: WorpdriveFieldObject = WorpdriveFieldObject {
          name: fieldName.to_string(),
          searchMe: f.searchMe,
          sortThisGuy: true,
          doIndexPrefixes: f.doIndexPrefixes,
          is_graphable_timestamp: f.is_graphable_timestamp
        };
        // println!("fNew obj: {:?}", fNew);
        schema.fields[i] = fNew;
        break;                                                          //assuming only 1 entry for fieldName!  need to validate this elsewhere?
      }
    }
  }
  //now schema is updated showing the change we want to see in the world.  now be the change.  ugh this fx is too long already
  writeSchema(pid, collectionName, &schema);

  let m_docId_doc = getDocsWithSingleField(pid, collectionName, fieldName);

  let isFieldReindexing = true;
  let allowSort = true;
  let allowTimeindexing = false;
  let allowPrefix = false;
  let allowSearch = false;

  indexBulk_docsMap(pid, collectionName, m_docId_doc, isFieldReindexing, allowSort, allowTimeindexing, allowPrefix, allowSearch)
}

pub fn unsetFieldAsTimegrapher(pid: &str, collectionName: &str,  fieldName: &str) -> Result<String, Box<dyn Error>> {
  let mut schema = readSchema(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE, false);

  let  fieldInSchema = schema.fields.iter().find(|f| f.name == fieldName);

  let mut isAlreadyTivabydized = false;
  let mut isAlreadyInSchema = true;

  match fieldInSchema {
    Some(f) => isAlreadyTivabydized = f.is_graphable_timestamp,
    None    => isAlreadyInSchema = false
  };
  if !isAlreadyTivabydized || !isAlreadyInSchema{
    return Err(werr(&format!("field {} isn't already tivabydized",  fieldName)));
    // return Err(WError { message: format!("field {} isn't already tivabydized",  fieldName)});
  }
  //field is in schema -- mark it as not a timegrapher
  for i in 0..schema.fields.len() {
    let  f = &schema.fields[i];
    if f.name == fieldName {
      let fNew: WorpdriveFieldObject = WorpdriveFieldObject {
        name: fieldName.to_string(),
        searchMe: f.searchMe,
        sortThisGuy: f.sortThisGuy,
        doIndexPrefixes: f.doIndexPrefixes,
        is_graphable_timestamp: false
      };
      schema.fields[i] = fNew;
      break;                                                          //assuming only 1 entry for fieldName!  need to validate this elsewhere?
    }
  }

  for shardName in getShards(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE) {
    let rr = &getDir_rocksRoot_tivabydid(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, &shardName);
    std::fs::remove_dir_all(rr);
  }
  writeSchema(pid, collectionName, &schema);
  publishNewVintage(pid, collectionName);   //a "vintage" by definition is stored by/with/for a ferret, i think

  Ok(serde_json::to_string_pretty(&schema)?)
}


pub fn setFieldAsTimegrapher(pid: &str, collectionName: &str,  fieldName: &str) -> Result<String, Box<dyn Error>> {
  // first, see if the field is already a sorter. if so, return an error with the right message
  let mut schema = readSchema(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE, false);

  let  fieldInSchema = schema.fields.iter().find(|f| f.name == fieldName);

  let mut isAlreadyTivabydized = false;
  let mut isAlreadyInSchema = true;

  match fieldInSchema {
    Some(f) => isAlreadyTivabydized = f.is_graphable_timestamp,
    None    => isAlreadyInSchema = false
  };
  if isAlreadyTivabydized {
    return Err(werr(&format!("field {} is already tivabydized",  fieldName)));
    // return Err(WError { message: format!("field {} is already tivabydized",  fieldName)});
  }
  if !isAlreadyInSchema {                                //first, add the field to the schema if not there
    let f: WorpdriveFieldObject = WorpdriveFieldObject {
      name: fieldName.to_string(),
      searchMe: false,
      sortThisGuy: false,
      doIndexPrefixes: false,
      is_graphable_timestamp: true
    };
    schema.fields.push(f);
  }
  else {                                                       //else if it IS in the schema, update the schema to show it's a tivabydizer now!
    for i in 0..schema.fields.len() {
      let  f = &schema.fields[i];
      if f.name == fieldName {
        // println!("found field {}", fieldName);
        // println!("before f obj: {:?}", f);
        let fNew: WorpdriveFieldObject = WorpdriveFieldObject {
          name: fieldName.to_string(),
          searchMe: f.searchMe,
          sortThisGuy: f.sortThisGuy,
          doIndexPrefixes: f.doIndexPrefixes,
          is_graphable_timestamp: true
        };
        // println!("fNew obj: {:?}", fNew);
        schema.fields[i] = fNew;
        break;                                                          //assuming only 1 entry for fieldName!  need to validate this elsewhere?
      }
    }
  }
  //now schema is updated showing the change we want to see in the world.  now be the change.  ugh this fx is too long already
  writeSchema(pid, collectionName, &schema);

  let  m_docId_doc = getDocsWithSingleField(pid, collectionName, fieldName);
  println!("setFieldAsTimegrapher: m_docId_doc: {:?}", m_docId_doc);

  let isFieldReindexing = true;
  let allowSort = false;
  let allowTimeindexing = true;
  let allowPrefix = false;
  let allowSearch = false;

  indexBulk_docsMap(pid, collectionName, m_docId_doc, isFieldReindexing, allowSort, allowTimeindexing, allowPrefix, allowSearch)
}


pub fn indexWithPrefixes(pid: &str, collectionName: &str,  fieldName: &str) -> Result<String, Box<dyn Error>> {
  let mut schema = readSchema(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE, false);

  let  fieldInSchema = schema.fields.iter().find(|f| f.name == fieldName);

  let mut isAlreadyPrefixed = false;
  let mut isAlreadyInSchema = true;

  match fieldInSchema {
    Some(f) => isAlreadyPrefixed = f.doIndexPrefixes,
    None    => isAlreadyInSchema = false
  };
  if isAlreadyPrefixed {
    return Err(werr(&format!("field {} is already prefixed",  fieldName)));
  }
  if !isAlreadyInSchema {                                //first, add the field to the schema if not there
    let f: WorpdriveFieldObject = WorpdriveFieldObject {
      name: fieldName.to_string(),
      searchMe: true,
      sortThisGuy: false,
      doIndexPrefixes: true,
      is_graphable_timestamp: false
    };
    schema.fields.push(f);
  }
  else {                                                       //else if it IS in the schema, update the schema to show it's a tivabydizer now!
    for i in 0..schema.fields.len() {
      let  f = &schema.fields[i];
      if f.name == fieldName {
        // println!("found field {}", fieldName);
        // println!("before f obj: {:?}", f);
        let fNew: WorpdriveFieldObject = WorpdriveFieldObject {
          name: fieldName.to_string(),
          searchMe: true,
          sortThisGuy: f.sortThisGuy,
          doIndexPrefixes: true,
          is_graphable_timestamp: f.is_graphable_timestamp
        };
        // println!("fNew obj: {:?}", fNew);
        schema.fields[i] = fNew;
        break;                                                          //assuming only 1 entry for fieldName!  need to validate this elsewhere?
      }
    }
  }
  //now schema is updated showing the change we want to see in the world.  now be the change.  ugh this fx is too long already
  writeSchema(pid, collectionName, &schema);

  let  m_docId_doc = getDocsWithSingleField(pid, collectionName, fieldName);
  println!("indexWithPrefixes: m_docId_doc: {:?}", m_docId_doc);

  let isFieldReindexing = true;
  let allowSort = false;
  let allowTimeindexing = false;
  let allowPrefix = true;
  let allowSearch = true;

  indexBulk_docsMap(pid, collectionName, m_docId_doc, isFieldReindexing, allowSort, allowTimeindexing, allowPrefix, allowSearch)
}


//TODO these two fxs!!!

pub fn unindexFieldInvinds(pid: &str, collectionName: &str,  fieldName: &str) -> Result<String, Box<dyn Error>> {

  let mut schema = readSchema(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE, false);

  let  fieldInSchema = schema.fields.iter().find(|f| f.name == fieldName);

  let mut isAlreadyInvinded = false;
  let mut isAlreadyInSchema = true;

  match fieldInSchema {
    Some(f) => isAlreadyInvinded = f.searchMe,
    None    => isAlreadyInSchema = false
  };
  if !isAlreadyInvinded || !isAlreadyInSchema{
    // return Err(WError { message: format!("field {} isn't already invinded",  fieldName)});
    return Err(werr(&format!("field {} isn't already invinded",  fieldName)));
  }
  //field is in schema -- mark it as not for searching at all
  for i in 0..schema.fields.len() {
    let  f = &schema.fields[i];
    if f.name == fieldName {
      let fNew: WorpdriveFieldObject = WorpdriveFieldObject {
        name: fieldName.to_string(),
        searchMe: false,
        sortThisGuy: f.sortThisGuy,
        doIndexPrefixes: false,
        is_graphable_timestamp: f.is_graphable_timestamp
      };
      schema.fields[i] = fNew;
      break;                                                          //assuming only 1 entry for fieldName!  need to validate this elsewhere?
    }
  }

  for shardName in getShards(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE) {
    let rr = &getDir_rocksRoot_invind(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, &shardName, &fieldName);
    std::fs::remove_dir_all(rr);
  }
  writeSchema(pid, collectionName, &schema);
  publishNewVintage(pid, collectionName);   //a "vintage" by definition is stored by/with/for a ferret, i think
  Ok(serde_json::to_string_pretty(&schema)?)

}

pub fn indexTokensWithoutPrefixes(pid: &str, collectionName: &str,  fieldName: &str) -> Result<String, Box<dyn Error>> {
  let mut schema = readSchema(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE, false);

  let  fieldInSchema = schema.fields.iter().find(|f| f.name == fieldName);

  let mut isAlreadyInvinded = false;
  let mut isAlreadyInSchema = true;

  match fieldInSchema {
    Some(f) => isAlreadyInvinded = f.searchMe,
    None    => isAlreadyInSchema = false
  };
  if isAlreadyInvinded {
    return Err(werr(&format!("field {} is already invinded",  fieldName)));
  }
  if !isAlreadyInSchema {                                //first, add the field to the schema if not there
    let f: WorpdriveFieldObject = WorpdriveFieldObject {
      name: fieldName.to_string(),
      searchMe: true,
      sortThisGuy: false,
      doIndexPrefixes: false,
      is_graphable_timestamp: false
    };
    schema.fields.push(f);
  }
  else {                                                       //else if it IS in the schema, update the schema to show it's a tivabydizer now!
    for i in 0..schema.fields.len() {
      let  f = &schema.fields[i];
      if f.name == fieldName {
        // println!("found field {}", fieldName);
        // println!("before f obj: {:?}", f);
        let fNew: WorpdriveFieldObject = WorpdriveFieldObject {
          name: fieldName.to_string(),
          searchMe: true,
          sortThisGuy: f.sortThisGuy,
          doIndexPrefixes: f.doIndexPrefixes,
          is_graphable_timestamp: f.is_graphable_timestamp
        };
        // println!("fNew obj: {:?}", fNew);
        schema.fields[i] = fNew;
        break;                                                          //assuming only 1 entry for fieldName!  need to validate this elsewhere?
      }
    }
  }
  //now schema is updated showing the change we want to see in the world.  now be the change.  ugh this fx is too long already
  writeSchema(pid, collectionName, &schema);

  let  m_docId_doc = getDocsWithSingleField(pid, collectionName, fieldName);
  println!("indexWithPrefixes: m_docId_doc: {:?}", m_docId_doc);

  let isFieldReindexing = true;
  let allowSort = false;
  let allowTimeindexing = false;
  let allowPrefix = false;
  let allowSearch = true;

  indexBulk_docsMap(pid, collectionName, m_docId_doc, isFieldReindexing, allowSort, allowTimeindexing, allowPrefix, allowSearch)
}

pub fn unindexPrefixesOnly(pid: &str, collectionName: &str,  fieldName: &str) -> Result<String, Box<dyn Error>> {

  let mut schema = readSchema(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE, false);

  let  fieldInSchema = schema.fields.iter().find(|f| f.name == fieldName);

  let mut isAlreadyPrefixed = false;
  let mut isAlreadyInSchema = true;

  match fieldInSchema {
    Some(f) => isAlreadyPrefixed = f.doIndexPrefixes,
    None    => isAlreadyInSchema = false
  };
  if !isAlreadyPrefixed || !isAlreadyInSchema{
    // return Err(werr(&format!("field {} isn't already prefixed",  fieldName));
    // return Err(Box::new(WError { message: format!("field {} isn't already prefixed",  fieldName)}));
    return Err(werr(&format!("field {} isn't already prefixed",  fieldName)));
    // return Err(Box::new("hi"));
    // return Err();
  }
  //field is in schema -- mark it as not for prefixes
  for i in 0..schema.fields.len() {
    let  f = &schema.fields[i];
    if f.name == fieldName {
      let fNew: WorpdriveFieldObject = WorpdriveFieldObject {
        name: fieldName.to_string(),
        searchMe: f.searchMe,
        sortThisGuy: f.sortThisGuy,
        doIndexPrefixes: false,
        is_graphable_timestamp: f.is_graphable_timestamp
      };
      schema.fields[i] = fNew;
      break;                                                          //assuming only 1 entry for fieldName!  need to validate this elsewhere?
    }
  }

  for shardName in getShards(SQUIRREL_COLLECTIONS_ROOT, pid, collectionName, DEFAULT_VINTAGE) {
    let rr = &getDir_rocksRoot_invind(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, &shardName, &fieldName);
    let iiDb = &openDb(rr);
    deleteAllPrefixes(iiDb);
  }
  writeSchema(pid, collectionName, &schema);
  publishNewVintage(pid, collectionName);   //a "vintage" by definition is stored by/with/for a ferret, i think

  Ok(serde_json::to_string_pretty(&schema)?)

}


fn getDocsWithSingleField(pid: &str, collectionName: &str, fieldName: &str) -> BTreeMap<usize, String> {
  let mut m_docId_doc: BTreeMap<usize, String> = BTreeMap::new();
  let docsRdb = &openDb(&getDir_rocksRoot_docs(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE));
  let docIds = getKeysAsIntsSortedIncreasing_(docsRdb);
  for docId in docIds {
    let docStr = readDoc_(docsRdb, docId);
    let docJson: BTreeMap<String, Value> = serde_json::from_str(&docStr).unwrap();

    match docJson.get(fieldName) {
      Some(fVal) => {
        let mut docMap: HashMap<String, &Value> = HashMap::new();
        docMap.insert(st(fieldName), fVal);
        let newDocStr = serde_json::to_string(&docMap).unwrap();
        m_docId_doc.insert(docId, newDocStr);
      },
      None => {}
    }
  }
  m_docId_doc
}

pub fn getShardName(docId: usize, docJson: &BTreeMap<String, Value>, schema: &WorpdriveSchema) -> String {
  getShardNameForDocId(docId,  schema.is_worptail,  if schema.is_worptail { docJson["_timestamp"].as_u64().unwrap() } else {0},  schema.max_docs_per_shard)
}

pub fn getShardNameForDocId(docId: usize, isWorptail: bool, docTimestamp: u64, maxShardLen: usize) -> String {
  let shardName: String;
  if isWorptail {
    shardName = getShardNameFromTimestamp(docTimestamp);
  } else {
    let shardNumber = getShardNumberForDocId(maxShardLen, docId as usize);
    shardName = shardNumber.to_string();
  }
  // println!("docId, timestamp, shardName: {}  {}  {}", docId, docTimestamp, shardName);

  shardName
}


/// nothing here should ever use QUERY_MODE cos we're opening all dbs manually and passing around right .... ?
/// this shoudl work from ferret cos ferret in different clxnsRoot ... so its ROCKSROOT map shouldn't incldue any of these rocksdbs anyway...
pub fn indexDocInCollection(pid: &str, collectionName: &str,  docJsonStr: &str) -> usize {
  let startTotal = std::time::Instant::now();

  let metaRdb = &openDb(&getDir_rocksRoot_meta(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE));
  let docsRdb = &openDb(&getDir_rocksRoot_docs(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE));
  let locsRdb = &openDb(&getDir_rocksRoot_locs(collectionName, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE));

  println!("time to open rdbs: {:?}", startTotal.elapsed());                        //72 ms to open all 4

  let mut docJson = getDocJson(docJsonStr);

  let docIds = getKeysAsIntsSortedIncreasing_(docsRdb);
  let docId = findMissingOrNextNumber(&docIds);               // use docId graveyard instead?

  let mut schema = readSchema_(metaRdb);
  println!("schema:\n {}", serde_json::to_string_pretty(&schema).unwrap());

  let maxShardLen = schema.max_docs_per_shard;
  let isWorptail = schema.is_worptail;
  let mut docTimestamp = 0 as u64;                    //only actually used for worptail  // USE DOC"S TIMESTMP!!!!
  let mut schema_was_changed = false;

  if isWorptail {
    let fields = &schema.fields;
    let timestampField = &fields.into_iter().find(|f| f.is_graphable_timestamp).unwrap().name.to_string();
    if !docJson.contains_key(timestampField) {
      docTimestamp = getEpochMs();
      docJson.insert(st(timestampField), serde_json::json!(docTimestamp));
    }
    else {
      docTimestamp = docJson[timestampField].as_u64().unwrap();
    }
  }

  // println!("docJson: {:?}", docJson);
  println!("docJson: {}", serde_json::to_string(&docJson).unwrap());

  writeDoc_(docId, &serde_json::to_string(&docJson).unwrap(), docsRdb, true);   // works
  // writeDoc_(docId, docJsonStr, docsRdb, true);
  writeNumDocs_(docIds.len() + 1, metaRdb);

  for (fieldName, fieldValue) in docJson {

    if !schemaContainsField(&schema, &fieldName) {
      schema = updateSchemaWithNewField(schema, &fieldName);                    //so when does updated schema get written?
      schema_was_changed = true;
    }

    let fields = &schema.fields;

    let isForSorting = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].sortThisGuy;
    let isForTimegraphing = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].is_graphable_timestamp;
    let isForPrefixes = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].doIndexPrefixes;
    let isForSearching = fields.into_iter().filter(|f| f.name == fieldName).collect::<Vec<&WorpdriveFieldObject>>()[0].searchMe;

    println!("indexDocInCollection:: docId ({}), field ({}), fVal: {}, doPrefix: {}, isForTimegraphing: {}", docId, fieldName, fieldValue, isForPrefixes, isForTimegraphing);

    if isForSearching || isForSorting || isForTimegraphing {
      indexDocField(pid, collectionName, &fieldName, &fieldValue, docId, isForSorting, isForTimegraphing, isForPrefixes, isForSearching, maxShardLen, false, locsRdb, isWorptail, docTimestamp);
    }
  }

  if schema_was_changed {
    writeSchema_(&schema, metaRdb)
  }

  publishNewVintage(pid, collectionName);   //a "vintage" by definition is stored by/with/for a ferret, i think
  println!("{:?}", startTotal.elapsed());
  docId
}

fn getShardNameFromTimestamp(docTimestamp: u64) -> String {
  // https://docs.rs/chrono/0.4.19/chrono/
  // let milliseconds = (docTimestamp % 1000) as usize;
  let seconds = (docTimestamp / 1000) as i64;
  let dt = Utc.timestamp(seconds, 0);
  format!("{}_{}", dt.year(), dt.month())
}

///index this new document & update all relevant ferrets.  index must already exist.  shard might not
pub fn indexDocField(pid: &str, collection: &str, fieldName: &str, fieldValue: &Value, docId: usize, isForSorting: bool, isForTimegraphing: bool, doIndexPrefixes: bool, isForSearching: bool,
                                                        maxShardLen: usize,
                                                        verbose: bool,
                                                        locsRdb: &rocksdb::DB,
                                                        isWorptail: bool,
                                                        docTimestamp: u64)   {
  if verbose {println!("collection: {}, fieldName: {}, fieldValue: {}", fieldName, fieldName, fieldValue );}

  let shardName = getShardNameForDocId(docId, isWorptail, docTimestamp, maxShardLen);

  //problem:  this function gets called from ferret query which is in query_mode == true ....
  // so since query_mode == true, these fxs try to open all rocksdbs connections and put in map.  cant do that cos we're already holding the connections open this fx
  // how to resolve?? turns out that QUERY_MODE as a global static is a bad idea.  just like everyone said.  gut it onw? or work around...
  // let's work around for now .... sloppy

  let invindsRocksRoot = getDir_rocksRoot_invind(collection, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, &shardName, fieldName);
  createAndLoadNewRocksDbMaybe(&invindsRocksRoot, false);     //this opens an rdb connection only if it doens't exist yet so it should be okay
  let invindsRdb = &openDb(&invindsRocksRoot);

  let sovabydidsRocksRoot = getDir_rocksRoot_sovabydid(collection, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, &shardName);
  createAndLoadNewRocksDbMaybe(&sovabydidsRocksRoot, false);     //this opens an rdb connection only if it doens't exist yet so it should be okay
  let sovabydidsRdb = &openDb(&sovabydidsRocksRoot);

  let tivabydidsRocksRoot = getDir_rocksRoot_tivabydid(collection, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, &shardName);
  createAndLoadNewRocksDbMaybe(&tivabydidsRocksRoot, false);     //this opens an rdb connection only if it doens't exist yet so it should be okay
  let tivabydidsRdb = &openDb(&tivabydidsRocksRoot);

  //why is this a separate fx?
  indexDocFieldValue_worker(collection, fieldName, fieldValue,  verbose, docId,  isForSorting, isForTimegraphing, doIndexPrefixes, isForSearching, &shardName, invindsRdb, locsRdb, sovabydidsRdb, tivabydidsRdb);
}

pub fn destroy_everything(){
  std::fs::remove_dir_all(SQUIRREL_COLLECTIONS_ROOT);
  std::fs::remove_dir_all(FERRET_COLLECTIONS_ROOT);
  resetStaticMaps_clxnsRoot(SQUIRREL_COLLECTIONS_ROOT, false);
  std::fs::remove_dir_all(CRITTERS_ROOT);
}

fn sum(nums: Vec<u16>) -> u16 {
  let mut sum: usize = 0;
  for num in nums {
    sum += num as usize;
  }
  min(u16::MAX as usize, sum) as u16
}


fn insertSortValues(v__docId_combinedRelevance: Vec<(u32, u16)>, m__docId_sortValue: HashMap<u32, f32> ) -> Vec<(u32, u16, f32)> {
  let mut v__docId_combinedRelevance_sortValue: Vec<(u32, u16, f32)> = Vec::new();

  for (docId, cr) in v__docId_combinedRelevance {
    let sv = m__docId_sortValue.get(&docId).unwrap();
    v__docId_combinedRelevance_sortValue.push((docId, cr, *sv));
  }
  v__docId_combinedRelevance_sortValue
}

fn aggregateRelevances(m__docId_relevances:   HashMap<u32, Vec<u16>> ) ->  Vec<(u32, u16)> {
  // combine the relevances per docIds
  let mut v__docId_combinedRelevance: Vec<(u32, u16)> = Vec::new();
  for (docId, relevances) in m__docId_relevances {
    let summedRelevances = sum(relevances);
    v__docId_combinedRelevance.push((docId, summedRelevances));                                                 // this might mess up pagination.  think about it.
  }
  v__docId_combinedRelevance.sort_by(|a, b| b.1.cmp(&a.1));   //sort by relevance decreasing
  v__docId_combinedRelevance
}

fn  get_m_field_docHitToks(docId: u32, m__docId_hitFields: &HashMap<u32, Vec<String>>, m_field_queryCookedToks: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {  //docId: u32, m__docId_hitFields: &HashMap<u32, Vec<String>>,

  let mut m_field_docHitToks: HashMap<String, Vec<String>> = HashMap::new();

  for hitFields in m__docId_hitFields.get(&docId){
    for f in hitFields {
      let toks = m_field_queryCookedToks.get(f).unwrap();
      m_field_docHitToks.insert(f.to_string(), toks.to_vec());
    }
  }
  m_field_docHitToks
}

      //dfLocationsDescending is locations of ALL HITTOKS for this field... but we need locs per hitTok...
      // so like ... m_hitTok_locsDescending ?  (what's a hitTok ?  that's a queryCookedTok)
// m_hitTok_locsDescending

fn get_doc_field_locs_descending(clxnsRoot: &str, pid: &str, collectionName: &str, docId: usize, field: &str, dFHitToks: &Vec<String>, doPrefixLastTok: bool, vintage: &str) -> (Vec<usize>, HashMap<String, Vec<u16>>){
  let mut dfLocations: Vec<u16> = Vec::new();

  let mut m_hitTok_dfLocs: HashMap<String,Vec<u16>> = HashMap::new();

  for i in 0..dFHitToks.len() {
    let dFHitTok = &dFHitToks[i];
    // println!("dFHitTok: {}", dFHitTok);
    let mut dfhtLocations = readLocations(clxnsRoot, pid, collectionName, field, docId, &dFHitTok, false, vintage);
    // println!("before prefix read dfhtLocations: {:?}", dfhtLocations);
    if i == dFHitToks.len()-1 && doPrefixLastTok {                                                              //if it's the last token in the query, do prefix search
      dfhtLocations.extend(readLocations(clxnsRoot, pid, collectionName, field, docId, &dFHitTok, true, vintage));
    }
    m_hitTok_dfLocs.insert(st(dFHitTok), dfhtLocations.to_vec());

    // println!("after prefix read dfhtLocations: {:?}", dfhtLocations);
    dfLocations.extend(dfhtLocations);
  }
  dfLocations.sort();  dfLocations.reverse();                                                         //this *should* be redundant cos done during indexing too.  should it b in one or the other? which?  //these 2 lines teogehr are faster than sort_by https://stackoverflow.com/a/60916195/8870055
  // println!("dfLocations: {:?}", dfLocations);

  let locsDescending = dfLocations;
  let mut locsDescendingUsize: Vec<usize> = Vec::new();
  for loc in locsDescending {
    locsDescendingUsize.push(loc as usize);
  }
  (locsDescendingUsize, m_hitTok_dfLocs)
}



pub fn ascendingComparator_3ple(left: &(u32, u16, f32), right:  &(u32, u16, f32) ) -> Ordering {

  if left.2.is_nan() {
    return Ordering::Greater                                          // put it at the end
  }
  if right.2.is_nan() {
    return Ordering::Less                                          // put it at the end
  }

  left.2.partial_cmp(&right.2)
                              .unwrap()
                              .then(Ord::cmp(&left.0, &right.0))
}

pub fn descendingComparator_3ple(left: &(u32, u16, f32), right:  &(u32, u16, f32) ) -> Ordering {

  if left.2.is_nan() {
    return Ordering::Greater                                          // put it at the end
  }
  if right.2.is_nan() {
    return Ordering::Less                                          // put it at the end
  }

  right.2.partial_cmp(&left.2)
                              .unwrap()
                              .then(Ord::cmp(&left.0, &right.0))    // dont reverse the relevance comparator
}


pub fn ascendingComparator_2ple(left: &(u32, f32), right:  &(u32, f32) ) -> Ordering {

  if left.1.is_nan() {
    return Ordering::Greater                                          // put it at the end
  }
  if right.1.is_nan() {
    return Ordering::Less                                          // put it at the end
  }

  left.1.partial_cmp(&right.1)
                              .unwrap()
                              // .then(Ord::cmp(&left.0, &right.0))
}

pub fn descendingComparator_2ple(left: &(u32, f32), right:  &(u32, f32) ) -> Ordering {

  if left.1.is_nan() {
    return Ordering::Greater                                          // put it at the end
  }
  if right.1.is_nan() {
    return Ordering::Less                                          // put it at the end
  }

  right.1.partial_cmp(&left.1)
                              .unwrap()
                              // .then(Ord::cmp(&left.0, &right.0))    // dont reverse the relevance comparator
}



fn sortResultsExact(mut v__docId_combinedRelevance_sortValue: Vec<(u32, u16, f32)>, sortDescending: bool) ->  Vec<(u32, u16, f32)>{

//m__docId_sortValue: HashMap<u32, f32>,

  // let s = readSovabydid(collectionName, sortField);
  if sortDescending {                                                                                           //https://rosettacode.org/wiki/Sort_using_a_custom_comparator#Java
    // v__docId_combinedRelevance.sort_unstable_by(|left, right|  descendingComparator(&s, *left, *right)    );
    println!("v__docId_combinedRelevance_sortValue: {:?}", v__docId_combinedRelevance_sortValue);

    // v__docId_combinedRelevance_sortValue.sort_unstable_by(|b, a| a.2.partial_cmp(&b.2).unwrap());    //.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v__docId_combinedRelevance_sortValue.sort_unstable_by(descendingComparator_3ple);    //.sort_by(|a, b| a.partial_cmp(b).unwrap());
  } else {
    // v__docId_combinedRelevance.sort_unstable_by(|left, right|  ascendingComparator(&s, *left, *right)    );
    // v__docId_combinedRelevance_sortValue.sort_unstable_by(|a, b| a.2.partial_cmp(&b.2).unwrap());    //.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v__docId_combinedRelevance_sortValue.sort_unstable_by(ascendingComparator_3ple);    //.sort_by(|a, b| a.partial_cmp(b).unwrap());
  }
  v__docId_combinedRelevance_sortValue
}

fn build__m_phraseTok_children(m_field_queryCookedToksOrig:  &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>>{

        //      - for each multi-word (space-containing) origTok:
        //          - get all newbornSubphrases (do this early, before loops)
        //          - put these in m_phraseTok_children.............................. why
        //          - and then flip that to build m_subPhrase_parentPhrase

  let mut m_phraseTok_children: HashMap<String, Vec<String>> = HashMap::new();

  for (_f, toks) in m_field_queryCookedToksOrig {
    for qctOrig in toks {
      if qctOrig.contains(&st(" ")){
        let subphrases = splitUpLongQueryPhrase(qctOrig);
        m_phraseTok_children.insert(st(qctOrig), subphrases);
      }
    }
  }
  m_phraseTok_children
}

fn build__m_subPhrase_parentPhrase(m_phraseTok_children:  &HashMap<String, Vec<String>>) -> HashMap<String, String> {
  let mut m_subPhrase_parentPhrase: HashMap<String, String> = HashMap::new();
  for (phrase, subphrases) in m_phraseTok_children {
    for subphrase in subphrases {
      m_subPhrase_parentPhrase.insert(st(subphrase), st(phrase));
    }
  }
  m_subPhrase_parentPhrase
}

fn displayPageNResults(clxnsRoot: &str,  pid: &str, collectionName: &str, numResultsPerPage: usize,  pageNumber: usize,
                                                                                          fieldsToReturn:                       Vec<String>,
                                                                                          doHighlight_tagged:                   bool,
                                                                                          doHighlight_objects:                  bool,
                                                                                          doHighlight_map:                      bool,
                                                                                          doRemoveRelevances:                   bool,
                                                                                          leftHighlightTag:                     &str,
                                                                                          rightHighlightTag:                    &str,
                                                                                          minHighlightContext:                  usize,
                                                                                          // maxHighlightContext:                  usize,
                                                                                          MAX_SNIPPETS_CHAR:                    usize,
                                                                                          m_field_queryCookedToks:              HashMap<String, Vec<String>>,
                                                                                          m_field_queryCookedToksOrig:          HashMap<String, Vec<String>>,
                                                                                          m_field_doPrefixLastTok:              HashMap<String, bool>,
                                                                                          m__docId_hitFields:                   HashMap<u32, Vec<String>>,
                                                                                          v__docId_combinedRelevance_sortValue: Vec<(u32, u16, f32)>,
                                                                                          vintage:                              &str)             ->  (String, Vec<WorpHitObject>) {
  let mut hitsVec: Vec<WorpHitObject> = Vec::new();
  let mut response = format!("");
  response = format!("{}\n{}", response, format!("in displayPageNResults, m_field_queryCookedToks: {:?}", m_field_queryCookedToks));
  response = format!("{}\n{}", response, format!("Page {}, {} per page:\n{:<7} {:<7}  {:<8}    {}", pageNumber,numResultsPerPage,  "i", "docId", "rel", "matched fields"));
  let mut hitNum = 0;
  let mut resultNumberFrom = (pageNumber - 1) * numResultsPerPage + 1 - 1;                                         // it makes sense okay
  let resultNumberTo = min(pageNumber * numResultsPerPage, v__docId_combinedRelevance_sortValue.len());
  resultNumberFrom = min(resultNumberFrom, resultNumberTo);

  let m_phraseTok_children = build__m_phraseTok_children(&m_field_queryCookedToksOrig);
  let m_subPhrase_parentPhrase = build__m_subPhrase_parentPhrase(&m_phraseTok_children);
  let subPhrases: Vec<String> = m_subPhrase_parentPhrase.keys().cloned().collect();

  println!("fieldsToReturn, doHighlight: {:?}", fieldsToReturn);
  println!("awfsdf v__docId_combinedRelevance_sortValue: {:?}", v__docId_combinedRelevance_sortValue);

  for (docId, combinedRelevance, _sortValue) in v__docId_combinedRelevance_sortValue[resultNumberFrom..resultNumberTo].to_vec() {          // generate highlighted results string here

    let doc = readDoc(clxnsRoot, pid, collectionName, vintage, docId as usize);
    println!("docId {}, combinedRel {}, _sortValue {},  fieldsToReturn: {:?}, read doc: {}", docId, combinedRelevance, _sortValue, fieldsToReturn, doc);
    let m_f_v = getDocJson(&doc);
    println!("m_f_v: {:?}", m_f_v);

    let m_field_docHitToks = get_m_field_docHitToks(docId, &m__docId_hitFields, &m_field_queryCookedToks);
    let mut fieldsAlphabetized: Vec<String> = Vec::new(); for f in m_field_docHitToks.keys() {fieldsAlphabetized.push(f.to_string()); } fieldsAlphabetized.sort();  //TODO this could be better obv
    let mut m_field_snippets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut highlightsVec: Vec<WorpHighlightObject> = Vec::new();
    // let mut hltsObjsArs: Vec<Vec<HashMap<String, String>>> = Vec::new();
    let mut hltsObjsArs: Vec<WorpHighlightObjectsArrayObject> = Vec::new();
    let mut hltsObjsMap: BTreeMap<String, Vec<BTreeMap<String, Value>>> = BTreeMap::new();
    let mut mSource_f_v: HashMap<String, Value> = HashMap::new();                //TODO the value could be string or numeric ???

    for (f, fVal) in &m_f_v {                                                   //this loop is all for highlighting i think? oh maybe filtering phrases too??? how does that work...
      if fieldsToReturn.contains(f) || fieldsToReturn.contains(&st("*")) {
        mSource_f_v.insert(st(f), serde_json::json!(fVal));
      }

      /* is everything from here down for highlighting??? */

      if !m_field_docHitToks.contains_key(f) {
        continue;
      }
      if !fVal.is_string() {                                                                   //wait what about numeric values.... should we just force everytyhing to string here?
        continue;
      }

      println!("docId {},  m_field_docHitToks: {:?}", docId, m_field_docHitToks);
      // m_field_docHitToks: &HashMap<String, Vec<String>>,
      let dFHitToks = m_field_docHitToks.get(f).unwrap();
      if dFHitToks.len() == 0 {
        continue;
      }

      let (snippets, hltsObjsAr) = getSnippetsForDocField(clxnsRoot, pid, collectionName, docId, f, fVal, &dFHitToks, &m_field_doPrefixLastTok, &m_field_queryCookedToksOrig,
        vintage, &subPhrases, &m_subPhrase_parentPhrase, &m_phraseTok_children, minHighlightContext, MAX_SNIPPETS_CHAR, &leftHighlightTag, &rightHighlightTag,
        doHighlight_tagged, doHighlight_objects, doHighlight_map);

      m_field_snippets.insert(st(f), snippets.to_vec());

      highlightsVec.push(WorpHighlightObject{field: st(f), snippets: snippets});
      hltsObjsArs.push(WorpHighlightObjectsArrayObject{field: st(f), content_objects: hltsObjsAr.to_vec()});
      hltsObjsMap.insert(st(f), hltsObjsAr);
      // hltsObjsArs.push(hltsObjsAr);
    }
    hitNum +=1 ;
    response = format!("{}\n{}", response, format!("{:<7} {:<7}  {:<8}    {:?}", hitNum, docId, combinedRelevance, m_field_snippets));

    println!("huh mSource_f_v: {:?}", mSource_f_v);

    let worpHitObject = WorpHitObject {id: docId as usize, score: combinedRelevance as usize,
                                                           source: mSource_f_v,
                                                           highlights_objects:  if doHighlight_objects  {Some(hltsObjsArs)} else {None},
                                                           highlights_tagged:   if doHighlight_tagged   {Some(highlightsVec)} else {None},
                                                           highlights_map:       if doHighlight_map      {Some(hltsObjsMap)} else {None} };
    hitsVec.push(worpHitObject);
  }
  (response, hitsVec)
}


pub fn fixOverlaps(tuples: HashSet<(usize, usize)>) -> Vec<(usize, usize)> {
  // okay um... this is tricky ... line them all up by ... startLoc?  and then for each, look at the next one...
  //          is the next one's start or stop between the curr start and stop, inclusive? if so, combine them into a single hlt.
  //          that means removing both of the other, and inserting a new one.  ? i think

  let mut highlights: Vec<(usize, usize)> = tuples.into_iter().collect();
  highlights.sort_by_key(|k| k.0);
  // println!("fixOverlaps: highlights init: {:?} ",highlights);

  if highlights.len() > 1 {

    fn doOverlap(p: (usize, usize), c: (usize, usize)) -> bool {
      (c.0 <= p.1 && c.0 >= p.0) ||
      (c.1 <= p.1 && c.1 >= p.0) ||
      (p.0 <= c.1 && p.0 >= c.0) ||
      (p.1 <= c.1 && p.1 >= c.0)
    }
    fn mergeHlts(p: (usize, usize), c: (usize, usize)) -> (usize, usize) {
      (min(p.0, c.0), max(p.1, c.1))
    }

    // let mut anOverlapExists = true;
    loop {
      for i in 1..highlights.len() {
        // anOverlapExists = false;
        let prevHlt = highlights[i-1];
        let currHlt = highlights[i];
        if doOverlap(prevHlt, currHlt){
          // anOverlapExists = true;
          // println!("overlapped! {:?}  {:?}", prevHlt, currHlt);
          let newHlt = mergeHlts(prevHlt, currHlt);

          // println!("newHlt! {:?} ",newHlt);
          highlights[i-1] = newHlt;
          highlights.remove(i);
          break;
        }
      }
      // anOverlapExists = false;
      break;
    }

    // let mut anOverlapExists = true;
    // while anOverlapExists {
    //   for i in 1..highlights.len() {
    //     anOverlapExists = false;
    //     let prevHlt = highlights[i-1];
    //     let currHlt = highlights[i];
    //     if doOverlap(prevHlt, currHlt){
    //       anOverlapExists = true;
    //       // println!("overlapped! {:?}  {:?}", prevHlt, currHlt);
    //       let newHlt = mergeHlts(prevHlt, currHlt);

    //       // println!("newHlt! {:?} ",newHlt);
    //       highlights[i-1] = newHlt;
    //       highlights.remove(i);
    //       break;
    //     }
    //   }
    //   anOverlapExists = false;
    //   // break;
    // }
  }
  highlights
}

fn getSnippetsForDocField(clxnsRoot: &str, pid: &str, collectionName: &str, docId: u32, f: &str, fVal: &Value, dFHitToks: &Vec<String>,
                                                                                      m_field_doPrefixLastTok:          &HashMap<String, bool>,
                                                                                      m_field_queryCookedToksOrig:      &HashMap<String, Vec<String>>,
                                                                                      vintage:                          &str,
                                                                                      subPhrases:                       &Vec<String>,
                                                                                      m_subPhrase_parentPhrase:         &HashMap<String, String>,
                                                                                      m_phraseTok_children:             &HashMap<String, Vec<String>>,
                                                                                      minHighlightContext: usize,
                                                                                      MAX_SNIPPETS_CHAR: usize,
                                                                                      leftHighlightTag: &str,
                                                                                      rightHighlightTag: &str,
                                                                                      doHighlight_tagged: bool,
                                                                                      doHighlight_objects: bool,
                                                                                      doHighlight_map: bool) -> (Vec<String>, Vec<BTreeMap<String, Value>>) {
  // let dFHitToks = m_field_docHitToks.get(f).unwrap();
  let doPrefixLastTok = m_field_doPrefixLastTok.get(f).unwrap();
  let queryCookedToksOrig = m_field_queryCookedToksOrig.get(f).unwrap();

  println!("dFHitToks: {:?}", dFHitToks);

  // let skipHighlighting = dFHitToks.len() == 0;


  //dfLocationsDescending is locations of ALL HITTOKS for this field... but we need locs per hitTok...
  // so like ... m_hitTok_locsDescending ?  (what's a hitTok ?  that's a queryCookedTok)
  let (_dfLocationsDescending, m_ht_dfLocs) = get_doc_field_locs_descending(clxnsRoot, pid, collectionName, docId as usize, &f, dFHitToks, *doPrefixLastTok, vintage);
  let mut m_dfht_vtLocPairs: HashMap<String, Vec<(usize, usize)>> = HashMap::new();          //what's this?

  let fVal = fVal.as_str().unwrap().to_string();                                              //only supporting string fVal for snippets right now... or ever right?

  //okay instead of just inserting highlight tags, we need to be buildnig a map of start and stop locaitons ...
  // later this will be used to filter potential phrase matches so this map? sould be indexed by ...
  //    query raw tok???? cos... does cookedTok still contain full phrase? or does it get split up....
  // ok now we've got m_field_queryCookedToksOrig
  //
  // but first we need to get start and stop locations for each cookedTok in m_field_queryCookedToks
  // everythnig will be within in this field loop block ... so no need to map per field... we Do need to map per token... cookedTok at first. and then cookedTokOrig

  println!("m_ht_dfLocs: {:?}", m_ht_dfLocs);
  for (hitTok, dfLocs) in m_ht_dfLocs {
    println!("hitTok, dfLocs: {}, {:?}", hitTok, dfLocs); //fVal[loc as usize..]
    let htLen = hitTok.replace(" ", "").len();                                 //have to take out spaces cos we ignore spaces in a few lines when we walk along raw text to find end location

    for loc in dfLocs {
      //now walk along the string at this location in "fVal"            // we know loc is a char bounday
      let mut countValidCookedTokChars = 0;                              //note - this will break if we do algolia's approach of SOMETIMES indexing periods :/  like for "2.5" but not "cat.hat"
      // println!("fVal[loc as usize..]: {:?}", st(&fVal[loc as usize..])); //
      for (i, c) in fVal[loc as usize..].chars().enumerate() {
        // println!("ht, htlen, i, c:  {}, {}, {}, {}", hitTok, htLen, i, c);
        if c.is_alphanumeric() || SPLIT_KEEP_CHARS.contains(c) {
          countValidCookedTokChars += 1;
        }
        if countValidCookedTokChars == htLen {
          m_dfht_vtLocPairs.entry(hitTok.to_string())
                           .or_insert_with(Vec::new)
                           .push((loc as usize, loc as usize + i));
          break;
        }
      }
    }
  }

  // check if it's in m_subPhrase_parentPhrase AND if all of that parentPhrase's children (m_phraseTok_children) are in dfhToks too - (m_phraseTok_children must b sorted)
  //              - - if so, then we know we need to ONLY highlight matches that overlap with the other children matches
  //              - -       - put parentPhrase in list of dfhParentPhrases for this doc-field
  let mut dfhParentPhrases: Vec<String> = Vec::new();
  for dfhTok in dFHitToks {
    if subPhrases.contains(dfhTok){
      let parentPhrase = m_subPhrase_parentPhrase.get(dfhTok).unwrap();
      let allSiblingSubphrases = &m_phraseTok_children.get(parentPhrase).unwrap();
      if containsAll(dFHitToks, allSiblingSubphrases){                                    //https://stackoverflow.com/questions/64226562/check-if-vec-contains-all-elements-from-another-vec
        dfhParentPhrases.push(parentPhrase.to_string());
      }
    }
  }

  println!("dfhParentPhrases: {:?}", dfhParentPhrases);

  let mut m__cookedTokOrig_locPairs: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

  for dfhTok in dFHitToks {                                                      //first, store the complete start and stop locations for all doc-field hitToks that are full orig toks (not subphrases)
    if queryCookedToksOrig.contains(dfhTok) {
      // println!("dfhTok, m_dfht_vtLocPairs: {}, {:?}", dfhTok, m_dfht_vtLocPairs);
      let locPairs = m_dfht_vtLocPairs.get(dfhTok).expect(&format!("tok [{}] missing in locations map", dfhTok));
      m__cookedTokOrig_locPairs.insert(st(dfhTok), locPairs.to_vec());
    }
  }
  println!("dfhParentPhrases: {:?}", dfhParentPhrases);

  for parentPhrase in dfhParentPhrases {                                    //now, deal with phrases ......
    let children = m_phraseTok_children.get(&parentPhrase).unwrap();
    let firstChildLocPairs = m_dfht_vtLocPairs.get(&children[0]).unwrap();
    let mut fullSubphraseSequenceContenders__locPairsSequences: Vec<Vec<(usize, usize)>> = Vec::new();
    for locPair in firstChildLocPairs {                                                                   // first plant all the first children in the locPairSequences
      fullSubphraseSequenceContenders__locPairsSequences.push(vec![*locPair]);
    }

    // println!("parentPhrase: {}, fullSubphraseSequenceContenders__locPairsSequences: {:?}", parentPhrase, fullSubphraseSequenceContenders__locPairsSequences);
    // if it overlaps with a sequence of subPhrases that make up the entire parentPhrase, then put it in fullMatchedFirstChildLocPairs
    for i in 1..children.len() {
      let sibling = &children[i];                                                    //a subphrase tok
      let thisSiblingsLocPairs = m_dfht_vtLocPairs.get(sibling).unwrap();

      for locPair in thisSiblingsLocPairs {
        let startLoc = locPair.0;
        for k in 0..fullSubphraseSequenceContenders__locPairsSequences.len() {
          let contenderLocPairSequence = &mut fullSubphraseSequenceContenders__locPairsSequences[k];
          let prevLocPair = contenderLocPairSequence[i-1];
          let prevSubphraseEndLoc = prevLocPair.1;
          if startLoc < prevSubphraseEndLoc {
            contenderLocPairSequence.push(*locPair);
          }
        }
      }
      //now, all viable contenders will have length i+1 --- so, delete all elements from fullSubphraseSequenceContenders__locPairsSequences with insufficient length
      fullSubphraseSequenceContenders__locPairsSequences = fullSubphraseSequenceContenders__locPairsSequences.into_iter().filter(|v| v.len() == i+1 ).collect();
    }
    //now, all sequences in fullSubphraseSequenceContenders__locPairsSequences are fully matched instances of parentPhrase ?!?!?!?!?!!!!!????
    // what now ?   build a m__parentPhrase_locPair  map ???  yeah but its called m__cookedTokOrig_locPairs
    for locPairsSequence in fullSubphraseSequenceContenders__locPairsSequences {
      let startLoc = locPairsSequence[0].0;
      let endLoc = locPairsSequence.last().unwrap().1;
      m__cookedTokOrig_locPairs.entry(st(&parentPhrase))
                               .or_insert_with(Vec::new)
                               .push((startLoc, endLoc));
    }
  }

  //m__cookedTokOrig_locPairs is complete????
  //so ...NOW.... uhm... we have pairs.  so it doesn't matter what the actual token is anymore
  // just get the locPairs as a list, sorted INCREASING by STARTLoc.  then insert the highlight tags

  // println!("fVal: {}", fVal);
  println!("m__cookedTokOrig_locPairs: {:?}", m__cookedTokOrig_locPairs);

  let    _highlights: HashSet<(usize, usize)> = m__cookedTokOrig_locPairs.values().cloned().flatten().collect();  //remove dups cos prefixes
  // println!("before fixOverlaps");
  let highlights = fixOverlaps(_highlights);
  // println!("after fixOverlaps");

  let mut highlightsCompleteObjectsArray: Vec<BTreeMap<String, Value>> = Vec::new();
  if doHighlight_objects || doHighlight_map {
    //it makes a highlights obejct array like this:
    /*
        [
          {"is_match": true, "content": "hey "},
          {                  "content": "greg"},
          {"is_match": true, "content": " this is "},
          {                  "content": "greg"},
          {"is_match": true, "content": "ory "},
          {                  "content": "amazing"}
        ]
    */

    let mut prevStop = 0;

    println!("here highlights: {:?}", highlights);
    for (hStart, hStop) in &highlights {
      // for each highlight, push the plain bit before it, and then bold part that's highlighted.  afterwards, we'll tack on the last bit -- oops we forgot to do that!!!

      let mut mapPrePlain: BTreeMap<String, Value> = BTreeMap::new();
      if *hStart > 0 {
        println!("here 000");
        mapPrePlain.insert(st("content"),  serde_json::json!(st(&fVal[prevStop..*hStart])));
        highlightsCompleteObjectsArray.push(mapPrePlain);
      }

      let mut boldPart: BTreeMap<String, Value> = BTreeMap::new();
      boldPart.insert(st("is_match"), serde_json::json!(true));
      // println!("here 001");
      boldPart.insert(st("content"), serde_json::json!(st(&fVal[*hStart..=*hStop])));
      highlightsCompleteObjectsArray.push(boldPart);

      prevStop = *hStop + 1;
    }
    let mut mapPrePlain: BTreeMap<String, Value> = BTreeMap::new();
    // println!("here 002");
    mapPrePlain.insert(st("content"),  serde_json::json!(st(&fVal[prevStop..])));
    highlightsCompleteObjectsArray.push(mapPrePlain);

    // println!("fVal: \n{:?}", fVal);
    // println!("highlightsCompleteObjectsArray: \n", );

    // for m in &highlightsCompleteObjectsArray {
    //   println!("{:?}", m);
    // }
  }

  let mut taggedSnippetsData: Vec<(usize, usize, Vec<(usize, usize)>)> = Vec::new();
  let mut snippets: Vec<String> = Vec::new();

  // ok there's a problem here when a query contains tokens that match on the same word.  like "cheeto chee".  this fails: "assert!(currHlt.0 > prevHlt.1);"

  if doHighlight_tagged {
    // println!("highlights: {:?}", highlights);

    {                       //for sanity
      // build out hltLists, but start at the beginning of the document, not the end.  after this var is built, start at ITS end just to insert the tags
      // start with first highlight.
      //   for each successive highlight, if it's within range of prev, AND their combination would be less than MAX_SNIPPETS_CHARS, then combine these into single highlightsList
      //      else, start a new highlightsList
      //  if total lists chars > MAX_SNIPPETS_CHARS, then break

      //but what if highlights.len == 0?? this happens when empty query.
      let mut hltList: Vec<(usize, usize)> = vec![highlights[0]];
      let mut start = getSpaceBoundary(&fVal, max(0, (highlights[0].0 as i32) - (minHighlightContext as i32)), "left");
      let mut stop = getSpaceBoundary(&fVal, min(fVal.len() as i32, (highlights[0].1 + minHighlightContext) as i32), "right");
      // println!("init hltList: {:?}", hltList);
      let mut lastWasPushed = false;

      let mut i = 1;
      while i < highlights.len() {
        let prevHlt = &hltList.last().unwrap();
        let currHlt = highlights[i];
        let potentialCurrHtlStop = getSpaceBoundary(&fVal, min(fVal.len() as i32, (currHlt.1 + minHighlightContext) as i32), "right");

        let totPotentialNewCombinedSnippetsLen = totalSnippetsLen2(&taggedSnippetsData) + (potentialCurrHtlStop - start);
        println!("i, prevHlt, currHlt, fLen, totPotNewCombSnipsLen, MAX_SNIPPETS_CHAR   {}, {:?}, {:?}, {}, {}, {}", i, prevHlt, currHlt, fVal.len(), totPotentialNewCombinedSnippetsLen, MAX_SNIPPETS_CHAR);

        assert!(currHlt.0 > prevHlt.1);

        if currHlt.0 - prevHlt.1 < minHighlightContext * 2 as usize  &&  totPotentialNewCombinedSnippetsLen <= MAX_SNIPPETS_CHAR {
          //if it's within range of prev, AND their combination would be less than MAX_SNIPPETS_CHARS, then combine these into single highlightsList
          hltList.push(currHlt);
          stop = getSpaceBoundary(&fVal, min(fVal.len() as i32, (currHlt.1 + minHighlightContext) as i32), "right");
          let mut k = i + 1;                                                  //there might be another hlt w/in right-side-context... so, next, fill in all the remaining in-context hlts
          while k < highlights.len() {
            let nextHlt = highlights[k];
            if nextHlt.1 - currHlt.1 < minHighlightContext {
              hltList.push(nextHlt);
              i += 1;
            } else { break; }
            k += 1;
          }
          lastWasPushed = false;
        }
        else if totPotentialNewCombinedSnippetsLen <= MAX_SNIPPETS_CHAR {
          //else, if the new Hlt is too far away from prev, but we're not out of total space yet, then start a new snippet
          taggedSnippetsData.push((start, stop, hltList.to_vec()));
          hltList = vec![currHlt];
          start = getSpaceBoundary(&fVal, max(0, (currHlt.0 as i32) - (minHighlightContext as i32)), "left");
          stop = getSpaceBoundary(&fVal, min(fVal.len() as i32, (currHlt.1 + minHighlightContext) as i32), "right");
          lastWasPushed = false;
        }
        else {
          //else, there's no room for this new higlight.  so just store what we've got and stop looking
          taggedSnippetsData.push((start, stop, hltList.to_vec()));
          lastWasPushed = true;
          break;
        }
        i += 1;
      }
      if !lastWasPushed {
        taggedSnippetsData.push((start, stop, hltList.to_vec()));
      }
    }
    // println!("taggedSnippetsData: {:?}", taggedSnippetsData);

    for (sStart, sStop, hltList) in taggedSnippetsData {                                          // sStart = "snippet start"
      // println!("start, stop, hltList: {}, {}, {:?}", sStart, sStop, hltList);

      let mut snippetBuilder = fVal[sStart..=sStop].to_string();
      // println!("0 snippetBuilder: {:?}", snippetBuilder);

      let snippetOffset = sStart;                                                     //how to use this?
      // println!("fVal: {:?}", fVal);

      for (hStart, hStop) in hltList.iter().rev() {
        {
          let loc = hStop - snippetOffset + 1;
          let tag = rightHighlightTag;
          // println!("here 00");
          let left = snippetBuilder[0..loc].to_string();
          // println!("here 01");
          let right = snippetBuilder[loc..].to_string();
          // println!("here 02");
          snippetBuilder = format!("{}{}{}", left, tag, right).to_string();
        }
        {
          let loc = hStart - snippetOffset;
          let tag = leftHighlightTag;
          // println!("here 03");
          let left = snippetBuilder[0..loc].to_string();
          // println!("here 04");
          let right = snippetBuilder[loc..].to_string();
          // println!("here 05");
          snippetBuilder = format!("{}{}{}", left, tag, right).to_string();
        }
      }
      if sStart != 0 {
        snippetBuilder = format!("…{}", snippetBuilder);
      }
      if sStop != fVal.len()-1 {
        snippetBuilder = format!("{}…", snippetBuilder);
      }

      // println!("snippetBuilder: {:?}", snippetBuilder);
      snippets.push(st(snippetBuilder.trim()));      //pushing in reverse order.  so, re-reverse later
    }
    // snippets.reverse();
    // println!("snippets: {:?}", snippets);
  }

  (snippets.to_vec(), highlightsCompleteObjectsArray)
}

fn totalSnippetsLen2(taggedSnippetsData: &Vec<(usize, usize, Vec<(usize, usize)>)>) -> usize {
  let mut sum = 0;
  for (start, stop, _hltList) in taggedSnippetsData {
    sum += stop - start;
  }
  sum
}

pub fn getSpaceBoundary(s: &str, loc: i32, direction: &str) -> usize {
  let mut boundary = loc;// + offset;

  if boundary >= s.len() as i32 {
    boundary = (s.len() - 1) as i32;
  }
  if boundary < 0 {
    boundary = 0;
  }

  // println!("0. i, s.len:  {}, {}", boundary, s.len());
  let mut boundary = find_end(s, boundary as usize);
  // println!("1. i, s.len:  {}, {}", boundary, s.len());

  if direction == "right" {
    //walk right until space
    boundary = find_word_break(s, boundary as usize, "right");
  } else {
    //walk left until space
    boundary = find_word_break(s, boundary as usize, "left");
  }

  assert!(boundary < s.len() );
  boundary as usize
}


pub fn buildQueriesMap(qp: &WorpdriveQueryParent, schema: &WorpdriveSchema) -> HashMap<String, Query> {
  let mut m_field_query: HashMap<String, Query> = HashMap::new();
  let searchableFields: Vec<String> = getAllSearchableFields_(schema);

  if qp.queries.len() == 1 && qp.queries[0].fields.len() == 1 && qp.queries[0].fields[0] == "*" {
    let qStr = qp.queries[0].query.to_string();
    for f in searchableFields {
      m_field_query.insert(f.to_string(), Query { query: qStr.to_string(), field: f.to_string(), doPrefixLast: qp.queries[0].doPrefixLast, collection: qp.queries[0].collection.to_string()});
    }
  }
  else {
    for q in &qp.queries {
      let qStr = &q.query;
      for f in &q.fields {
        if f == "*" {
          panic!("wildcard field only allowed for single query");
        }
        if m_field_query.contains_key(f) {
          panic!("multiple queries for same field: {}", f);
        }
        m_field_query.insert(f.to_string(), Query { query: qStr.to_string(), field: f.to_string(), doPrefixLast: q.doPrefixLast, collection: q.collection.to_string()});
      }
    }
  }
  m_field_query
}

pub fn queryClxLocal(queryJson: &str, isQueryMode: bool) {

  unsafe{QUERY_MODE = isQueryMode;}

  let clxnsRoot = SQUIRREL_COLLECTIONS_ROOT;
  resetStaticMaps_clxnsRoot(clxnsRoot, isQueryMode);

  let query_userInput: WorpdriveQueryParent_userInput = serde_json::from_str(&queryJson).unwrap();

  let collectionName = &query_userInput.collection;
  let pid = &query_userInput.pid;

  let schema = readSchema(clxnsRoot, pid, collectionName, DEFAULT_VINTAGE, isQueryMode);

  let queryParent = sanitizeQueryUserInput(query_userInput, &schema);
  let (consoleResponse, jsonResponse) = queryClxLocal_clxnsRoot(&queryParent, &schema, clxnsRoot, DEFAULT_VINTAGE).unwrap();
  println!("\n\n\n{}\n\n{}", jsonResponse, consoleResponse);
}

pub fn queryClxLocal_parallel(queryJson: &str, isQueryMode: bool) {

  unsafe{QUERY_MODE = true;}

  let clxnsRoot = SQUIRREL_COLLECTIONS_ROOT;
  resetStaticMaps_clxnsRoot(clxnsRoot, isQueryMode);

  let start = std::time::Instant::now();

  let query_userInput: WorpdriveQueryParent_userInput = serde_json::from_str(&queryJson).unwrap();

  let collectionName = &query_userInput.collection;
  let pid = &query_userInput.pid;

  let schema = readSchema(clxnsRoot, pid, collectionName, DEFAULT_VINTAGE, isQueryMode);

  // let collectionName = query_userInput.collection.to_string();
  // let schema = readSchema(clxnsRoot, &collectionName, DEFAULT_VINTAGE);

  let queryParent = sanitizeQueryUserInput(query_userInput, &schema);


  (0..10000).into_par_iter().for_each(|_i| {
    queryClxLocal_clxnsRoot(&queryParent, &schema, clxnsRoot, DEFAULT_VINTAGE);
  });

  println!("duration: {:?}", start.elapsed());
}

fn validateInput(qo: &WorpdriveQueryParent) -> String {

  if qo.max_total_snippets_length < qo.min_highlight_context {
    return format!("max_total_snippets_length ({}) is less than min_highlight_context ({})", qo.max_total_snippets_length, qo.min_highlight_context);
  }
  format!("")
}

pub fn expandQuery_fromStr(queryJson: &str, clxnsRoot: &str, vintage: &str, isQueryMode: bool) -> Result<String, Box<dyn Error>> {
  let query_userInput: WorpdriveQueryParent_userInput = serde_json::from_str(queryJson).unwrap();
  expandQuery(query_userInput, clxnsRoot, vintage, isQueryMode)
}

pub fn expandQuery(query_userInput: WorpdriveQueryParent_userInput, clxnsRoot: &str, vintage: &str, isQueryMode: bool) -> Result<String, Box<dyn Error>> {
  // let query_userInput: WorpdriveQueryParent_userInput = serde_json::from_str(queryJson).unwrap();
  // let collectionName = &query_userInput.collection;

  // let schema = readSchema(clxnsRoot, collectionName, vintage);


  let collectionName = &query_userInput.collection;
  let pid = &query_userInput.pid;

  let schema = readSchema(clxnsRoot, pid, collectionName, vintage, isQueryMode);

  let queryParent: WorpdriveQueryParent = sanitizeQueryUserInput(query_userInput, &schema);           //TODO maek endpoint that returns this https://trello.com/c/IHZrlEfB/98-touch-up-endpoints-for-ross-send-binaries-or-ec2

  let myjsonValue = serde_json::to_value(queryParent).unwrap();

  let jsonResponseStr = serde_json::to_string_pretty(&myjsonValue).unwrap();

  Ok(jsonResponseStr.to_string())
}

fn getShards(clxnsRoot: &str, pid: &str, collectionName: &str, vintage: &str) -> Vec<String> {
  let mut shardNames: Vec<String> = Vec::new();

  let shardsRoot = &getDir_shards(&collectionName, clxnsRoot, pid, vintage);
  for shardName in getDirsLast(shardsRoot){
    shardNames.push(shardName);
  }
  shardNames
}

pub fn queryClxLocal_clxnsRoot(queryParent: &WorpdriveQueryParent, schema: &WorpdriveSchema, clxnsRoot: &str, vintage: &str) -> Result<(String, String), Box<dyn Error>> {

  let startTotal = std::time::Instant::now();

  let collectionName = &queryParent.collection;
  let pid = &queryParent.pid;

  if getShards(clxnsRoot, pid, collectionName, vintage).len() > 1 {
    panic!("multiple shards not actually fully supported right now.  need docId offsets for that to work. https://trello.com/c/ijgxkDlr/145-use-docid-offsets-everywhere");
  }

  // let schema = readSchema(clxnsRoot, pid, collectionName, DEFAULT_VINTAGE);

  let potentialError = validateInput(&queryParent);
  if potentialError.len() > 0 {
    return Err(werr(&potentialError));
  }
  let qpStr = serde_json::to_string_pretty(&queryParent).unwrap();
  println!("queryParent: \n{}\n", qpStr);
  let pageNumber = queryParent.page_number;
  let numResultsPerPage = queryParent.num_results_per_page;
  // println!("pageNumber: {}, numResultsPerPage: {}", pageNumber, numResultsPerPage);
  let m_field_query: HashMap<String, Query> = buildQueriesMap(&queryParent, &schema);     // NEED_RDB uses metadb - reads schema
  let sortField = &queryParent.sort_by[0].name;
  let useSorter = sortField != "_score";
  let doSortDescending = queryParent.sort_by[0].is_descending;                                                                       //ensure that dbs are in the map
  let mut m__docId_sortValue: HashMap<u32, f32> = HashMap::new();
  let mut m__docId_relevances: HashMap<u32, Vec<u16>> =  HashMap::new();
  let mut m__docId_hitFields: HashMap<u32, Vec<String>> = HashMap::new();
  let mut m__timeBin_count: HashMap<u32, u32> = HashMap::new();
  let startIterateQFields = std::time::Instant::now();
  let mut numReturnedResults = 0;              // including dups
  let mut numCumReportedResults = 0;             // the total cumulative amount of docIds that all the shards said they found
  let mut durationTokenize =          startIterateQFields.elapsed();
  let mut durationShardIntersection = startIterateQFields.elapsed();
  let mut durationOrganize =          startIterateQFields.elapsed();

  let mut m_field_queryToks:        HashMap<String, Vec<String>> = HashMap::new();
  let mut m_field_queryToksOrig:    HashMap<String, Vec<String>> = HashMap::new();
  let mut m_field_doPrefixLastTok:  HashMap<String, bool> = HashMap::new();

  let isWorptail = schema.is_worptail;
  let mut timestampField: String = st("initialized value that wont be used");

  if useSorter {
    let fields = &schema.fields;
    let schemaSaysItsASortby = fields.into_iter().find(|f| &f.name == sortField).unwrap().sortThisGuy;  //just gets the first one.  TODO in future, support multiple timestamps. query param
    if !schemaSaysItsASortby {
      return Err(werr(&format!("field {} is not a sortby.  TODO make this message more helpful", sortField)));
    }
   }

  if isWorptail {
    let fields = &schema.fields;
    timestampField = fields.into_iter().find(|f| f.is_graphable_timestamp == true).expect("graphable timestamp not found. no good. worptail must have a tivabydized field").name.to_string();  //just gets the first one.  TODO in future, support multiple timestamps. query param
  }

  //so, this iterates through fields ... but could a query span different fields ?!?!?!?!?!?!?!?!?!!!!

  // outer loop should be shard number, not field i think !?!?!?!?!?!?!
  // because we use the same shard's sovabydid for multiple fields.... any reason not to?  that field has to be outer loop?
  //  // well maybe not - consider empty queries? empty sorted queries?  empty timed queries?

  let mut doRemoveRelevances = false;

  for (fieldName, queryObj) in &m_field_query {
    let fQueryStr = &queryObj.query;

    // println!("0 f, q: {}, {}", fieldName, fQueryStr);
    assert!( collectionExists_clxnsRoot(collectionName, clxnsRoot, pid, vintage) );                                                           // use extra space after assert!() so not confused by !
    let mut indexShardsResults: Vec<(String, (usize, Vec<(u32, u16, f32)>, HashMap<u32, u32>, std::time::Duration))> = Vec::new();
    // let indexName = get_indexName_from_collectionName_and_fieldName(collectionName, &fieldName);
    let startTokenize = std::time::Instant::now();
    let (queryCookedToks, excludeTokens, queryOrigCookedToks) = tokenizeQuery(&fQueryStr);                        // TOO SLOW
    let lastQueryCharIsSpace = fQueryStr.len() > 0 && fQueryStr.chars().last().unwrap().to_string() == " ";
    println!("hi queryCookedToks: {:?}", queryCookedToks);
    // assert!(queryCookedToks.len() > 0);
    // assert!(queryCookedToks[0].len() > 0);
    m_field_queryToks.insert(st(fieldName), queryCookedToks.to_vec());
    m_field_queryToksOrig.insert(st(fieldName), queryOrigCookedToks.to_vec());    //why?
    m_field_doPrefixLastTok.insert(st(fieldName), !lastQueryCharIsSpace);
    durationTokenize = startTokenize.elapsed();
    // println!("cookedToks: {:?}", queryCookedToks);


    /*###################################*/
    /*####### Shard Intersections #######*/       //very fast
    /*###################################*/
    let startShardIntersection = std::time::Instant::now();
    //NOW DEAL WITH ... WHAT IF EMPTY QUERY?
    //TODO start here! test these! emptyDefaultQuery and emptySortedQuery
    if queryCookedToks.len() == 0 && !useSorter {
      let shardResults = (st("not necessarily a single shard; doesnt matter"), emptyDefaultQuery(clxnsRoot, pid, collectionName, vintage, numResultsPerPage));
      doRemoveRelevances = true;
      indexShardsResults.push(shardResults);
    }
    else if queryCookedToks.len() == 0 && useSorter {
      for shardName in &getShards(clxnsRoot, pid, collectionName, vintage) {
        println!("shardName LOOP for emptySortedQuery! fieldName: {}, fquery: {}, shardName: {},", fieldName, fQueryStr, shardName);
        let shardResults = (shardName.clone(), emptySortedQuery(clxnsRoot, pid, collectionName, vintage, shardName, numResultsPerPage, sortField, doSortDescending));
        doRemoveRelevances = true;
        indexShardsResults.push(shardResults);
      }
    }
    else {
      for shardName in getShards(clxnsRoot, pid, collectionName, vintage) {
        println!("for shardName LOOP! fieldName: {}, fquery: {}, shardName: {},", fieldName, fQueryStr, shardName);

        let shardResults = (shardName.clone(), getShardMultiIntersection_page1Sorted(clxnsRoot, pid, collectionName, &queryCookedToks, &excludeTokens, queryObj.doPrefixLast && !lastQueryCharIsSpace, &fieldName,
                                                                                    &shardName, pageNumber * numResultsPerPage * 2, useSorter, sortField, isWorptail, &timestampField, doSortDescending, vintage));

        // println!("fieldName: {}, fquery: {}, shardName: {}, shardResultsLen: {},  shardNumResults: {}, shardResults {:?}", fieldName, fQueryStr, shardName, shardResults.1.1.len(),  shardResults.1.0, shardResults);
        indexShardsResults.push(shardResults);
      }
    }
    durationShardIntersection = startShardIntersection.elapsed();
    // println!("1 f, q: {}, {}, durShardIntersection:   {:?} ms", fieldName, fQueryStr, durationShardIntersection.as_millis() );



    /*###########################################*/         // 1. get combined relevance for docIds hit by multiple fields -- loop over ALL results if using custom sorter.  this seems unavoidable.
    /*####### WHAT IS HAPPENING HERE ???? #######*/         // 2. keep track of which fields were responsible for a doc hit - loop over ALL results if using custom sorter
    /*###########################################*/   // very slow
    let startOrganize = std::time::Instant::now();
    for (_shardName, (numShardResults, intersectionTuples, m__timeBin_fieldCount, _)) in indexShardsResults {                // this could be mroe effiricnet cos we're loopingthrough results again later to get intersecitons again.  prob should just put all the indexResults in a map and deal w/ later all at once.  but these intersections are small so its trivial for perf i think
      numCumReportedResults += numShardResults;

      for (fTimeBin, fCount) in m__timeBin_fieldCount {
        *m__timeBin_count.entry(fTimeBin).or_insert(0) += fCount as u32;                                   //https://users.rust-lang.org/t/efficient-string-hashmaps-for-a-frequency-count/7752/2
      }
      println!("gregfsdf intersectionTuples: {:?}", intersectionTuples);

      for (docId, rel, sortValue) in intersectionTuples {
        numReturnedResults += 1;
        m__docId_sortValue.insert(docId, sortValue);                                         //might see multiple same docIds, but they'll all have the same sortValue.  cos sortValue is per doc
        m__docId_relevances.entry(docId).or_insert_with(Vec::new).push(rel);
        m__docId_hitFields.entry(docId).or_insert_with(Vec::new).push(fieldName.to_string());
      }
    } durationOrganize = startOrganize.elapsed();
    // println!("2 f, q: {}, {},     durationOrganize:   {:?} ms", fieldName, fQueryStr, durationOrganize.as_millis() );
  }
  let durIterateQFields =  startIterateQFields.elapsed();
  let startAggRels = std::time::Instant::now();
  let     v__docId_combinedRelevance = aggregateRelevances(m__docId_relevances); //insert sortValues here?
  let mut v__docId_combinedRelevance_sortValue = insertSortValues(v__docId_combinedRelevance, m__docId_sortValue);
  let numUniqueReturnedResults = v__docId_combinedRelevance_sortValue.len();
  let numTotalResults = estimateTotalNumResults(numReturnedResults, numUniqueReturnedResults, numCumReportedResults);
  let durAggRels =  startAggRels.elapsed();

  let startCountDocs = std::time::Instant::now();

  let numDocs = readNumDocs(collectionName, clxnsRoot, pid, vintage);
  let durCountDocs =  startCountDocs.elapsed();
  let mut response = st("");
  response = format!("{}\n{}", response, format!("\nReturned {} results with approx {} unique docIds out of {} documents in {} indexes split in {} shards", numReturnedResults, numTotalResults, numDocs, m_field_query.len(), getShards(clxnsRoot, pid, collectionName, vintage).len()));

  /*################################*/
  /*####### SORT THE RESULTS #######*/
  /*################################*/            //very slow
  let startSort = std::time::Instant::now();
  if useSorter {
    v__docId_combinedRelevance_sortValue = sortResultsExact(v__docId_combinedRelevance_sortValue, doSortDescending);
  }
  if doRemoveRelevances {
    for i in 0..v__docId_combinedRelevance_sortValue.len() {
      v__docId_combinedRelevance_sortValue[i] = (v__docId_combinedRelevance_sortValue[i].0, 0, v__docId_combinedRelevance_sortValue[i].2);
    }

  }

  //sort the worptail timeBin count aggregates by increasing timeBin
  // ok we have analytics data here, 10minuteBins: m__timeBin_count -- how to return?  we should sort first
  let mut vt__timeBin_count: Vec<_> = m__timeBin_count.into_iter().collect();                                                            //https://users.rust-lang.org/t/sort-hashmap-data-by-keys/37095
  vt__timeBin_count.sort_by_key(|k| k.0);
  // vt__timeBin_count.reverse();                                                      // TODO if we return object as streaming data, pass more recent data first

  let durationSort = startSort.elapsed();
  let durationTotal =  startTotal.elapsed();

  response = format!("{}\n{}", response, format!("durTokenize:            {:?} ms", durationTokenize.as_millis() ));
  response = format!("{}\n{}", response, format!("durShardIntersection:   {:?} ms", durationShardIntersection.as_millis()    ));
  response = format!("{}\n{}", response, format!("durOrganize:            {:?} ms", durationOrganize.as_millis()    ));
  response = format!("{}\n{}", response, format!("---------------------------------------"));
  response = format!("{}\n{}", response, format!("iterateQFields:         {:?} ms", durIterateQFields.as_millis()       ));
  response = format!("{}\n{}", response, format!("aggRelevances:          {:?} ms", durAggRels.as_millis()        ));
  response = format!("{}\n{}", response, format!("durCountDocs:           {:?} ms", durCountDocs.as_millis()     ));
  response = format!("{}\n{}", response, format!("durSort:                {:?} ms", durationSort.as_millis()     ));
  response = format!("{}\n{}", response, format!("---------------------------------------"));
  response = format!("{}\n{}", response, format!("durTotal:               {:?}", durationTotal    ));
  response = format!("{}\n{}", response, format!(""));

  let fieldsToReturn = queryParent.fields_to_return.to_vec();
  let doHighlight_tagged = queryParent.do_highlights_tagged;
  let doHighlight_objects = queryParent.do_highlights_objects;
  let doHighlight_map = queryParent.do_highlights_map;
  let leftHighlightTag = &queryParent.highlight_pre_tag;
  let rightHighlightTag = &queryParent.highlight_post_tag;
  let minHighlightContext = queryParent.min_highlight_context;
  let maxSnippetsLen = queryParent.max_total_snippets_length;
  let (displayResponse, hitsVec) = displayPageNResults(clxnsRoot, pid, collectionName, numResultsPerPage, pageNumber, fieldsToReturn,
                                            doHighlight_tagged, doHighlight_objects, doHighlight_map, doRemoveRelevances,
                                            leftHighlightTag, rightHighlightTag, minHighlightContext, maxSnippetsLen,
                                            m_field_queryToks, m_field_queryToksOrig, m_field_doPrefixLastTok, m__docId_hitFields, v__docId_combinedRelevance_sortValue, vintage);
  response = format!("{}\n{}", response, displayResponse);

  let responseObj: WorpQueryResponse = WorpQueryResponse { total: numTotalResults, hits: hitsVec, timestamp_10min_aggregates: if isWorptail { Some(vt__timeBin_count) } else { None } };

  let myjsonValue = serde_json::to_value(responseObj).unwrap();

  let jsonResponseStr = serde_json::to_string_pretty(&myjsonValue).unwrap();

  unsafe { QUERY_MODE = true; }
  Ok((response, jsonResponseStr.to_string()))
}

pub fn estimateTotalNumResults(numReturnedResults: usize, numUniqueReturnedResults: usize, numCumReportedResults: usize) -> usize {
  let numDups = numReturnedResults - numUniqueReturnedResults;
  let ratioDups = (numDups as f32) / (numReturnedResults as f32);
  let estimatedTotalDups = ratioDups * (numCumReportedResults as f32);
  let estimatedTotalUniqueResults = numCumReportedResults as f32 - estimatedTotalDups;
  estimatedTotalUniqueResults.floor() as usize
}


/// https://stackoverflow.com/a/43279057/8870055
fn find_end(s: &str, i: usize) -> usize {
    assert!(i <= s.len());                        // why?
    let mut end = i;
    while !s.is_char_boundary(end) {
        end += 1;
    }
    end
}

/// direction = "right" or "left"
fn find_word_break(s: &str, i: usize, direction: &str) -> usize {
  // println!("find_word_break: s {}, i {}, s.len: {}, direction: {}", s, i, s.len(), direction);
  assert!(i <= s.len());
  if i == 0 || i == s.len() {
    return i;
  }
  let mut k = i;
  if direction == "right" {
    while k < s.len(){
      if s.is_char_boundary(k) && !s[k..k+1].chars().nth(0).unwrap().is_alphanumeric() {
        return k;
      }
      k += 1;
    }
  } else if direction == "left" {
    while k > 0 {
      if s.is_char_boundary(k) && !s[k..k+1].chars().nth(0).unwrap().is_alphanumeric() {
        return k;
      }
      k -= 1;
    }
  }
  if k >= s.len() {
    return s.len() - 1;
  }
  k
}


fn findMissingOrNextNumber(nums: &Vec<usize>) -> usize {

  // println!("existing docIds: {:?}", nums);
  if nums.len() == 0 {
    return 1;
  }

  let mut i = 1;

  while i < nums.len(){
    if nums[i] > nums[i-1] + 1 {
      return nums[i-1] + 1;
    }
    i += 1;
  }
  let newDocId = nums[i-1] + 1;
  // println!("new docId: {:?}", newDocId);
  newDocId
}


pub fn getPrefixesForToken(token: &str) -> Vec<String> {
  let mut prefixes: Vec<String> = Vec::new();

  let theChars: Vec<char> = token.chars().collect();

  prefixes.push(theChars[0].to_string());

  // for i in 1..theChars.len()-1 {
  for i in 1..theChars.len() {
    prefixes.push(prefixes[i-1].clone() + &theChars[i].to_string());
  }

  prefixes
}

pub fn emptySortedQuery(clxnsRoot: &str, pid: &str, collection: &str, vintage: &str, shard: &str, numResultsToReturn: usize, sortby: &str, doSortDescending: bool) -> (usize,  Vec<(u32, u16, f32)>,  HashMap<u32, u32>, std::time::Duration) {

  let mut intersectionTuples:  Vec<(u32, u16, f32)>  = Vec::new();
  let mut m_timeBin_count: HashMap<u32, u32> = HashMap::new();

  let readtimeStart = std::time::Instant::now();

  // ok but how to do this one ......
  // ignore docIds .... get the sovabydid

  // println!("sortby:: {}", sortby);

  let sovabydid = readSovabydid(collection, clxnsRoot, pid, vintage, shard, sortby);

  //ok this is a ... "sorter values by docId" array ....  (for a single shard. fine)
  //now do a "partial"? sort?  just get the top n values
  // "lazy sort"
  // is there a rust api for this?  or do manually...
  // unsafe { v.sort_lazy_fast(i32::cmp, N) };
  // eh just do manually like in intersection

  let myclosureAscending = |a: &(u32, u16, f32), b: &(u32, u16, f32)| ascendingComparator_3ple(a, b);
  let myclosureDescending = |a: &(u32, u16, f32), b: &(u32, u16, f32)| descendingComparator_3ple(a, b);
  let myclosure =  if doSortDescending { myclosureDescending } else { myclosureAscending };

  let mut binHeap__docId_sortValue = BinaryHeap::from_vec_cmp(Vec::new(), myclosure);
  let mut binHeap_size = 0;
  let mut binHeap_lastValue = if doSortDescending { std::f32::MIN } else { std::f32::MAX };
  let firstIsBetter_ascending = |a: f32, b: f32| a < b;
  let firstIsBetter_descending = |a: f32, b: f32| a > b;
  let firstIsBetter = if doSortDescending { firstIsBetter_descending } else { firstIsBetter_ascending };

  //ok now what ... we need to iterate thorugh the sov, and for each element, make a tuple.  use into_iter ?
  // take that tuple, and put it in the binary heap if the sortervalue is good enough
  // and then rmeove an element.  just liek in the interseciton fx

  // println!("sovabydid len:: {:?}", sovabydid.len());    //TODO why is this so big???? dlfkjsiesoijfldk
  // println!("sovabydid[0..100].to_vec():: {:?}", sovabydid[0..100].to_vec());    //TODO why is this so big???? dlfkjsiesoijfldk
  // std::process::exit(1);

  // for (docId, sortValue) in sovabydid.iter().enumerate() {
  // let mut debuggingCount = 0;
  for i in 1..sovabydid.len() {
    // debuggingCount += 1;
    // if debuggingCount > 80 {
    //   break;
    // }
    let docId = i;
    let sortValue = sovabydid[i];
    if binHeap_size < numResultsToReturn {
      binHeap__docId_sortValue.push((docId as u32, max(0, min(numResultsToReturn  as i64 - i as i64, 65000)) as u16, sortValue)); // this max(0... stuff is just so the non-sorted docs are displayed in docId-increasing order
      binHeap_size = binHeap__docId_sortValue.len();
    }
    else if firstIsBetter(sortValue, binHeap_lastValue) {
      binHeap__docId_sortValue.push((docId as u32, max(0, min(numResultsToReturn  as i64 - i as i64, 65000))as u16, sortValue));
      binHeap__docId_sortValue.pop();
      binHeap_lastValue = binHeap__docId_sortValue.peek().unwrap().2;
    }

    // println!("docId, sortValue, binHeap__docId_sortValue:: {}, {}, {:?}", docId, sortValue, binHeap__docId_sortValue);
  }
  for (docId, rel, sortValue) in binHeap__docId_sortValue.into_iter() {
    intersectionTuples.push((docId, rel, sortValue));
  }

  (0,  intersectionTuples,  m_timeBin_count , readtimeStart.elapsed())
}

pub fn emptyDefaultQuery(clxnsRoot: &str, pid: &str, collection: &str, vintage: &str, numResultsToReturn: usize) -> (usize,  Vec<(u32, u16, f32)>,  HashMap<u32, u32>, std::time::Duration) {

  //NO.  the real way to do this is to get the list of all the docIds, then return the first n, and then ...
  // what to put in intersectionTuples ???       for (docId, rel, sortValue) in intersectionTuples {

    // just docId... leave other items whatever
  let readtimeStart = std::time::Instant::now();

  // println!("emptyDefaultQuery: numResultsToReturn: {:?}", numResultsToReturn);
  let docIds_all = getDocIds(clxnsRoot, pid, collection, vintage, true);
  // println!("emptyDefaultQuery: docIds_all: {:?}", docIds_all);
  let docIds = docIds_all[..min(docIds_all.len(), numResultsToReturn)].to_vec();
  // println!("emptyDefaultQuery: docIds: {:?}", docIds);

  let mut intersectionTuples:  Vec<(u32, u16, f32)>  = Vec::new();

  let mut count: u16 = numResultsToReturn as u16;
  for docId in &docIds {
    intersectionTuples.push((*docId as u32, count, f32::NAN));        //count inserted so final sort order matches docId order.  have to remember to remove these before returning!
    count -= 1;
  }

  let mut m_timeBin_count: HashMap<u32, u32> = HashMap::new();

  (docIds.len(),  intersectionTuples,  m_timeBin_count , readtimeStart.elapsed())
}

/// returns tuple (numTotalResults, page1_intersection, duration)
pub fn getShardMultiIntersection_page1Sorted(clxnsRoot: &str, pid: &str, collection: &str, queryToks: &Vec<String>, excludeToks: &Vec<String>,  doPrefixForLastTok: bool,
                                              fieldName: &str, shardName: &str,
                                             numResultsToReturn: usize, useAltSort: bool, sortField: &str,  isWorptail:bool, timestampField:&str, doSortDescending: bool,
                                             vintage: &str) -> (usize, Vec<(u32, u16, f32)>, HashMap<u32, u32>, std::time::Duration) {
  // println!("in getShardMultiIntersection_page1Sorted shardName, fieldName: {}, {}", shardName, fieldName);
  // println!("in getShardMultiIntersection_page1Sorted doPrefixForLastTok: {:?}", doPrefixForLastTok);
  // println!("in getShardMultiIntersection_page1Sorted queryTokens: {:?}", queryToks);
  let shart = std::time::Instant::now();
  // let readtimeStart = std::time::Instant::now();

  let sovabydid:  Vec<f32> =     if !useAltSort {            Vec::new() } else { readSovabydid(collection, clxnsRoot, pid, vintage, shardName, sortField) };
  let tivabydid:  Vec<u64> =     if !isWorptail {            Vec::new() } else { readTivabydid(collection, clxnsRoot, pid, vintage, shardName, timestampField) };
  let invinds:    Vec<Vec<u8>> = if queryToks.len() == 0 {   Vec::new() } else { readInvindBas(&queryToks, fieldName, shardName, doPrefixForLastTok, clxnsRoot, pid, collection, vintage) };
  let excludeIis: Vec<Vec<u8>> = if excludeToks.len() == 0 { Vec::new() } else { readInvindBas(&excludeToks, fieldName, shardName, false, clxnsRoot, pid, collection, vintage) };

  // println!("sovabydid: {:?}", sovabydid);
  // println!("tivabydid: {:?}", tivabydid);
  // println!("invinds: {:?}", invinds);
  // println!("excludeInvinds: {:?}", excludeIis);

  // let readtimeDuration = readtimeStart.elapsed();
  let mut iiRefs: Vec<&Vec<u8>> = Vec::new();                 //why?
  for ii in &invinds {
    iiRefs.push(ii);
  }
  let mut excldudeIiRefs: Vec<&Vec<u8>> = Vec::new();
  for ii in &excludeIis {
    excldudeIiRefs.push(ii);
  }
  // let intersectiontimeStart = std::time::Instant::now();

  let (numTotalResults, intersectionTuples, m_timeBin_count) =
      multiIiIntersection(&mut iiRefs, &mut excldudeIiRefs, useAltSort, &sovabydid, isWorptail, &tivabydid, doSortDescending, numResultsToReturn);

  // let (numTotalResults, intersectionTuples, m_timeBin_count) = multiIiIntersection(&mut iiRefs, &mut excldudeIiRefs, useAltSort, &sovabydid, isWorptail, &tivabydid, doSortDescending, numResultsToReturn);
  // let intersectiontimeDuration = intersectiontimeStart.elapsed();

  // println!("numTotalResults: {},  intersectionTuples: {:?}", numTotalResults,  intersectionTuples);

  let duration = shart.elapsed();

  // println!("in shard: queryToks: {:?}, excludeToks: {:?}, numTotalResults: {}, readTime: {:?}, intersectTime: {:?}, totalTime: {:?}",
  //     queryTokens, excludeTokens, numTotalResults, readtimeDuration, intersectiontimeDuration, duration);

  (numTotalResults, intersectionTuples, m_timeBin_count, duration)
}

pub fn displayInvind(pid: &str, collection: &str, fieldName: &str, cookedTok: &str, isPrefix: bool, shardName: &str){
    let key = format!("{}", &cookedTok);
    let rocksRoot = &getDir_rocksRoot_invind(collection, SQUIRREL_COLLECTIONS_ROOT, pid, DEFAULT_VINTAGE, shardName, fieldName);
    println!("displayInvind rr: {}", rocksRoot);
    println!("displayInvind key: {}", key);
    println!("displayInvind read frm fieldName, shard, key: {:<8} {:<2} {:<10}   {:?}", fieldName, shardName, key, readInvindVec(cookedTok, isPrefix, rocksRoot));
}

pub fn getCollectionDataSize(clxnsRoot: &str, pid: &str, collection: &str, vintage: &str) -> u64 {
  let vintageDir = &getDir_collectionVintage(collection, clxnsRoot, pid, vintage);
  let folder_size = get_size(vintageDir).unwrap();
  folder_size
}

pub fn getSortBys(clxnsRoot: &str, pid: &str, collection: &str, vintage: &str, isQueryMode: bool) -> Vec<String> {
  let schema = readSchema(clxnsRoot, pid, collection, vintage, isQueryMode);
  let mut sortbys: Vec<String> = Vec::new();
  for f in schema.fields {
    if f.sortThisGuy {
      sortbys.push(f.name.to_string());
    }
  }
  sortbys
}

pub fn modifyFieldAttribute(pid: &str, collection: &str, field: &str, attribute: &str, value: bool) -> Result<String, Box<dyn Error>> {

  println!("modifyFieldAttribute: {} {} {} {} {}", pid, collection, field, attribute, value);

  if false {}
  else if attribute == "sortThisGuy" && value == true {             return           setFieldAsSorter(pid, collection, field);   }
  else if attribute == "sortThisGuy" && value == false {            return         unsetFieldAsSorter(pid, collection, field);   }
  else if attribute == "is_graphable_timestamp" && value == true {  return      setFieldAsTimegrapher(pid, collection, field);   }
  else if attribute == "is_graphable_timestamp" && value == false { return    unsetFieldAsTimegrapher(pid, collection, field);   }
  else if attribute == "doIndexPrefixes" && value == true {         return          indexWithPrefixes(pid, collection, field);   }
  else if attribute == "doIndexPrefixes" && value == false {        return        unindexPrefixesOnly(pid, collection, field);   }
  else if attribute == "searchMe" && value == true {                return indexTokensWithoutPrefixes(pid, collection, field);   }
  else if attribute == "searchMe" && value == false {               return        unindexFieldInvinds(pid, collection, field);   }

  Err(werr(&format!("invalid params: pid, collection, field, attribute, value: {} {} {} {} {}", pid, collection, field, attribute, value)))
}


pub fn ensureCollectionExists(pid: &str, collection: &str) -> Result<(), Box<dyn Error>>{

  let collectionSquirrelDir = &getDir_collectionParent(collection, SQUIRREL_COLLECTIONS_ROOT, pid);

  if exists(collectionSquirrelDir) {
    Ok(())
  }
  else {
    Err(werr(&format!("project/collection {}/{} not found, cos dir not found: {}", pid, collection, collectionSquirrelDir)))
  }
}

pub fn ensureProjectExists(pid: &str) -> Result<(), Box<dyn Error>>{

  let projectDir = &getDir_project(SQUIRREL_COLLECTIONS_ROOT, pid);

  if exists(projectDir) {
    Ok(())
  }
  else {
    Err(werr(&format!("project {} not found, cos dir not found: {}", pid, projectDir)))
  }
}