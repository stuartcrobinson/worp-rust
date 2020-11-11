use crate::h::*;
use crate::stateManagement::*;

use std::io;
use std::error::Error;
// use crate::worpError::WError;
use crate::worpError::werr;


/* ############################################################################################################################## */
/* ######################################################   util    ############################################################# */
/* ############################################################################################################################## */


pub fn addPaths(root: &str, paths: Vec<&str>) -> String {
  let mut pathBuilder = std::path::PathBuf::from(root);
  for path in paths {
    pathBuilder.push(path);
  }
  return pathBuilder.to_str().unwrap().to_string();
}


pub fn createDirMaybe(path: &str){
  if !std::path::Path::new(&path).exists() {
      std::fs::create_dir_all(path);
  }
}

pub fn exists(path: &str) -> bool {
  return std::path::Path::new(&path).exists();
}

pub fn getDirs(path: &str) ->  Result<Vec<String>, Box<dyn Error>>{
  match std::fs::read_dir(path) {
    Ok(entries)  => Ok(entries
                              .map(|res| res.map(|e| e.path().to_str().unwrap().to_string()))
                              .collect::<Result<Vec<_>, io::Error>>()
                              .unwrap()),
    Err(_err)    => Err(werr(&format!("dir not found: {}", path)))
  }
}

pub fn getDirsLast(path: &str) -> std::vec::Vec<String>{

    return std::fs::read_dir(path)
        .expect(&format!("file: {}", path))
        .map(|res| res.map(|e| lastPath(&e.path().to_str().unwrap().to_string())))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();
}



/* ############################################################################################################################## */
/* ###################################################### "getters" ############################################################# */
/* ############################################################################################################################## */




pub fn getDir_projects(clxnsRoot: &str) -> String {
  addPath(clxnsRoot, PROJECTS_DIR_NAME)
}

pub fn getDir_project(clxnsRoot: &str, pid: &str) -> String {
  let projectsDir = &getDir_projects(clxnsRoot);
  addPath(projectsDir, pid)
}

// # TODO update this!  needs pid param.  get collections for a specific project (right?)

pub fn getProjects(clxnsRoot: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let projectsDir = &getDir_projects(clxnsRoot);
  match getDirs(projectsDir) {
    Ok(dirs) => Ok(dirs.iter().map(|projectDir| lastPath(&projectDir)).collect()),
    Err(e)   => Ok(Vec::new())
  }

  // Ok(dirs?.iter().map(|projectDir| lastPath(&projectDir)).collect())
}

/// accross all projects
pub fn getPidCollectionTuples(clxnsRoot: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
  let projects = getProjects(clxnsRoot)?;
  let mut pidCollectionTuples: Vec<(String, String)>  = Vec::new();
  for pid in projects {
    let collections = getProjectCollections(clxnsRoot, &pid)?;
    for c in collections {
      pidCollectionTuples.push((pid.to_string(), c.to_string()));
    }
  }
  Ok(pidCollectionTuples)
}

pub fn getProjectCollections(clxnsRoot: &str, pid: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let projectDir = &getDir_project(clxnsRoot, pid);

  match getDirs(projectDir) {
    Ok(dirs) => Ok(dirs.iter().map(|collectionParent| lastPath(&collectionParent)).collect()),
    Err(e)   => Ok(Vec::new())
  }

  // Ok(getDirs(projectDir)?.iter().map(|collectionParent| lastPath(&collectionParent)).collect())
}

pub fn getCollectionVintages(collection: &str, clxnsRoot: &str, pid: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let vintagesDir = &getDir_collectionVintages(collection, clxnsRoot, pid);

  match getDirs(vintagesDir) {
    Ok(dirs) => Ok(dirs.iter().map(|vDir| lastPath(&vDir)).collect()),
    Err(e)   => Ok(Vec::new())
  }

  // Ok(getDirs(vintagesDir)?.iter().map(|vDir| lastPath(&vDir)).collect())
}

pub fn getDir_collectionParent(collection: &str, clxnsRoot: &str, pid: &str) -> String {
  let projectDir = &getDir_project(clxnsRoot, pid);
  addPath(projectDir, collection)
}

pub fn getDir_collectionVintages(collection: &str, clxnsRoot: &str, pid: &str) -> String {
  let collectionParentRoot = &getDir_collectionParent(collection, clxnsRoot, pid);
  addPath(collectionParentRoot, VINTAGES_DIR_NAME)
}

pub fn getDir_collectionVintage(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str) -> String {
  let collectionVintages = &getDir_collectionVintages(collection, clxnsRoot, pid);
  addPaths(collectionVintages, vec![vintage])
}

pub fn getDir_main(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str) -> String {
  let clxnVintageRoot = &getDir_collectionVintage(collection, clxnsRoot, pid, vintage);
  addPath(clxnVintageRoot, MAIN_DIR_NAME)
}

/// literal actual shards.  not "shards" like used earlier that just meant anything
pub fn getDir_shards(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str) -> String {
  let clxnVintageRoot = &getDir_collectionVintage(collection, clxnsRoot, pid, vintage);
  addPath(clxnVintageRoot, SHARDS_DIR_NAME)
}

pub fn getDir_rocksRoot_meta(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str) -> String {
  let mainDir = &getDir_main(collection, clxnsRoot, pid, vintage);
  addPaths(mainDir, vec![SHARD_META, ROCKSDB_DIR_NAME])
}

pub fn getDir_rocksRoot_docs(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str) -> String {
  let mainDir = &getDir_main(collection, clxnsRoot, pid, vintage);
  addPaths(mainDir, vec![SHARD_DOCUMENTS, ROCKSDB_DIR_NAME])
}

pub fn getDir_rocksRoot_locs(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str) -> String {
  let mainDir = &getDir_main(collection, clxnsRoot, pid, vintage);
  addPaths(mainDir, vec![SHARD_LOCATIONS, ROCKSDB_DIR_NAME])
}

pub fn getDir_invindsParent(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str, shardNumber: &str) -> String {
  let shardsDir = &getDir_shards(collection, clxnsRoot, pid, vintage);
  addPaths(shardsDir, vec![shardNumber, INDEXES_ROOT_NAME])
}

pub fn getDir_rocksRoot_invind(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str, shardNumber: &str, fieldName: &str) -> String {

  let shardsDir = &getDir_shards(collection, clxnsRoot, pid, vintage);
  let result = addPaths(shardsDir, vec![shardNumber, INDEXES_ROOT_NAME, fieldName, ROCKSDB_DIR_NAME]);
  // println!("in getDir_rocksRoot_invind !! returning: {}", result);
  result
}

pub fn getDir_rocksRoot_sovabydid(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str, shardNumber: &str) -> String {
  let shardsDir = &getDir_shards(collection, clxnsRoot, pid, vintage);
  addPaths(shardsDir, vec![shardNumber, SHARD_SOVABYDID, ROCKSDB_DIR_NAME])
}

pub fn getDir_rocksRoot_tivabydid(collection: &str, clxnsRoot: &str, pid: &str, vintage: &str, shardName: &str) -> String {
  let shardsDir = &getDir_shards(collection, clxnsRoot, pid, vintage);
  addPaths(shardsDir, vec![shardName, SHARD_TIVABYDID, ROCKSDB_DIR_NAME])
}

/*

- clxnsRoot                     ("collections")
  - projects
    - project                     ("npr")
    - project                     ("etsy")
      - collectionParentRoot                 ("c1")
        - vintages
          - vintageRoot
            - main
              - documents
              - locations
              - meta
            - shards
              - 1
                - indexes
                  - f1
                  - f2
                  - s1
                - sovaybdids
              - 2
                - indexes
                - sovaybdids

*/
