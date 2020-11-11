// put helper functions in here to clean up main, etc
//stuff like, get child dirs, create dir even if exists, etc

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::prelude::*;
use chrono::{Datelike, Timelike, Utc};
use argon2::{self, Config, ThreadMode, Variant, Version};



pub fn pathStr(path: &std::path::PathBuf) -> String{
    return  path.to_str().unwrap().to_string();
}

pub fn lastPath(path: &str) -> String{
    let _path = std::path::Path::new(path);
    return _path.file_name().unwrap().to_str().unwrap().to_string();
}

pub fn addPath(_path: &str, filename: &str) -> String{
    let mut path = std::path::PathBuf::from(_path);
    path.push(filename);
    return pathStr(&path);
}

pub fn st(stringRef: &str) -> String{
    return stringRef.to_string();
}

pub fn getArgs() -> Vec<String> {
    let args_orig: Vec<String> = std::env::args().collect();                                             //https://stackoverflow.com/questions/33216514/convert-vecstring-to-vecstr
    let mut args: Vec<String> = args_orig.iter().map(|x| x.to_string()).collect();
    args.remove(0);
    return args;
}

/// a.containsAll(b)
pub fn containsAll(a: &Vec<String>, b: &Vec<String>) -> bool {
  b.iter().all(|x| a.contains(x))
}

pub fn getEpochMs() -> u64 {
  //https://stackoverflow.com/questions/26593387/how-can-i-get-the-current-time-in-milliseconds
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_millis();  // why is this u128 ????? way overkill????

    // println!("{:?}", since_the_epoch);
    // println!("{:?}", in_ms);
    in_ms as u64
}

///UPDATE ross says we dont need this - just return array of tuples
#[no_mangle]
#[inline(never)]
fn flatten6(data: &[(u32, u32)]) -> Vec<u32> {
    //https://users.rust-lang.org/t/flattening-a-vector-of-tuples/11409/2
    let mut result = data.to_vec();
    unsafe {
        result.set_len(data.len() * 2);
        std::mem::transmute(result)
    }
}

pub fn getCurrYearMonthString() -> String {
//https://rust-lang-nursery.github.io/rust-cookbook/datetime/parse.html
  let now = Utc::now();
  format!("{}-{:02}", now.year(), now.month())
}

pub fn prettyStringHashMap(map1: HashMap<String, serde_json::Value>) -> String {


  // println!("in prettyStringHashMap: map: {:?}", &map);


  let map: BTreeMap<String, serde_json::Value> = map1.into_iter().collect();
  // map.sort();    //ugh how to sort


  let asdfasdf: serde_json::Value = serde_json::to_value(map).unwrap();

  // println!("in prettyStringHashMap: asdfasdf: {:?}", &asdfasdf);


  // let asString = format!("{:?}", map);
  // let asJson: serde_json::Value = serde_json::from_str(&asString).unwrap();
  let asPrettyString = serde_json::to_string_pretty(&asdfasdf).unwrap();
  // println!("in prettyStringHashMap: asPrettyString: {:?}", &asPrettyString);

  asPrettyString
  // let jsonMaybe = serde_json::
  //how ???
  // panic!("how???")
}

pub fn hashPasswordArgon2(inputString: &str) -> String {
  // https://argon2.online/
  //https://crates.io/crates/rust-argon2

  // // // dont delete
  // // generates: "$argon2i$v=19$m=8,t=3,p=1$c29tZXNhbHQ$eYtl4kn8J9RfK5/tnJCvOs6vlxOOPxzSdZ/LgkOiLAU"
  // //            "$argon2i$v=19$m=8,t=3,p=1$c29tZXNhbHQ$eYtl4kn8J9RfK5/tnJCvOs6vlxOOPxzSdZ/LgkOiLAU" <-- from "password".as_bytes()
  // // let password = b"password";
  // // let salt = b"somesalt";
  // // let config = Config {
  // //     variant: Variant::Argon2i,
  // //     version: Version::Version13,
  // //     mem_cost: 8,
  // //     time_cost: 3,
  // //     lanes: 1,
  // //     thread_mode: ThreadMode::Parallel,
  // //     secret: &[],
  // //     ad: &[],
  // //     hash_length: 32
  // // };

  // let password = b"password";
  let password = inputString.as_bytes();

  let salt = b"pinkhimalayan";
  let config = Config {
      variant: Variant::Argon2i,
      version: Version::Version13,
      mem_cost: 8,
      time_cost: 3,
      lanes: 1,
      thread_mode: ThreadMode::Parallel,
      secret: &[],
      ad: &[],
      hash_length: 32
  };
  let hash = argon2::hash_encoded(password, salt, &config).unwrap();
  hash
}
