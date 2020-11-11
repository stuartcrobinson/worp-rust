use crate::global::{resetStaticMaps_clxnsRoot, queryClxLocal_clxnsRoot};
use crate::h::*;
use crate::global::*;
use crate::dirStuff::*;
use crate::rocksdb_tools::*;
use crate::stateManagement::*;
use std::collections::HashMap;
use crate::queues::IsQueue;
use std::time;
use std::thread::sleep;
use std::collections::HashSet;
use std::collections::BTreeMap;

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use warp::{
  http::{Response, StatusCode},
  Filter,
};
use crate::worpError::WError;
use crate::worpError::werr;
use std::error::Error;
use queues::Queue;
use chrono::{Datelike, Timelike, Utc};
use argon2::{self, Config};

use serde_json::Value;


const millisToWait_active: u64 = 0;
const millisToWait_inactive: u64 = 2000;


fn vintageIsInDbsMap(pid: &str, collectionName: &str, vintage: &str) -> bool {

  // let mut qStr = String::new();
  // {
  //   let qqqq = M___PID_COLLECTION___Q__SCRIBE_TASKS.lock().unwrap().get(&(st(pid), st(collectionName)));
  //   qStr = serde_json::to_string_pretty(&qqqq).unwrap();
  // }



  //check and see if the meta db is in the map for this vintage
  // let shardId = &getShardId_vintage(collectionName, vintage, SHARD_META);
  let rocksRoot = &getDir_rocksRoot_meta(collectionName, FERRET_COLLECTIONS_ROOT, pid, vintage);
  // let rocksId = &getMetaRocksId_v(collectionName, vintage);
  // println!("here1.2 {:?}", &rocksRoot);
  unsafe {
    if QUERY_MODE {                                                               //must be in query mode cos we're querying
        // displayMap();
        let map = M_ROCKSROOT_DB.read().unwrap();
        // println!("M_ROCKSROOT_DB: {:?}",  map);

        return map.contains_key(rocksRoot);
    }
  }
  panic!("wont happen")
}

fn addVintageToDbsMap(collectionName_: &str, clxnsRoot: &'static str, pid_: &str, vintageDir: &str) {//-> JoinHandle<()> {
  let collectionName = collectionName_.to_string();
  let pid = pid_.to_string();
  let v = lastPath(vintageDir);
  let _thread_one = std::thread::spawn(move || {
    addRocksDbsToStaticMap(&collectionName, &clxnsRoot, &pid, &v, true)
  }  );
}

fn getVintageDirs(collectionName: &str, clxnsRoot: &str, pid: &str) -> Result<Vec<String>, Box<dyn Error>> {

  let vintagesRoot = &getDir_collectionVintages(collectionName, clxnsRoot, pid);

  let dirs = match getDirs(vintagesRoot) {
    Ok(x)    => x,
    Err(_e)  => return Ok(Vec::new())
  };

  let mut vintagesDirs: Vec<String> = dirs.into_iter().filter( |pathStr| !pathStr.ends_with("_busy") ).collect();
  vintagesDirs.sort();
  vintagesDirs.reverse();
  Ok(vintagesDirs)
}

/// gets *ready* vintages only
/// sorted decreasing
fn getVintages(collectionName: &str, clxnsRoot: &str, pid: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let vintagesDirs = getVintageDirs(collectionName, clxnsRoot, pid)?;

  Ok(vintagesDirs.iter().map(|vDir| { lastPath(&vDir) }).collect())
}

fn getReadyMappedVintage_etAl(collectionName: &str, clxnsRoot: &str, pid: &str) -> Result<(Option<String>, Vec<String>,  Vec<String>), Box<dyn Error>> {

  // let vintagesRoot = &getDir_collectionVintages(collectionName, clxnsRoot, pid);

  // //TODO put this back.  (or dont) er, replace with something like "assertCollectionExists and assertVintageExists"
  // if !exists(vintagesRoot){
  //   // return Err(Box::new(WError { message: format!("vintagesRoot dir not found: {}", vintagesRoot)}));
  //   return Err(wErr(werr(&format!("vintagesRoot dir not found: {}", vintagesRoot)));
  // } ????????????

  let vintageDirs = getVintageDirs(collectionName, clxnsRoot, pid)?;

  // println!("vintages: \n{:?}", vintageDirs);

  //add vintages to add
  let mut vintageDirsToAddToDbsMap: Vec<String> = Vec::new();
  let mut vintageDirsToRetire: Vec<String> = Vec::new();

  let mut vToUse = st("");                                  //v for vintage
  for vDir in &vintageDirs {
    // println!("here 0 {:?}", &vDir);

    if vToUse == "" {                           // just keep the newest one ... mark others for deleting (but they wont get deleted if in use! that's checked later, at the last minute)
      let v = lastPath(&vDir);

      // println!("rrrr 0:  {}", v);
      if vintageIsInDbsMap(pid, &collectionName, &v) {
        // println!("rrrr 1");
        vToUse = v.to_string();
      } else {
        // println!("rrrr 2");
        vintageDirsToAddToDbsMap.push(st(vDir));
      }
    } else {
      // println!("rrrr 4  retire");
      vintageDirsToRetire.push(st(vDir));
    }
  }
  let readyMappedVintage = if vToUse == "" { None } else { Some(vToUse.to_string()) };
  Ok( (readyMappedVintage, vintageDirsToAddToDbsMap, vintageDirsToRetire)  )                   // todo ... we dont need to open db connections to all vintages. just newest one(s) right?
}

fn ensureCorrectSecret(secret: &str, isGlobalOption: &Option<bool>) -> Result<(), Box<dyn Error>> {

  let isGlobal: bool = match isGlobalOption {
    Some(b) => *b,
    None    => false
  };

  let expectedHash = if isGlobal {
    GlobalSecretHash.read().unwrap().as_ref().unwrap().to_string()
  }
  else {
    ProjectSecretHash.read().unwrap().as_ref().unwrap().to_string()
  };
  match argon2::verify_encoded(&expectedHash, &secret.as_bytes()).unwrap() {
    true => Ok(()),
    false => {
      if isGlobal { Err(werr(&format!("invalid global secret") )) }
      else        { Err(werr(&format!("invalid project secret") )) }
    }
  }
}

fn getRequestSubmittedResponse(topic: &str, requestId: &str) -> Result<String, Box<dyn Error>> {
  // Ok(format!("[{}] request submitted. requestId: \n{}", topic, requestId))
  let mut map : HashMap<&str, &str> = HashMap::new();
  map.insert("topic", topic);
  map.insert("request_id", requestId);

  Ok(serde_json::to_string_pretty(&map).unwrap())
}

fn enqueueTaskForScribe(pid: &str, collectionName: &str, requestId: &str, task: &str, payload: &str) {
  // holds tuple: (requestId, task, pid, collection, payload)
  // this assumes the queue exists in the map.  so we have to make sure the queue gets added at startup and whenever a collection is created
        // no that's stupid
  // M___PID_COLLECTION___Q__SCRIBE_TASKS.lock().unwrap().get_mut(&(st(pid), st(collectionName))).unwrap().add((  st(requestId),  st(task),  st(pid),  st(collectionName),  st(payload)  ));
  M___PID_COLLECTION___Q__SCRIBE_TASKS.lock().unwrap().entry((st(pid), st(collectionName))).or_insert(Queue::new()).add((  st(requestId),  st(task),  st(pid),  st(collectionName),  st(payload)  ));
}

fn getResponseFromScribe(requestId: &str) -> Result<String, Box<dyn Error>> {
  let mut count = 0;
  let maxCount = 100;
  loop {
    count += 1;
    if count > maxCount {
      break;
    }
    {
      let map = MScribeResponses__REQUESTID_PAYLOAD.lock().unwrap();
      if map.contains_key(requestId){
        let payload = &map[requestId];
        if payload.0 {
          return Ok(payload.1.to_string());
        }
        else {
          return Err(werr(&payload.1));
        }
      }
    }
    sleep(time::Duration::from_millis(1000));
  }
  Err(werr(&format!("timed out polling MScribeResponses__REQUESTID_PAYLOAD for requestID {}", requestId)))
}


fn enqueueQueryDocsForScribe(pid: &str, collectionName: &str, queries: Vec<WorpdriveQueryChild>) {

  let mut queryStringsSet: HashSet<String> = HashSet::new();          // to remove dups
  for q in queries {
    if q.query != "*" {
      queryStringsSet.insert(q.query.to_string());
    }
  }
  let collectionName_hqc = &format!("{}_hiddenQueryCollection", collectionName);

  for q in queryStringsSet {

    let mut docJsonMap: HashMap<&str, &str> = HashMap::new();
    docJsonMap.insert("query", &q);

    let docJsonStr = serde_json::to_string(&docJsonMap).unwrap();

    // println!("about to index hidden queries doc: {}, clxn: {}", docJsonStr, collectionName_hqc);

    enqueueTaskForScribe(pid, collectionName_hqc, "_", INDEX_DOCUMENT, &docJsonStr);
    // M___PID_COLLECTION___Q__SCRIBE_TASKS.lock().unwrap().add((  st("_"),  st(INDEX_DOCUMENT),  st(pid),  st(collectionName_hqc),  docJsonStr ));
  }
}

fn convertResultToScribePayload(r: Result<String, Box<dyn Error>>, topic: &str) -> (bool, String, String) {
  match r {
    Ok(msg) => (true, msg, st(topic)),
    Err(e)  => (false, e.to_string(), st(topic))
  }
}

fn respond(requestId: &str, result: Result<String, Box<dyn Error>>, topic: &str) {
  let mut map = MScribeResponses__REQUESTID_PAYLOAD.lock().unwrap();
  let scribePayload = convertResultToScribePayload(result, topic);
  map.insert(st(requestId), scribePayload);    // now poll this in the getter ...
}

fn startSamTarly(clxnsRoot: &str) -> Result<(), Box<dyn Error>> {
  println!("starting SamTarly clxnsRoot:{}", clxnsRoot);

  for pid in &getProjects(clxnsRoot)? {
    println!("loopin   pid {}", pid);
    for collection in &getProjectCollections(clxnsRoot, pid)? {
      println!("loopin   pid, collection: {} {}", pid, collection);
      startScribe(st(pid), st(collection));
    }
  }

  let _handle = std::thread::spawn( move || {
    loop {
      let mut didStuff = false;

      let mut queuedItem = (st(""),st(""),st(""),st(""),st(""));

      {                                                       //code block to give back the mutex lock asap
        let mut map = M___PID_COLLECTION___Q__SCRIBE_TASKS.lock().unwrap();
        let mut q = map.entry((st(SAM_TARLY_DUMMY_PID), st(SAM_TARLY_DUMMY_COLLECTION))).or_insert(Queue::new());     //https://stackoverflow.com/a/30414450/8870055
        if q.size() > 0 {
          queuedItem = q.remove().unwrap();
          didStuff = true;
        }
      }

      if queuedItem.0.len() > 0 {
        // println!("startSamTarly polled, de-queued item: {:?}", queuedItem);
      }

      let requestId =   &queuedItem.0;
      let topic =       &queuedItem.1;
      let pid =         &queuedItem.2;
      let collection =  &queuedItem.3;
      let payload =     &queuedItem.4;

      if CREATE_COLLECTION == topic {
        let o: WorpdriveSchema_userInput = serde_json::from_str(payload).unwrap();

        match createCollection_fromObj(o) {
          Ok(schema) => {
            startScribe(st(pid), st(collection));                                             // only do this if collection created successfully.  this is the reason samTarly exists. cos parent of new scribe needs to stay active

            let s: WorpdriveSchema = serde_json::from_str(&schema).unwrap();

            if s.do_log_queries {
              startScribe(st(pid), format!("{}_hiddenQueryCollection", collection));
            }
            respond(requestId, Ok(schema), topic);
          },
          Err(e)     => respond(requestId, Err(e), topic)
        }

        // let schema = &createCollection_fromObj(o).unwrap().to_string();
        // let s: WorpdriveSchema = serde_json::from_str(schema).unwrap();

        // if s.do_log_queries {
        //   startScribe(st(pid), format!("{}_hiddenQueryCollection", collection));
        // }
        // respond(requestId, Ok(st(schema)));
      }
      sleep(time::Duration::from_millis( if didStuff {millisToWait_active} else {millisToWait_inactive} ));
    }
  });
  Ok(())
}

fn unsetScribeStatus(pid: &str, collection: &str) -> Result<(), Box<dyn Error>> {
  M___PID_COLLECTION___STATUS.write().unwrap().remove(&(st(pid), st(collection)));
  Ok(())
}

fn setScribeStatus(pid: &str, collection: &str, requestId: &str, topic: &str, message: &str) -> Result<(), Box<dyn Error>> {
  //https://trello.com/c/UrMIlUqU/135-scribe-status-v0035

  let epochMillis = getEpochMs().to_string();
  let timeIso =  Utc::now().to_string();

  let mut map: HashMap<&str, &str> = HashMap::new();
  map.insert("start_time_utc_iso", &timeIso);
  map.insert("start_time_utc_millis", &epochMillis);
  map.insert("topic", topic);
  map.insert("message", message);
  map.insert("request_id", requestId);

  let status = serde_json::to_string_pretty(&map).unwrap();

  M___PID_COLLECTION___STATUS.write().unwrap().insert((st(pid), st(collection)), status);
  Ok(())
}


///scribe is a single ferret process that is always running.  constanly polling this queue for new docs to index into any collection
/// scribe is the only process that's allowed to write to squirrel dbs
/// shoudl scribe be renamed to squirrel ?????
fn startScribe(pid: String, collection: String) {
  println!("starting startScribe to poll M___PID_COLLECTION___Q__SCRIBE_TASKS ...   pid, collection: {} {}", pid, collection);

  let _handle = std::thread::spawn( move || {
    loop {
      let mut didStuff = false;

      let mut queuedItem = (st(""),st(""),st(""),st(""),st(""));

      {                                                       //code block to give back the mutex lock asap
        let mut map = M___PID_COLLECTION___Q__SCRIBE_TASKS.lock().unwrap();
        let mut q = map.entry((st(&pid), st(&collection))).or_insert(Queue::new());     //https://stackoverflow.com/a/30414450/8870055
        if q.size() > 0 {
          queuedItem = q.remove().unwrap();
          didStuff = true;
        }
      }
      // println!("startScribe polled, pid, collection, de-queued item: {} {} {:?}", pid, collection, queuedItem);

      if queuedItem.0.len() > 0 {
        // println!("startScribe polled, de-queued item: {:?}", queuedItem);
      }

      let requestId =   &queuedItem.0;
      let topic =       &queuedItem.1;
      let pid =         &queuedItem.2;
      let collection =  &queuedItem.3;
      let payload =     &queuedItem.4;


      if INDEX_DOCUMENT == topic {                                         // match cases would be easier to read
        setScribeStatus(pid, collection, requestId, topic, payload);
        let docId = indexDocInCollection(pid, collection, payload);
        let mut map: HashMap<&str, usize> = HashMap::new();
        map.insert("_id", docId);
        let mapStr = serde_json::to_string(&map).unwrap();
        unsetScribeStatus(pid, collection);
        respond(requestId, Ok(mapStr), topic);
      }
      else if INCREMENT_BILLING_METRIC_READS == topic {
        incrementBillingMetricReads(pid, collection);
      }
      else if INCREMENT_BILLING_METRIC_WRITES == topic {
        incrementBillingMetricWrites(pid, collection);
      }
      else if GET_ALL_META_OBJECTS == topic {
        let allMetaJsonPrettyStr = getAllMetaJsonPrettyStr(SQUIRREL_COLLECTIONS_ROOT, pid, collection, DEFAULT_VINTAGE, false);
        respond(requestId, allMetaJsonPrettyStr, topic);
      }
      else if MODIFY_FIELD_ATTRIBUTE == topic {
        let o: ModifyFieldAttribute = serde_json::from_str(payload).unwrap();
        setScribeStatus(pid, collection, requestId, topic, payload);
        let mfaResponse = modifyFieldAttribute(&o.pid, &o.collection, &o.field, &o.attribute, o.value);
        unsetScribeStatus(pid, collection);
        respond(requestId, mfaResponse, topic);
      }
      else if DELETE_COLLECTION == topic {
        setScribeStatus(pid, collection, requestId, topic, "");
        let res = deleteCollection(pid, collection);
        unsetScribeStatus(pid, collection);
        respond(requestId, res, topic);
      }
      else if INDEX_BULK == topic {
        setScribeStatus(pid, collection, requestId, topic, &format!("size of docs object = {} bytes", payload.len()));
        let res = indexBulk_ndjson(pid, collection, payload);
        unsetScribeStatus(pid, collection);
        respond(requestId, res, topic);
      }
      else if DELETE_DOC == topic {
        setScribeStatus(pid, collection, requestId, topic, "");
        //TODO temp sleep here to test status ...
        sleep(time::Duration::from_millis(10000));
        let res = deleteDocInCollection(pid, collection, payload.parse().unwrap());
        unsetScribeStatus(pid, collection);
        respond(requestId, res, topic);
      }
      else if RETIRE_SCRIBE == topic {
        // std::process::exit(1);   //lol no this kills everything
        return;
      }
      sleep(time::Duration::from_millis( if didStuff {millisToWait_active} else {millisToWait_inactive} ));
    }
  });
}

/// deletes vintages that aren't in use
fn cleanUpVintages(collectionName_: &str, clxnsRoot_copy: &'static str, pid_: &str) -> Result<(), WError> {
  // println!("cleanUpVintages: collection, clxnsRoot, pid: {} {} {}", collectionName_, clxnsRoot_copy, pid_);
  let collectionName = collectionName_.to_string();
  let pid = pid_.to_string();
  let (_vintage, vintageDirsToAddToDbsMap, vintageDirsToRetire) = getReadyMappedVintage_etAl(&collectionName, clxnsRoot_copy, &pid)?;

  // println!("_vintage, vintageDirsToAddToDbsMap, vintageDirsToRetire: \n{:?} \n{:?} \n{:?}", _vintage, vintageDirsToAddToDbsMap, vintageDirsToRetire);


  for vintageDir in vintageDirsToAddToDbsMap {
    addVintageToDbsMap(&collectionName, clxnsRoot_copy, &pid, &vintageDir);
  }

  let start = std::time::Instant::now();
  // let curr = currentTimeMillis();

  let _thread_one = std::thread::spawn( move || {
    for vintageDir in vintageDirsToRetire {
      let vintage = lastPath(&vintageDir);
      let mut vintageIsInUse = false;
      {
        let map = M__QID__INUSE_ClxnVintage.lock().unwrap();
        let keys : Vec<String> = map.keys().cloned().collect();
        for qid in keys {
          let (c, v) = map.get(&qid).unwrap();
          if c == &collectionName && v == &vintage {
            vintageIsInUse = true;
            break;
          }
        }
      }

      if !vintageIsInUse {
        removeVintageFromDbsMap(&collectionName, clxnsRoot_copy, &pid, &vintage);
        let _result = std::fs::remove_dir_all(&vintageDir);
      }
    }
  });
  // println!("deleteDuration: {:?}", start.elapsed());
  Ok(())
}

/// 1 vintner per machine.   so has to tend all projects
fn startVintner(clxnsRoot: &'static str) -> Result<(), Box<dyn Error>> {

  println!("starting vintner to poll ferret filesystem for vintage changes...");
  let _handle = std::thread::spawn( move ||  {

    let mut m_pidC_prevVintages: BTreeMap< (String,String), String > = BTreeMap::new();
    let mut didStuff = true;  //so it doesnt wait for first loop

    loop {
      sleep(time::Duration::from_millis( if didStuff {millisToWait_active} else {millisToWait_inactive} ));
      didStuff = false;

      let mut m_pidC_currVintages: BTreeMap< (String,String), String > = BTreeMap::new();   //btree for sane output debugging

      let vt_pid_collection = match getPidCollectionTuples(clxnsRoot) {
        Ok(c)  => c,
        Err(e) => {
          println!("error found, so gonna continue: {:?}", e);
          continue;
        }
      };

      for (pid, c) in vt_pid_collection {
        let vintages = getVintages(&c, clxnsRoot, &pid).unwrap();
        let vintagesStr = format!("{:?}", vintages);
        m_pidC_currVintages.insert((pid, c), vintagesStr);
      }

      for (prevPidC, _vintages) in &m_pidC_prevVintages {
        if !m_pidC_currVintages.contains_key(prevPidC) {
          // panic!("not sure about this ... vintage was deleted in the ferret ....maybe/probably fine, just unclear about it ... -stuart oct 30 2020");
          // this is fine. happens when a collection gets deleted.  do we need to do any more rocksroots map maintenance?
        }
      }

      for (currPidC, currVintages) in &m_pidC_currVintages {
        let pid = currPidC.0.to_string();
        let c = currPidC.1.to_string();
        if !m_pidC_prevVintages.contains_key(&currPidC) {
          // println!(" if !m_pidC_prevVintages.contains_key(&currPidC) !!!!  currPidC: {:?}", currPidC);
          match cleanUpVintages(&c, clxnsRoot, &pid) {                                                  //found a new vintage!  what do we do...  handle the collection like we would any collection that has changed vintages (i think...)
            Ok(()) => {},
            Err(e) => println!("cleanUpVintages error: {:?}", e)
          };
          didStuff = true;
        }
        else {
          let prevVintages = m_pidC_prevVintages.get(&currPidC).unwrap();
          if &prevVintages != &currVintages {
            match cleanUpVintages(&c, clxnsRoot, &pid) {                                                  //found a new vintage!  what do we do...  handle the collection like we would any collection that has changed vintages (i think...)
              Ok(()) => {},
              Err(e) => println!("cleanUpVintages error: {:?}", e)
            };
            didStuff = true;
          }
        }
      }
      if didStuff {
        println!("startVintner polled, clxnsRoot, m_pidC_prevVintages, m_pidC_currVintages: {}\n{:?}\n{:?}", &clxnsRoot, m_pidC_prevVintages, m_pidC_currVintages);
      }
      // println!("startVintner polled, clxnsRoot, m_pidC_prevVintages, m_pidC_currVintages: {}\n{:?}\n{:?}", &clxnsRoot, m_pidC_prevVintages, m_pidC_currVintages);
      // displayMap();

      m_pidC_prevVintages = m_pidC_currVintages;
    }
  });
  Ok(())
}

//getReadyMappedVintage_etAl
fn handleFerretQuery(query_userInput: WorpdriveQueryParent_userInput, clxnsRoot: &str) -> Result<String, Box<dyn Error>> {
  let collectionName = &query_userInput.collection.to_string();
  let pid = &query_userInput.pid.to_string();
  let qid = Uuid::new_v4().to_string();
  let (vintageOption, _, _) = getReadyMappedVintage_etAl(collectionName, clxnsRoot, pid)?;
  let vintage = match vintageOption {
    Some(v) => v,
    None    => return Err(werr(&format!("ready-mapped-vintage not found for clxnsRoot, collection, pid: {}, {}, {}", clxnsRoot, collectionName, pid)))
  };
  {
    let mut map = M__QID__INUSE_ClxnVintage.lock().unwrap();
    map.insert(qid.to_string(), (collectionName.to_string(), vintage.to_string()));                             //mark the vintage as "in use" so it doens't get deleted if it's old
  }
  let schema = readSchema(clxnsRoot, pid, &collectionName, &vintage, true);
  let queryParent = sanitizeQueryUserInput(query_userInput, &schema);
  let (_response, jsonResponse) = queryClxLocal_clxnsRoot(&queryParent, &schema, &clxnsRoot, &vintage)?;
  {
    let mut map = M__QID__INUSE_ClxnVintage.lock().unwrap();
    map.remove(&qid);                                                                                               //release the vintage from "in use"
  }
  if queryParent.do_log_query_for_analytics && !collectionName.contains("_hiddenQueryCollection"){       //TODO take out second part after website checks for do_log_query_for_analytics correctly
    enqueueQueryDocsForScribe(&pid, &collectionName, queryParent.queries);
  }
  enqueueTaskForScribe(pid, collectionName, "_", INCREMENT_BILLING_METRIC_READS, "_");              //mark the vintage as "in use" so it doens't get deleted if it's old

  Ok(format!("{}", jsonResponse))
}

fn handle(res: Result<String, Box<dyn Error>>) -> Result<warp::http::Response<String>, warp::http::Error> {
  match res{
    Ok(r)  => Response::builder().status(200).body(st(&r)),
    Err(e) => Response::builder().status(StatusCode::BAD_REQUEST).body(e.to_string())
  }
}

/// TODO - this should pull these password hashes from posgtres, not from hardcoded values
fn initializeSecrets() -> Result<(), Box<dyn Error>> {
  *ProjectSecretHash.write().unwrap() = Some(st(PROJECT_SECRET_HASH_HARDCODED_temporary_shouldComeFromPostgres));
  *GlobalSecretHash.write().unwrap() = Some(st(GLOBAL_SECRET_HASH_HARDCODED_temporary_shouldComeFromPostgres));
  Ok(())
}

#[tokio::main]
pub async fn startFerret(port: u16) -> Result<(), Box<dyn Error>> {

  println!("starting ferret at port {}", port);
  println!("some handy links:");
  println!("http://localhost:{}/list_projects", port);
  println!("http://localhost:{}/list_collections?pid=w1", port);
  println!("http://localhost:{}/list_docs?collection=c1&pid=w1)", port);
  println!("http://localhost:{}/meta?collection=c1&pid=w1", port);
  println!("http://localhost:{}/query?collection=c1&pid=w1&query=reali", port);
  println!("http://localhost:{}/scribe_queue?collection=c1&pid=w1                                 <-- the scribe is the single-threaded process that makes changes to indexed data", port);
  println!("http://localhost:{}/scribe_responses", port);
  println!("http://localhost:{}/scribe_response?request_id=94235c0f-050b-4bee-b036-cef11bf3828f   <-- a request_id is returned with every modification request (like indexing a doc)", port);

  println!();
  println!();
  unsafe { QUERY_MODE = true; }
  std::env::set_var("RUST_BACKTRACE", "1");

  let clxnsRoot = FERRET_COLLECTIONS_ROOT;                      // should this even be a var?  shouldjust use ferret const everywhere?....

  resetStaticMaps_clxnsRoot(&clxnsRoot, true);                              //so indexing into the HQC should be fine cos that's the squirrel's clxnsRoot

  let clxnsRoot_copy = clxnsRoot.to_string();

  startSamTarly(&clxnsRoot)?;
  startVintner(&clxnsRoot)?;
  initializeSecrets()?;                                  // TODO - should pull password hashes from posgtres, not from hardcoded values

  // POST /query  <query json object>
  let query = warp::post()
    .and(warp::path("query"))
    .and(warp::body::content_length_limit(1024 * 16))
    .and(warp::body::json())
    .map(move |query_userInput: WorpdriveQueryParent_userInput| {
      handle(|| -> Result<String, Box<dyn Error>> {

        ensureCollectionExists(&st(&query_userInput.pid), &st(&query_userInput.collection))?;

        handleFerretQuery(query_userInput, &clxnsRoot_copy)

      }())
    });

  // GET /query?collection=stories&pid=npr    // uses the query string to populate a custom object
  let clxnsRoot_copy = clxnsRoot.to_string();
  let queryGet = warp::get()
      .and(warp::path("query"))
      .and(warp::query::<PidAndCollectionAndQuery>())
      .map(move |p: PidAndCollectionAndQuery| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let collectionName = p.collection;
          let pid = p.pid;
          let q = p.query;
          ensureCollectionExists(&pid, &collectionName)?;
          let query_userInput: WorpdriveQueryParent_userInput = WorpdriveQueryParent_userInput {
            collection: collectionName,
            pid: pid,
            queries: Some(vec![WorpdriveQueryChild_userInput {query: Some(q), collection: None, fields: None, doPrefixLast: None}]),        // TODO makea  constructor for all this junk?
            do_log_query_for_analytics:  None,
            num_results_per_page: None,
            page_number: None,
            sort_by: None,
            fields_to_return: None,
            do_highlights_tagged: None,
            do_highlights_map: None,
            highlight_pre_tag: None,
            highlight_post_tag: None,
            min_highlight_context: None,
            max_total_snippets_length: None,
            do_highlights_objects: None
          };
          handleFerretQuery(query_userInput, &clxnsRoot_copy)
        }())
      });



  // POST /expand_query <query json object>
  let clxnsRoot_copy = clxnsRoot.to_string();
  let expand_query = warp::post()
      .and(warp::path("expand_query"))
      .and(warp::body::content_length_limit(1024 * 16))
      .and(warp::body::json())
      .map(move |query_userInput: WorpdriveQueryParent_userInput| {
        handle(|| -> Result<String, Box<dyn Error>> {

          let collectionName = &query_userInput.collection;
          let pid = &query_userInput.pid;
          ensureCollectionExists(pid, collectionName)?;

          let (vintageOption, vintageDirsToAddToDbsMap, vintageDirsToRetire) = getReadyMappedVintage_etAl(&collectionName, &clxnsRoot_copy, &pid)?;
          let vintage = match vintageOption {
            Some(v) => v,
            None    => return Err(werr(&format!("ready-mapped-vintage not found for clxnsRoot, collection, pid: {}, {}, {}", clxnsRoot, collectionName, pid)))
          };
          expandQuery(query_userInput, &clxnsRoot_copy, &vintage, true)
        }())
      });

  // GET /meta?collection=stories&pid=npr    // uses the query string to populate a custom object
  let clxnsRoot_copy = clxnsRoot.to_string();
  let meta = warp::get()
      .and(warp::path("meta"))
      .and(warp::query::<PidAndCollection>())
      .map(move |p: PidAndCollection| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let collectionName = p.collection;
          let pid = p.pid;
          ensureCollectionExists(&pid, &collectionName)?;

          // let (vintage, vintageDirsToAddToDbsMap, vintageDirsToRetire) = getReadyMappedVintage_etAl(&collectionName, &clxnsRoot_copy, &pid)?;
          let requestId = &Uuid::new_v4().to_string();
          enqueueTaskForScribe(&pid, &collectionName, requestId, GET_ALL_META_OBJECTS, "_");
          getResponseFromScribe(requestId)
        }())
      });

  // GET /list_docs?collection=stories&pid=npr    // uses the query string to populate a custom object
  let clxnsRoot_copy = clxnsRoot.to_string();
  let listDocs = warp::get()
      .and(warp::path("list_docs"))
      .and(warp::query::<PidAndCollection>())
      .map(move |p: PidAndCollection| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let collectionName = p.collection;
          let pid = p.pid;
          ensureCollectionExists(&pid, &collectionName)?;

          let (vintageOption, vintageDirsToAddToDbsMap, vintageDirsToRetire) = getReadyMappedVintage_etAl(&collectionName, &clxnsRoot_copy, &pid)?;
          let vintage = match vintageOption {
            Some(v) => v,
            None    => return Err(werr(&format!("ready-mapped-vintage not found for clxnsRoot, collection, pid: {}, {}, {}", clxnsRoot, collectionName, pid)))
          };
          let allMetaJsonPrettyStr = getAllDocsJsonPrettyStr(&clxnsRoot_copy, &pid, &collectionName, &vintage, true);
          Ok(allMetaJsonPrettyStr)
        }())
      });


  #[derive(Deserialize, Serialize)]
  #[serde(deny_unknown_fields)]
  struct PidOnly {
      pid: String
  }

  // GET /list_collections?&pid=npr
  let clxnsRoot_copy = clxnsRoot.to_string();
  let listCollections = warp::get()
      .and(warp::path("list_collections"))
      .and(warp::query::<PidOnly>())
      .map(move |p: PidOnly| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let pid = p.pid;
          ensureProjectExists(&pid)?;
          let collections = getProjectCollections(FERRET_COLLECTIONS_ROOT, &pid)?;
          let mut map: HashMap<&str, Vec<String>> = HashMap::new();
          map.insert("data", collections);
          let mapStr = serde_json::to_string(&map).unwrap();
          Ok(mapStr)
        }())
      });

  // GET /list_projects
  let clxnsRoot_copy = clxnsRoot.to_string();
  let listProjects = warp::get()
      .and(warp::path("list_projects"))
      .map(move || {
        handle(|| -> Result<String, Box<dyn Error>> {
          let pids = getProjects(FERRET_COLLECTIONS_ROOT)?;   ////getProjectCollections(FERRET_COLLECTIONS_ROOT, &pid)?;
          let mut map: HashMap<&str, Vec<String>> = HashMap::new();
          map.insert("data", pids);
          let mapStr = serde_json::to_string(&map).unwrap();
          Ok(mapStr)
        }())
      });

  // GET /get_data_size?collection=c1&pid=w1
  let clxnsRoot_copy = clxnsRoot.to_string();
  let getIndexDataSizeForCollection = warp::get()
      .and(warp::path("get_data_size"))
      .and(warp::query::<PidAndCollection>())
      .map(move |p: PidAndCollection| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let pid = p.pid;
          let collection = p.collection;
          ensureCollectionExists(&pid, &collection)?;

          let (vintageOption, vintageDirsToAddToDbsMap, vintageDirsToRetire) = getReadyMappedVintage_etAl(&collection, &clxnsRoot_copy, &pid)?;
          let vintage = match vintageOption {
            Some(v) => v,
            None    => return Err(werr(&format!("ready-mapped-vintage not found for clxnsRoot, collection, pid: {}, {}, {}", clxnsRoot, collection, pid)))
          };
          let numBytes: String = getCollectionDataSize(FERRET_COLLECTIONS_ROOT, &pid, &collection, &vintage).to_string();
          let mut map: HashMap<&str, String> = HashMap::new();
          map.insert("data", numBytes);
          let mapStr = serde_json::to_string(&map).unwrap();
          Ok(mapStr)
        }())
      });



  // GET /get_data_size?collection=c1&pid=w1
  let clxnsRoot_copy = clxnsRoot.to_string();
  let getSortbys_endpoint = warp::get()
      .and(warp::path("get_sortbys"))
      .and(warp::query::<PidAndCollection>())
      .map(move |p: PidAndCollection| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let pid = p.pid;
          let collection = p.collection;
          ensureCollectionExists(&pid, &collection)?;

          let (vintageOption, vintageDirsToAddToDbsMap, vintageDirsToRetire) = getReadyMappedVintage_etAl(&collection, &clxnsRoot_copy, &pid)?;
          let vintage = match vintageOption {
            Some(v) => v,
            None    => return Err(werr(&format!("ready-mapped-vintage not found for clxnsRoot, collection, pid: {}, {}, {}", clxnsRoot, collection, pid)))
          };
          let sortbys = getSortBys(FERRET_COLLECTIONS_ROOT, &pid, &collection, &vintage, true);
          let mut map: HashMap<&str, Vec<String>> = HashMap::new();
          map.insert("data", sortbys);
          let mapStr = serde_json::to_string(&map).unwrap();
          Ok(mapStr)
        }())
      });


  // PUT /create_collection       {object}
  let clxnsRoot_copy = clxnsRoot.to_string();
  let createCollection = warp::put()
      .and(warp::path("create_collection"))
      // .and(warp::query::<WorpdriveSchema_userInput>())
      .and(warp::body::json())
      .map(move |mut p: WorpdriveSchema_userInput| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let requestId = &Uuid::new_v4().to_string();
          let pid = &p.pid;
          let secret = match &p.global_secret {
            Some(s) => s.to_string(),
            None    => return Err(werr(&format!("missing field [global_secret]")))
          };
          
          // println!("WorpdriveSchema_userInput: p: {:?}", p);
          ensureCorrectSecret(&secret, &Some(true))?;
          let collection = &p.collection;
          enqueueTaskForScribe(SAM_TARLY_DUMMY_PID, SAM_TARLY_DUMMY_COLLECTION, requestId, CREATE_COLLECTION, &serde_json::to_string(&p).unwrap());
          // getResponseFromScribe(requestId)
          getRequestSubmittedResponse(CREATE_COLLECTION, requestId)
        }())
      });

  // DELETE /delete_collection?&pid=npr&collection
  let clxnsRoot_copy = clxnsRoot.to_string();
  let deleteCollection = warp::delete()
      .and(warp::path("delete_collection"))
      .and(warp::query::<PidAndCollectionAndSecret>())
      // .and(warp::body::json())
      .map(move |p: PidAndCollectionAndSecret| {
        handle(|| -> Result<String, Box<dyn Error>> {

          ensureCorrectSecret(&p.secret, &p.is_global_secret)?;

          ensureCollectionExists(&p.pid, &p.collection)?;

          let requestId = &Uuid::new_v4().to_string();
          enqueueTaskForScribe(&p.pid, &p.collection, requestId, DELETE_COLLECTION, "_");
          // let response = getResponseFromScribe(requestId);
          enqueueTaskForScribe(&p.pid, &p.collection, requestId, RETIRE_SCRIBE, "_");


          let collection_HQC = &format!("{}_hiddenQueryCollection", &p.collection);

          enqueueTaskForScribe(&p.pid, collection_HQC, requestId, DELETE_COLLECTION, "_");
          enqueueTaskForScribe(&p.pid, collection_HQC, requestId, RETIRE_SCRIBE, "_");

          // response

          getRequestSubmittedResponse(DELETE_COLLECTION, requestId)
        }())
      });

  // PATCH /modify_field_attribute            possible to get optional like this too? or is taht stupid: ?pid=<>&collection=<>field=<>&attribute=<>&value=<t/f>
  let clxnsRoot_copy = clxnsRoot.to_string();
  let mfa = warp::patch()
      .and(warp::path("modify_field_attribute"))
      .and(warp::body::json())
      .map(move |p: ModifyFieldAttribute| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let collectionName = &p.collection;
          let pid = &p.pid;
          ensureCorrectSecret(&p.secret, &p.is_global_secret)?;
          ensureCollectionExists(pid, collectionName)?;
          let mfaJsonStr = &serde_json::to_string_pretty(&p)?;
          let requestId = &Uuid::new_v4().to_string();
          enqueueTaskForScribe(pid, collectionName, requestId, MODIFY_FIELD_ATTRIBUTE, mfaJsonStr);
          // getResponseFromScribe(requestId)
          getRequestSubmittedResponse(MODIFY_FIELD_ATTRIBUTE, requestId)
        }())
    });

  // put /index_bulk?collection=stories&pid=npr          ndjson
  let clxnsRoot_copy = clxnsRoot.to_string();
  let indexBulkEndpoint = warp::put()
      .and(warp::path("index_bulk"))
      .and(warp::body::content_length_limit(1024 * 16 * 1000))   //PidAndCollectionAndBulkDocsArray
      .and(warp::body::json())
      .map(move |q: PidAndCollectionAndBulkDocsArray| {
        handle(|| -> Result<String, Box<dyn Error>> {
          // let docsStr = &String::from_utf8(bytes.to_vec()).unwrap();

          ensureCorrectSecret(&q.secret, &q.is_global_secret)?;
          ensureCollectionExists(&q.pid, &q.collection)?;

          let docsValues = q.documents;
          let docsStrs: Vec<String> = docsValues.iter().map(|v| serde_json::to_string(&v).unwrap()).collect();
          let mut docsStr = format!("");
          for doc in docsStrs {
            println!("doc: {}", doc);
            docsStr = format!("{}\n{}", docsStr, doc);
          }
          println!("docsStr: \n{}", docsStr);
          docsStr = docsStr.trim().to_string();
          let requestId = &Uuid::new_v4().to_string();
          enqueueTaskForScribe(&q.pid, &q.collection, requestId, INDEX_BULK, &docsStr);
          // getResponseFromScribe(requestId)
          getRequestSubmittedResponse(INDEX_BULK, requestId)
        }())
    });

  // put /index_document {obj}
  let clxnsRoot_copy = clxnsRoot.to_string();
  let indexDocEndpoint = warp::put()
      .and(warp::path("index_document"))
      .and(warp::body::content_length_limit(1024 * 16 * 1000))   //PidAndCollectionAndBulkDocsArray
      .and(warp::body::json())
      .map(move |p: PidAndCollectionAndDocument| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let collection = &p.collection;
          let pid = &p.pid;
          ensureCorrectSecret(&p.secret, &p.is_global_secret)?;
          ensureCollectionExists(pid, collection)?;
          let docValue = &p.document;
          let docStr: String = serde_json::to_string(docValue).unwrap();
          let requestId = &Uuid::new_v4().to_string();
          enqueueTaskForScribe(pid, collection, requestId, INDEX_DOCUMENT, &docStr);
          // getResponseFromScribe(requestId)
          getRequestSubmittedResponse(INDEX_DOCUMENT, requestId)
        }())
    });

  // delete /delete_doc {object pid, collection, docId (u32)}
  let clxnsRoot_copy = clxnsRoot.to_string();
  let deleteDoc = warp::delete()
      .and(warp::path("delete_document"))
      .and(warp::query::<PidAndCollectionAndDocId>())
      .map(move |o: PidAndCollectionAndDocId| {
        handle(|| -> Result<String, Box<dyn Error>> {
          ensureCorrectSecret(&o.secret, &o.is_global_secret)?;
          ensureCollectionExists(&o.pid, &o.collection)?;

          let requestId = &Uuid::new_v4().to_string();
          enqueueTaskForScribe(&o.pid, &o.collection, requestId, DELETE_DOC, &o.docId.to_string());
          // enqueueTaskForScribe(&o.pid, &o.collection, requestId, DELETE_DOC, &serde_json::to_string(&o).unwrap());
          // getResponseFromScribe(requestId)   //dont wait around TODO this is just for testing awefawef put this back
          // Ok(format!("[delete_document] request submitted. requestId: {}", requestId))
          getRequestSubmittedResponse(DELETE_DOC, requestId)
        }())
    });

  // GET /collection_scribe_status?collection=c1&pid=w1
  let clxnsRoot_copy = clxnsRoot.to_string();
  let collection_scribe_status = warp::get()
      .and(warp::path("collection_scribe_status"))
      .and(warp::query::<PidAndCollection>())
      .map(move |p: PidAndCollection| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let pid = &p.pid;
          let collection = &p.collection;
          ensureCollectionExists(pid, collection)?;
          let map = M___PID_COLLECTION___STATUS.read().unwrap();

          // let statusJsonStr = match map.get(&(st(pid), st(collection))) {
          match map.get(&(st(pid), st(collection))) {
            Some(s) => Ok(st(s)),
            None    => Ok(st("ready"))
          }
          // Ok(st(statusJsonStr))
        }())
      });

  let clxnsRoot_copy = clxnsRoot.to_string();
  let scribe_queue = warp::get()
      .and(warp::path("scribe_queue"))
      .and(warp::query::<PidAndCollection>())
      .map(move |p: PidAndCollection| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let pid = &p.pid;
          let collection = &p.collection;
          ensureCollectionExists(pid, collection)?;

          let mut qStr = String::new();
          {
            let mm = M___PID_COLLECTION___Q__SCRIBE_TASKS.lock().unwrap();
            let qqqq = mm.get(&(st(pid), st(collection))).unwrap();

            qStr = format!("{:?}", qqqq);
          }
          Ok(qStr)
        }())
      });

  let scribe_responses = warp::get()
      .and(warp::path("scribe_responses"))
      // .and(warp::query::<PidAndCollection>())
      // .map(move |p: PidAndCollection| {
      .map(move || {
        handle(|| -> Result<String, Box<dyn Error>> {
          // let pid = &p.pid;
          // let collection = &p.collection;
          // ensureCollectionExists(pid, collection)?;

          let mut qStr = String::new();
          {
            let mm = MScribeResponses__REQUESTID_PAYLOAD.lock().unwrap();
            // let qqqq = mm.get(&(st(pid), st(collection))).unwrap();

            qStr = format!("{:?}", mm);
            // qStr = serde_json::to_string_pretty(&mm).unwrap();
          }
          Ok(qStr)
        }())
      });

  let scribe_response = warp::get()
      .and(warp::path("scribe_response"))
      .and(warp::query::<RequestId>())
      .map(move |p: RequestId| {
        handle(|| -> Result<String, Box<dyn Error>> {
          let requestId = &p.request_id;

          let mut qStr = String::new();
          let mut didSucceed = false;
          let mut message = String::new();
          let mut topic = String::new();
          {
            let mm = MScribeResponses__REQUESTID_PAYLOAD.lock().unwrap();
            let payload =  match mm.get(requestId) {
              Some(p) => p,
              None    => return Err(werr(&format!("request_id not found: {}", requestId) ))
            };
            didSucceed = payload.0;
            message = payload.1.to_string();
            topic = payload.2.to_string();
          }

          let mut map : HashMap<&str, Value> = HashMap::new();

          map.insert("topic", serde_json::json![topic]);
          map.insert("did_succeed", serde_json::json![didSucceed]);
          map.insert("message", serde_json::json![message]);

          Ok(serde_json::to_string_pretty(&map).unwrap())

          // Ok(qStr)
        }())
      });


  #[derive(Deserialize, Serialize)]
  struct MyObject {
      key1: String,
      key2: u32,
  }

  // get /example2?key1=value&key2=42
  // uses the query string to populate a custom object    //https://github.com/seanmonstar/warp/blob/master/examples/query_string.rs
  let example2 = warp::get()
      .and(warp::path("example2"))
      .and(warp::query::<MyObject>())
      .map(|p: MyObject| {
          Response::builder().body(format!("key1 = {}, key2 = {}", p.key1, p.key2))
      });

  #[derive(Deserialize, Serialize)]
  #[serde(deny_unknown_fields)]
  struct Employee {
      name: String,
      rate: u32,
  }

  // POST /employees/:rate  {"name":"Sean","rate":2}    https://github.com/seanmonstar/warp/blob/b9637567f96c64f1bfe734842718e28a7c633e23/examples/body.rs
  let employees = warp::post()
      .and(warp::path("employees"))
      .and(warp::path::param::<u32>())
      // Only accept bodies smaller than 16kb...
      .and(warp::body::content_length_limit(1024 * 16))
      .and(warp::body::json())
      .map(|rate, mut employee: Employee| {
          employee.rate = rate;
          warp::reply::json(&employee)
      });

  // POST /employees2  {"name":"Sean","rate":2}    https://github.com/seanmonstar/warp/blob/b9637567f96c64f1bfe734842718e28a7c633e23/examples/body.rs
  let employees2 = warp::post()
      .and(warp::path("employees2"))
      .and(warp::body::content_length_limit(1024 * 16))
      .and(warp::body::json())
      .map(|e: Employee| Response::builder().status(200).body(format!("{}, {}", e.name, e.rate)));


  let hi = warp::path!("hi" / String)
      .map(|name| format!("hi , {}!", name));

  let help = warp::path!("help")
      .map(|| "dont worry be happy");

  let cors = warp::cors()
                        .allow_any_origin()
                        .allow_headers(vec!["User-Agent", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "content-type"])
                        .allow_methods(vec!["POST", "GET", "PUT", "PATCH", "DELETE"]);

  let routes =  //expand_query
                  hi
                  .or(help)
                  .or(example2)
                  .or(employees)
                  .or(employees2)
                  .or(query)
                  .or(queryGet)
                  .or(expand_query)
                  .or(meta)               //TODO why does meta break it??????
                  .or(listDocs)
                  .or(listCollections)
                  .or(listProjects)
                  .or(getSortbys_endpoint)
                  .or(getIndexDataSizeForCollection)
                  .or(createCollection)                 // X require secret!
                  .or(mfa)                             // X require secret!
                  .or(indexBulkEndpoint)                 //X require secret!
                  .or(indexDocEndpoint)                 // X require secret!
                  .or(deleteCollection)                 // X require secret!
                  .or(deleteDoc)                      // X require secret!
                  .or(collection_scribe_status)
                  .or(scribe_queue)
                  .or(scribe_responses)
                  .or(scribe_response)
                  .with(cors);


  Ok(warp::serve(routes).run(([127, 0, 0, 1], port)).await)
}
