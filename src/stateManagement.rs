use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::RwLock;

// use std::error::Error;


use serde_json::Value;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
// extern crate queues;
use queues::Queue;

use crate::h::st;

#[derive(Debug)]
pub struct DbStuff {
  pub db: std::option::Option<rocksdb::DB>,
}

lazy_static! {  pub static ref RootSecretHash: RwLock<Option<String>> = RwLock::new(Some(st(ROOT_SECRET_HASH_HARDCODED)));}

lazy_static! {  pub static ref ProjectSecretHash: RwLock<Option<String>> = RwLock::new(None);}    // TODO set on startup from GLOBAL_SECRET_HASH_HARDCODED_TEMPORARY

lazy_static! {  pub static ref GlobalSecretHash: RwLock<Option<String>> = RwLock::new(None);}    // TODO set on startup

lazy_static! {  pub static ref M___PID_COLLECTION___STATUS: RwLock<HashMap<(String, String), String>> = RwLock::new(HashMap::new());}

lazy_static! {  pub static ref M_ROCKSROOT_DB: RwLock<HashMap<String, DbStuff>> = RwLock::new(HashMap::new()); }      //it's much faster now using rwlock isntead of mutex!  i guess mutexes were waiting on each other for reads before

lazy_static! {
  // written from ferret query, read by scribe
  /// aka "scribe's queue" \
  /// holds tuple: (requestId, task, pid, collection, payload)
  pub static ref M___PID_COLLECTION___Q__SCRIBE_TASKS: Mutex<HashMap<(String, String), Queue<(String, String, String, String, String)>>> = Mutex::new(HashMap::new());             //mutex instead of rwlock cos we'll never need parallel reads
  //scribe is a single ferret process that is always running.  constanly polling this queue for new queries to idnex into the HQC
}
lazy_static! {
  // written from ferret query, read by vintner
  pub static ref M__QID__INUSE_ClxnVintage: Mutex<HashMap<String, (String,String)>> = Mutex::new(HashMap::new());
  //vintner is a single ferret process that is always running.  constanly polling the vintagesDir for new vintages. if found, delete old ones - read this map to ensure not being used first
}
lazy_static! {
  /// payload should prob be an http response object or something in case it contains an error?
  // pub static ref MScribeResponses__REQUESTID_PAYLOAD: Mutex<HashMap<String, Result<String, Box<dyn Error>>>> = Mutex::new(HashMap::new());  //   | |_^ `(dyn std::error::Error + 'static)` cannot be sent between threads safely   :(

  /// this payload is (isSuccess, message, topic)
    pub static ref MScribeResponses__REQUESTID_PAYLOAD: Mutex<HashMap<String, (bool, String, String)   >> = Mutex::new(HashMap::new());
}
// lazy_static! {
//   // written from ferret query, read by bookkeeper
//   pub static ref Q_USER_USAGE_METRICS_TO_INCREMENT: Mutex<Queue<(String, String)>> = Mutex::new(Queue::new());
//   //bookkeeper is single ferret process that records user metrics.  constantly polling this queue and writing results to its own "user metrics" rocksdb - todo! how/when/where create this new db!
//   // no! bookkeeping is a "write" job, so it gets sent to M___PID_COLLECTION___Q__SCRIBE_TASKS ! we should delete all bookkeeper stuff now i think???
// }

pub static SHARD_META: &str = "meta";
pub static SHARD_DOCUMENTS: &str = "documents";
pub static SHARD_LOCATIONS: &str = "locations";
pub static SHARD_SOVABYDID: &str = "sovabydids";
pub static SHARD_TIVABYDID: &str = "tivabydids";
pub static INDEXES_ROOT_NAME: &str = "invinds";
pub static VINTAGES_DIR_NAME: &str = "vintages";
pub static PROJECTS_DIR_NAME: &str = "projects";
pub static MAIN_DIR_NAME: &str = "main";
pub static SHARDS_DIR_NAME: &str = "shards";
pub static ROCKSDB_DIR_NAME: &str = "rocksdb";
// pub static ROCKSDB_DIR_NAME: &str = "rocksdb";
pub static DEFAULT_VINTAGE: &str = "default";

pub static SPLIT_KEEP_CHARS: &str = "$#+";


pub static CRITTERS_ROOT: &str = "worp_data";                         // wish other vars here could use this ... ???  maybe SQUIRREL_COLLECTIONS_ROOT should b a fx???
/// squirrels squirrel things away
pub static SQUIRREL_COLLECTIONS_ROOT: &str = "worp_data/squirrel";
/// ferrets ferret things out
pub static FERRET_COLLECTIONS_ROOT: &str = "worp_data/ferret";

pub static mut QUERY_MODE: bool = false;

/* rocksdb stuff */

pub static KEY_SCHEMA: &str = "schema";
pub static KEY_NUM_DOCS: &str = "numDocs";
pub static KEY_MARCO: &str = "marco";
pub static KEY_POLO: &str = "polo";

// pub static WORP_AUTO_FIELD_PREFIX: &str = "𝒲";   // just use underscore prefix for worp-defined fields. reserved in elasticsearch too https://discuss.elastic.co/t/illegal-characters-in-elasticsearch-field-names/17196
pub static WORP_AUTO_FIELD_PREFIX: &str = "_";   // just use underscore prefix for worp-defined fields. reserved in elasticsearch too https://discuss.elastic.co/t/illegal-characters-in-elasticsearch-field-names/17196



// the head scribe, always running.  starts other scribes

pub static SAM_TARLY_DUMMY_PID: &str  = "SAM_TARLY";
pub static SAM_TARLY_DUMMY_COLLECTION: &str  = "SAM_TARLY";


// scribe queue tasks:
// pub static HIRE_NEW_SCRIBE: &str  = "HIRE_NEW_SCRIBE";

pub static INDEX_DOCUMENT: &str  = "INDEX_DOCUMENT";
pub static INCREMENT_BILLING_METRIC_READS : &str = "INCREMENT_BILLING_METRIC_READS";
pub static INCREMENT_BILLING_METRIC_WRITES: &str  = "INCREMENT_BILLING_METRIC_WRITES";
pub static GET_ALL_META_OBJECTS: &str  = "GET_ALL_META_OBJECTS";
pub static MODIFY_FIELD_ATTRIBUTE: &str  = "MODIFY_FIELD_ATTRIBUTE";
pub static CREATE_COLLECTION: &str  = "CREATE_COLLECTION";
pub static DELETE_COLLECTION: &str  = "DELETE_COLLECTION";
pub static INDEX_BULK: &str  = "INDEX_BULK";
pub static DELETE_DOC: &str  = "DELETE_DOC";
pub static RETIRE_SCRIBE: &str  = "RETIRE_SCRIBE";


/// my secret rootSecret
pub static ROOT_SECRET_HASH_HARDCODED: &str  = "$argon2i$v=19$m=8,t=3,p=1$cGlua2hpbWFsYXlhbg$X0EX//XwZB/NB7Xn2+afYBuhSADKO7QoDFbqoJ2scb0";
///my secret globalSecret
pub static GLOBAL_SECRET_HASH_HARDCODED_temporary_shouldComeFromPostgres: &str  = "$argon2i$v=19$m=8,t=3,p=1$cGlua2hpbWFsYXlhbg$y0EWp5BWabUrrQAvZC4WRuqeNEjSM+a5jO/RSId/3lQ";
/// my secret projectSecret
pub static PROJECT_SECRET_HASH_HARDCODED_temporary_shouldComeFromPostgres: &str  = "$argon2i$v=19$m=8,t=3,p=1$cGlua2hpbWFsYXlhbg$fIDyM/GeZo/N0zEhrnCswVzdWAmqsBaSVzR5RscXY3Y";

// /////////////////////////////////////////////// structs & stuff
// /////////////////////////////////////////////// structs & stuff
// /////////////////////////////////////////////// structs & stuff
// /////////////////////////////////////////////// structs & stuff
// /////////////////////////////////////////////// structs & stuff

/* struct helpers? */

pub fn parseSchemaStr(schemaJsonStr: &str) -> WorpdriveSchema {
  // println!(" parseSchemaStr: {}", schemaJsonStr);
  let schemaParsed: WorpdriveSchema = serde_json::from_str(&schemaJsonStr).unwrap();
  schemaParsed
}


pub fn schemaContainsField(schema: &WorpdriveSchema, fieldName: &str) -> bool {
// maybe use thsi instead:
//      if fields.into_iter().filter(|f| f.name == fieldName).count() == 0 {

  for fieldObj in &schema.fields {
    if fieldObj.name == fieldName {
      return true;
    }
  }
  false
}


/// ad hoc creation when new field noticed - assumed to be prefixed, searchable.  not sorted or timestamp.
pub fn updateSchemaWithNewField(mut schema: WorpdriveSchema, fieldName: &str) -> WorpdriveSchema {
  //TODO replace w/ default vars  ?
  let newFieldObj: WorpdriveFieldObject = WorpdriveFieldObject {
    name: fieldName.to_string(),
    searchMe: true,
    sortThisGuy: false,
    doIndexPrefixes: true,
    is_graphable_timestamp: false
  };
  schema.fields.push(newFieldObj);

  schema
}

pub fn getAllSearchableFields_(schema: &WorpdriveSchema) -> Vec<String> {
  let mut searchableFields: Vec<String> = Vec::new();
  for f in &schema.fields {
    if f.searchMe {
      searchableFields.push(f.name.to_string());
    }
  }
  searchableFields
}

/* actual structs */


#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BillingMetricsMonthObj {
  pub reads: usize,
  pub writes: usize
}




#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PidAndCollectionAndDocId {
    pub pid: String,
    pub collection: String,
    pub docId: u32,
    pub secret: String,
    pub is_global_secret: Option<bool>
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PidAndCollection {
    pub pid: String,
    pub collection: String
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RequestId {
    pub request_id: String
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PidAndCollectionAndSecret {
    pub pid: String,
    pub collection: String,
    pub secret: String,
    pub is_global_secret: Option<bool>
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PidAndCollectionAndBulkDocsArray {
    pub pid: String,
    pub collection: String,
    pub documents: Vec<Value>,
    pub secret: String,
    pub is_global_secret: Option<bool>
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PidAndCollectionAndDocument {
    pub pid: String,
    pub collection: String,
    pub document: Value,
    pub secret: String,
    pub is_global_secret: Option<bool>
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PidAndCollectionAndQuery {
    pub pid: String,
    pub collection: String,
    pub query: String
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ModifyFieldAttribute {
    pub pid: String,
    pub collection: String,
    pub field: String,
    pub attribute: String,
    pub value: bool,
    pub secret: String,
    pub is_global_secret: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)] //Debug is for println default
#[serde(deny_unknown_fields)]
pub struct WorpdriveSchema {
  pub pid: String,
  pub collection: String,
  pub do_log_queries: bool,
  pub max_docs_per_shard: usize,
  pub default_num_results_per_page: usize,
  pub is_worptail: bool,
  pub fields: Vec<WorpdriveFieldObject>,
}

// #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WorpdriveFieldObject {
  pub name: String,
  pub searchMe: bool,                             // what does this even mean?  what does it mean exactly to be "isForSearching"
  pub sortThisGuy: bool,
  pub doIndexPrefixes: bool,
  pub is_graphable_timestamp: bool
}

#[derive(Serialize, Deserialize, Debug)] //Debug is for println default
#[serde(deny_unknown_fields)]
pub struct WorpdriveSchema_userInput {
  pub pid: String,
  pub collection: String,
  pub do_log_queries: Option<bool>,                     //default: true
  pub max_docs_per_shard: Option<usize>,                //default: 150000 ? what's max?  max doc id which is u32 max = 4.29 billion?  wait no not yet cos docIds arent offset per shard ...
  pub default_num_results_per_page: Option<usize>,              //default: 10
  pub is_worptail: Option<bool>,
  pub fields: Option<Vec<WorpdriveFieldObject_userInput>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub global_secret: Option<String>                                     //this secret will always be global, cos users wont have access to this endpoint.  u can only make a new collection via worp website. right?
}

// #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WorpdriveFieldObject_userInput {
  pub name: String,
  pub searchMe: Option<bool>,
  pub sortThisGuy: Option<bool>,
  pub doIndexPrefixes: Option<bool>,
  pub is_graphable_timestamp: Option<bool>,
}

/////////////////
/////////////////
/////////////////
/////////////////
/////////////////
/////////////////
/////////////////
/////////////////
/////////////////
/////////////////
/////////////////

#[derive(Serialize, Deserialize, Debug)] //Debug is for println default
#[serde(deny_unknown_fields)]
pub struct WorpdriveQueryParent {
  pub collection: String,
  pub pid: String,
  pub do_log_query_for_analytics: bool,
  pub num_results_per_page: usize,
  pub page_number: usize,
  pub queries: Vec<WorpdriveQueryChild>,
  pub sort_by: Vec<SortByObject>,
  pub fields_to_return: Vec<String>,
  pub do_highlights_tagged: bool,
  pub do_highlights_map: bool,
  pub highlight_pre_tag: String,
  pub highlight_post_tag: String,
  pub min_highlight_context: usize,
  pub max_total_snippets_length: usize,
  pub do_highlights_objects: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct SortByObject {
  //there's no SortByObject_userInput for this.  if User wants to submit a sort object, it must be complete
  pub name: String,
  pub is_descending: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WorpdriveQueryChild {
  pub query: String,
  pub fields: Vec<String>,
  pub doPrefixLast: bool,
  pub collection: String,
}

#[derive(Serialize, Deserialize, Debug)] //Debug is for println default
#[serde(deny_unknown_fields)]
pub struct WorpdriveQueryParent_userInput {
  pub collection: String,
  pub pid: String,
  pub do_log_query_for_analytics: Option<bool>,                           // default true
  pub num_results_per_page: Option<usize>,
  pub page_number: Option<usize>,
  pub queries: Option<Vec<WorpdriveQueryChild_userInput>>,
  pub sort_by: Option<Vec<SortByObject>>,
  pub fields_to_return: Option<Vec<String>>,
  pub do_highlights_tagged: Option<bool>,
  pub do_highlights_map: Option<bool>,
  pub highlight_pre_tag: Option<String>,
  pub highlight_post_tag: Option<String>,
  pub min_highlight_context: Option<usize>,
  // pub // max_highlight_context: Option<usize>,
  pub max_total_snippets_length: Option<usize>,
  pub do_highlights_objects: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct WorpdriveQueryChild_userInput {
  pub query: Option<String>,
  pub fields: Option<Vec<String>>,
  pub doPrefixLast: Option<bool>,
  pub collection: Option<String>,
}

// #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Query {
  pub query: String,
  pub field: String,
  pub doPrefixLast: bool,
  pub collection: String,
}


//////////////////////////////////////////////////
//////////////////////////////////////////////////
//////////////////////////////////////////////////
//////////////////////////////////////////////////
//////////////////////////////////////////////////
//////////////////////////////////////////////////


#[derive(Serialize, Deserialize, Debug)]  //Debug is for println default
#[serde(deny_unknown_fields)]
pub struct WorpQueryResponse {
  // took: u128,                // this is also stupid.  who cares. dont encourage website to distract user w meaninglessness
  // query_id: String,         //this is stupid/pointless.  for follow-up queries like pagination and aggregates, in the backend, just create an ID for this query based on the query.  no need for special other id. ... ok actually we might want this later ... cos maybe u want page n of a specific query that's locked in before changes were made to the index.  but that seems rare.  it seems fine if page content changes during pagination to reflect new data... in the future we just need better pagination capability
  pub total: usize,
  // max_score: usize,
  pub hits: Vec<WorpHitObject>,            // a hit == a matched document

  #[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp_10min_aggregates: Option<Vec<(u32, u32)>>
}

#[derive(Serialize, Deserialize, Debug)]  //Debug is for println default
#[serde(deny_unknown_fields)]
pub struct WorpHitObject {
  pub id: usize,
  pub score: usize,
  pub source: HashMap<String, Value>,                      //https://github.com/serde-rs/json/issues/144#issuecomment-242877324
  // source: Option<HashMap<String, Value>>,                      //https://github.com/serde-rs/json/issues/144#issuecomment-242877324

  #[serde(skip_serializing_if = "Option::is_none")]
  pub highlights_tagged: Option<Vec<WorpHighlightObject>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub highlights_objects: Option<Vec<WorpHighlightObjectsArrayObject>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub highlights_map: Option<BTreeMap<String, Vec<BTreeMap<String, Value>>>>
  // highlights: Option<Vec<WorpHighlightObject>>
}

#[derive(Serialize, Deserialize, Debug)]  //Debug is for println default
#[serde(deny_unknown_fields)]
pub struct WorpHighlightObject {                                                       // ... is this struct actually necessary?
  pub field: String,
  pub snippets: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]  //Debug is for println default
#[serde(deny_unknown_fields)]
pub struct WorpHighlightObjectsArrayObject {                                          // ... is this struct actually necessary?
  pub field: String,
  pub content_objects: Vec<BTreeMap<String, Value>>
}

