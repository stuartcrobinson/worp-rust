// // use hmac_sha256::Hash;
// extern crate argon2;

// // pub const D: &str  = "INDEX_DOCUMENT";

// fn main() {

//   use argon2::{self, Config, ThreadMode, Variant, Version};

// let password = b"password";
// let salt = b"somesalt";
// let config = Config {
//     variant: Variant::Argon2i,
//     version: Version::Version13,
//     mem_cost: 8,
//     time_cost: 3,
//     lanes: 1,
//     thread_mode: ThreadMode::Parallel,
//     secret: &[],
//     ad: &[],
//     hash_length: 32
// };
// let hash = argon2::hash_encoded(password, salt, &config).unwrap();
// println!("{}", hash);
// println!("{}", "$argon2i$v=19$m=8,t=3,p=1$c29tZXNhbHQ$eYtl4kn8J9RfK5/tnJCvOs6vlxOOPxzSdZ/LgkOiLAU");
// let matches = argon2::verify_encoded(&hash, password).unwrap();
// assert!(matches);

// TODO use this setting for argon2 ^


  // use argon2::{self, Config};

  // let password = b"password";
  // let salt = b"somesalt";
  // let config = Config::default();
  // let hash = argon2::hash_encoded(password, salt, &config).unwrap();
  // println!("{}", hash);
  // println!("{}", "$argon2i$v=19$m=16,t=2,p=1$c29tZXNhbHQ$w49EH5mP5bg0KwuVPsfjQg");
  // println!("{}", "$argon2i$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$r+lfEnJijsNNWDIxs8UD6Q");
  // println!("{}", "$argon2i$v=19$m=4096,t=3,p=1$c29tZXNhbHQ$iWh06vD8Fy27wf9npn6FXWiCX4K6pW6Ue1Bnzz07Z8A");
  // let matches = argon2::verify_encoded(&hash, password).unwrap();
  // assert!(matches);

  // let data = "hi";

  // let res = Hash::hash(data.as_bytes());

  // println!("{:?}", res);
  // println!("{:?}", std::str::from_utf16(&res));

  // let a = &format!("hi");
  // let b = "ho";

  // let c = "hi";

  // match c {
  //   // a => println!("it's a"),
  //   // b => println!("it's a"),
  //   D => println!("it's a"),
  //   _ => println!("asdf")
  // }
// }

// use field_types::FieldName;

// #[derive(FieldName)]
// struct Test {
//     first: i32,
//     second_field: Option<String>,
//     #[field_name(skip)]
//     third: bool,
// }

// fn main() {

//   println!("{}", TestFieldName::SecondField.name());
//   println!("{:?}", TestFieldName::f);

//   assert_eq!(TestFieldName::First.name(), "first");
//   assert_eq!(TestFieldName::SecondField.name(), "second_field");

//   assert_eq!(Some(TestFieldName::First), TestFieldName::by_name("first"));
//   assert_eq!(Some(TestFieldName::SecondField), TestFieldName::by_name("second_field"));
//   assert_eq!(None, TestFieldName::by_name("third"));

//   let fields = Test::as_field_name_array();
//   assert_eq!([TestFieldName::First, TestFieldName::SecondField], fields);

// }

// // [dependencies]
// // fs_extra = "1.2.0"

// // extern crate fs_extra;
// // use fs_extra::dir::get_size;

// // fn main(){
// //   let folder_size = get_size("dir").unwrap();
// //   println!("{}", folder_size); // print directory sile in bytes
// // }

// // // tuple map key works fine
// // use std::collections::HashMap;
// // fn main() {
// //   let mut m: HashMap<(String, String), String> = HashMap::new();
// //   m.insert((format!("k1.a"), format!("k1.b")), format!("v1"));
// //   println!("m: {:?}", m);
// //   println!("m: {:?}", m.get(&(format!("k1.a"), format!("k1.b"))));
// // }

// // // extern crate notify;

// // // use notify::{RecommendedWatcher, Watcher, RecursiveMode};
// // // use std::sync::mpsc::channel;
// // // use std::time::Duration;

// // // fn watch() -> notify::Result<()> {
// // //     // Create a channel to receive the events.
// // //     let (tx, rx) = channel();

// // //     // Automatically select the best implementation for your platform.
// // //     // You can also access each implementation directly e.g. INotifyWatcher.
// // //     let mut watcher: RecommendedWatcher = r#try!(Watcher::new(tx, Duration::from_secs(2)));

// // //     // Add a path to be watched. All files and directories at that path and
// // //     // below will be monitored for changes.
// // //     r#try!(watcher.watch("/Users/stuartrobinson/repos/worp/worp-rust/notes", RecursiveMode::NonRecursive));

// // //     // This is a simple loop, but you may want to use more complex logic here,
// // //     // for example to handle I/O.
// // //     loop {
// // //         println!("sup");
// // //         match rx.recv() {
// // //             Ok(event) => println!("{:?}", event),
// // //             Err(e) => println!("watch error: {:?}", e),
// // //         }
// // //     }
// // // }

// // // fn main() {
// // //     if let Err(e) = watch() {
// // //         println!("error: {:?}", e)
// // //     }
// // // }

// // // // extern crate rayon;
// // // // use rayon::prelude::*;
// // // // fn main() {
// // // //   (0..10).into_par_iter().for_each(|i| {
// // // //     println!("{}", i);
// // // //   });
// // // // }

// // // // // #![feature(proc_macro_hygiene)]	// Needed to use the macro in an expression
// // // // extern crate utf16_literal;

// // // // fn main() {
// // // //   let v = utf16_literal::utf16!("Foo\u{1234}😀");
// // // //   assert_eq!(v[0], 'F' as u16);
// // // //   assert_eq!(v[1], 'o' as u16);
// // // //   assert_eq!(v[2], 'o' as u16);
// // // //   assert_eq!(v[3], 0x1234);
// // // //   assert_eq!(v[4], 0xD83D);
// // // //   assert_eq!(v[5], 0xDE00);
// // // //   assert_eq!(v.len(), 6);

// // // // println!("{:?}", v);
// // // // println!("x: {:?}", utf16_literal::utf16!("x"));
// // // // println!("😀: {:?}", utf16_literal::utf16!("😀"));
// // // // println!("👨‍🦰: {:?}", utf16_literal::utf16!("👨‍🦰"));
// // // // println!("👨‍👩‍👧‍👧: {:?}", utf16_literal::utf16!("👨‍👩‍👧‍👧"));
// // // // }

// // // // // fn main() {
// // // // //   let s1 = "hi";
// // // // //   println!("string:    {}", s1);
// // // // //   println!("byte len:  {}", s1.len());
// // // // //   let mut count = 0;
// // // // //   for c in s1.chars() {
// // // // //     count += 1;
// // // // //     println!("{}", c);
// // // // //   }
// // // // //   println!("chars len:  {}", count);
// // // // //   println!();

// // // // //   let s1 = "👨‍🦰";
// // // // //   println!("string:    {}", s1);
// // // // //   println!("byte len:  {}", s1.len());
// // // // //   let mut count = 0;
// // // // //   for c in s1.chars() {
// // // // //     count += 1;
// // // // //     println!("{}", c);
// // // // //   }
// // // // //   println!("chars len:  {}", count);
// // // // //   println!();
// // // // // }

// // // // // //   /// speed test for walking along chars.  to determine if we should save endLocations or determine on the fly
// // // // // // fn main() {
// // // // // //   //s1 len 1000 chars

// // // // // //   let s1 = "123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 123456789 ";
// // // // // //   let s2 = "113355779 113355779 113355779 113355779 113355779 113355779 113355779 113355779 113355779 113355779 ";

// // // // // //   let start = std::time::Instant::now();

// // // // // //   let myC = '1';
// // // // // //   let mut matched = 0;
// // // // // //   for i in 0..100 {
// // // // // //     for c in s1.chars() {
// // // // // //       // println!("{}", c);
// // // // // //       if c.is_alphanumeric() || "dfsuhaif7&^@#%&$".contains(c) {
// // // // // //         matched += 1;
// // // // // //       }
// // // // // //     }
// // // // // //   }
// // // // // //   println!("{:?}", matched);
// // // // // //   println!("{:?}", start.elapsed());

// // // // // // }

// // // // // // // // ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////
// // // // // // // // ///////// post simple, raw string
// // // // // // // // // why tf doest this work
// // // // // // // // #![deny(warnings)]
// // // // // // // use bytes::{Bytes};

// // // // // // // use warp::Filter;

// // // // // // // use serde::Serialize;
// // // // // // // use serde::Deserialize;
// // // // // // // use serde_json::{Result, Value};

// // // // // // // #[tokio::main]
// // // // // // // async fn main() {

// // // // // // //     let jsonStr = "{\"name\":\"Sean\",\"rate\":3,\"iscool\":true,\"myfloat\":1.234}";
// // // // // // //     let v: Value = serde_json::from_str(jsonStr).unwrap();
// // // // // // //     println!("{:?}", v);
// // // // // // //     println!("{:?}", v["name"]);
// // // // // // //     println!("{:?}", v["rate"]);
// // // // // // //     println!("{:?}", v["iscool"]);
// // // // // // //     println!("{:?}", v["myfloat"]);

// // // // // // //     //prints:
// // // // // // //     // Object({"name": String("Sean"), "rate": Number(3), "iscool": Bool(true), "myfloat": Number(1.234)})
// // // // // // //     // String("Sean")
// // // // // // //     // Number(3)
// // // // // // //     // Bool(true)
// // // // // // //     // Number(1.234)

// // // // // // //     // POST /employees/:rate  {"name":"Sean","rate":2}
// // // // // // //     let promote = warp::post()
// // // // // // //         .and(warp::path("employees"))
// // // // // // //         .and(warp::path::param::<u32>())
// // // // // // //         .and(warp::body::content_length_limit(1024 * 16))
// // // // // // //         .and(warp::body::bytes())
// // // // // // //         .map(|_a, bytes: Bytes| {
// // // // // // //           // let x: String = bytes.parse().unwrap();
// // // // // // //           let x = String::from_utf8(bytes.to_vec()).unwrap();

// // // // // // //           // format!("{:?}", bytes)
// // // // // // //           format!("{}", x)
// // // // // // //         });

// // // // // // //     warp::serve(promote).run(([127, 0, 0, 1], 3030)).await
// // // // // // // }

// // // // // // // // ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////
// // // // // // // // ///////// post simple, bytes
// // // // // // // #![deny(warnings)]

// // // // // // // use warp::Filter;

// // // // // // // #[tokio::main]
// // // // // // // async fn main() {
// // // // // // //     // pretty_env_logger::init();

// // // // // // //     // POST /employees/:rate  {"name":"Sean","rate":2}
// // // // // // //     let promote = warp::post()
// // // // // // //         .and(warp::path("employees"))
// // // // // // //         .and(warp::path::param::<u32>())
// // // // // // //         // Only accept bodies smaller than 16kb...
// // // // // // //         .and(warp::body::content_length_limit(1024 * 16))
// // // // // // //         // .and(warp::body::json())
// // // // // // //         .and(warp::body::bytes())
// // // // // // //         .map(|_x, y| {
// // // // // // //           format!("hi , {:?}!",  y)
// // // // // // //         });
// // // // // // //         // .map(|rate, mut employee: Employee| {
// // // // // // //         //     employee.age = rate;
// // // // // // //         //     println!("{:?}", employee);
// // // // // // //         //     warp::reply::json(&employee)
// // // // // // //         // });

// // // // // // //     warp::serve(promote).run(([127, 0, 0, 1], 3030)).await
// // // // // // // }

// // // // // // // // ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////
// // // // // // // // ///////// post
// // // // // // // #![deny(warnings)]

// // // // // // // use serde_derive::{Deserialize, Serialize};

// // // // // // // use warp::Filter;

// // // // // // // #[derive(Deserialize, Serialize, Debug)]
// // // // // // // struct Employee {
// // // // // // //     name: String,
// // // // // // //     age: u32,
// // // // // // // }

// // // // // // // #[tokio::main]
// // // // // // // async fn main() {
// // // // // // //     // pretty_env_logger::init();

// // // // // // //     // POST /employees/:rate  {"name":"Sean","rate":2}
// // // // // // //     let promote = warp::post()
// // // // // // //         .and(warp::path("employees"))
// // // // // // //         .and(warp::path::param::<u32>())
// // // // // // //         // Only accept bodies smaller than 16kb...
// // // // // // //         .and(warp::body::content_length_limit(1024 * 16))
// // // // // // //         .and(warp::body::json())
// // // // // // //         .map(|rate, mut employee: Employee| {
// // // // // // //             employee.age = rate;
// // // // // // //             println!("{:?}", employee);
// // // // // // //             warp::reply::json(&employee)
// // // // // // //         });

// // // // // // //     warp::serve(promote).run(([127, 0, 0, 1], 3030)).await
// // // // // // // }

// // // // // // // ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////
// // // // // // // /////// simple

// // // // // // // use warp::Filter;

// // // // // // // fn sayHello(input: String) -> String {
// // // // // // //   format!("Hello, {}!", input)
// // // // // // // }

// // // // // // // #[tokio::main]
// // // // // // // async fn main() {
// // // // // // //     // GET /hello/warp => 200 OK with body "Hello, warp!"
// // // // // // //     let hello = warp::path!("hello" / String)
// // // // // // //         .map(|name| sayHello(name));

// // // // // // //     let hi = warp::path!("hi" / String)
// // // // // // //         .map(|name| format!("hi , {}!", name));

// // // // // // //     let routes = hello.or(hi);

// // // // // // //     warp::serve(routes)
// // // // // // //         .run(([127, 0, 0, 1], 3030))
// // // // // // //         .await;
// // // // // // // }
// // // // // // // ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////// ///////

// // // // // // // #![allow(non_snake_case)]

// // // // // // // use serde_json::Value;

// // // // // // // fn main() {
// // // // // // //     let s = r#"{"x": 1.0, "y": 2.0, "z": [{"1":"one"}, {"2":"two"}]}"#;
// // // // // // //     let mut value: Value = serde_json::from_str(s).unwrap();

// // // // // // //     println!("value: {:?}", value);

// // // // // // //     // Check value using read-only pointer
// // // // // // //     assert_eq!(value.pointer("/x"), Some(&1.0.into()));
// // // // // // //     // Change value with direct assignment
// // // // // // //     *value.pointer_mut("/x").unwrap() = 1.5.into();

// // // // // // //     println!("value: {:?}", value);

// // // // // // //     // Change value with direct assignment
// // // // // // //     *value.pointer_mut("/z").unwrap() = 1.5.into();

// // // // // // //     println!("value: {:?}", value);

// // // // // // //     // Check that new value was written
// // // // // // //     assert_eq!(value.pointer("/x"), Some(&1.5.into()));
// // // // // // //     // Or change the value only if it exists
// // // // // // //     value.pointer_mut("/x").map(|v| *v = 1.5.into());

// // // // // // //     println!("value: {:?}", value);

// // // // // // //     // "Steal" ownership of a value. Can replace with any valid Value.
// // // // // // //     let old_x = value.pointer_mut("/x").map(Value::take).unwrap();
// // // // // // //     assert_eq!(old_x, 1.5);
// // // // // // //     assert_eq!(value.pointer("/x").unwrap(), &Value::Null);

// // // // // // //     println!("value: {:?}", value);
// // // // // // // }

// // // // // // // // extern crate comma;

// // // // // // // // use shellwords::split;

// // // // // // // // fn main () {
// // // // // // // //     // let parsed = Command::from_str("sendmsg joe \"I say \\"hi\\" to you!\"");
// // // // // // // //     // println!("Command name: {}", parsed.name); // Command name: sendmsg
// // // // // // // //     // println!("Command arguments: {:#?}", parsed.arguments); // Command arguments: [ "joe", "I say \"hi\" to you!" ]

// // // // // // // //     // assert_eq!(split("here are \"two words\"").unwrap(), ["here", "are", "two words"]);

// // // // // // // //     // println!("{:?}", split("here are \"two words\" cool! huh? 'yeah' \"neat").unwrap());

// // // // // // // //     let mut query = "here are \"two words\" cool! huh? 'yeah' neat \"just incredible\"".to_string();
// // // // // // // //     let theSplit = splitIntoPhrases(&query);
// // // // // // // //     println!("{:?}", theSplit);
// // // // // // // // }

// // // // // // // // pub fn splitIntoPhrases(s: &str) -> Vec<String> {

// // // // // // // //   let mut query = s.replace("\"", " \" ");
// // // // // // // //   // println!("{:?}", query);

// // // // // // // //   let numDubquotes = query.matches("\"").count();
// // // // // // // //   // println!("numDubquotes: {:?}", numDubquotes);
// // // // // // // //   if numDubquotes % 2 == 1 {
// // // // // // // //     let dubQuoteIndex = query.rfind("\"").unwrap();
// // // // // // // //     query.replace_range(dubQuoteIndex..=dubQuoteIndex, "");
// // // // // // // //   }
// // // // // // // //   let mut splitIntoPhrases = shellwords::split(&query).unwrap();
// // // // // // // //   for i in 0..splitIntoPhrases.len() {
// // // // // // // //     // splitIntoPhrases[i] = splitIntoPhrases[i].to_string().trim().to_string();
// // // // // // // //     splitIntoPhrases[i] = splitIntoPhrases[i].trim().to_string();
// // // // // // // //   }
// // // // // // // //   splitIntoPhrases
// // // // // // // // }

// // // // // // // use uuid::Uuid; //println!("{}", Uuid::new_v4());

// // // // // // // fn main() {
// // // // // // //     // let my_uuid =  Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
// // // // // // //     // println!("{}", my_uuid.to_urn());
// // // // // // //     // println!("{}", my_uuid);
// // // // // // //     // println!("{}", Uuid::new_v1());
// // // // // // //     // println!("{}", Uuid::new_v2());
// // // // // // //     // println!("{}", Uuid::new_v3());
// // // // // // //     println!("{}", Uuid::new_v4());
// // // // // // //     // println!("{}", Uuid::new_v5());
// // // // // // // }

// // // // // // // // // extern crate uuid;
// // // // // // // // // use uuid::Uuid;

// // // // // // // // // fn main() {
// // // // // // // // //     let my_uuid = Uuid::new_v1();
// // // // // // // // //     println!("{}", my_uuid);
// // // // // // // // // }

// // // // // // // // // extern crate unidecode;
// // // // // // // // // use unidecode::unidecode;

// // // // // // // // // extern crate snips_nlu_utils;
// // // // // // // // // use snips_nlu_utils::string::normalize;

// // // // // // // // // fn main() {
// // // // // // // // //   // let str_of_emojis = "😘🤗🦀 hello ❤️❤️♥️ 👨‍🦰 👨‍👨‍👧‍👧";

// // // // // // // // //   // for emoji in str_of_emojis.chars() {
// // // // // // // // //   //     println!("{}", emoji);
// // // // // // // // //   // }

// // // // // // // // //   // println!("{}", str_of_emojis.escape_unicode());

// // // // // // // // //   println!("[{}]", unidecode("ß   ss"));
// // // // // // // // //   println!("[{}]", unidecode("ł   l"));
// // // // // // // // //   println!("[{}]", unidecode("đ   d"));
// // // // // // // // //   println!("[{}]", unidecode("æ   ae"));
// // // // // // // // //   println!("[{}]", unidecode("œ   oe"));
// // // // // // // // //   println!("[{}]", unidecode("ﬆ   st"));
// // // // // // // // //   println!("[{}]", unidecode("ﬀ	  ff"));
// // // // // // // // //   println!("[{}]", unidecode("ﬃ   ffi"));
// // // // // // // // //   println!("[{}]", unidecode("ﬄ   ffl"));
// // // // // // // // //   println!("[{}]", unidecode("ﬁ   fi"));
// // // // // // // // //   println!("[{}]", unidecode("ﬂ   fl"));
// // // // // // // // //   println!("[{}]", unidecode("ﬅ   st"));
// // // // // // // // //   println!("[{}]", unidecode("𝓬𝓸𝓹𝔂 𝒶𝓃𝒹 𝓹𝓪𝓼𝓽𝓮 ..."));
// // // // // // // // //   println!("[{}]", unidecode("Crème brûlée"));
// // // // // // // // //   println!("[{}]", unidecode("Đapps"));
// // // // // // // // //   println!("[{}]", unidecode("Maß"));
// // // // // // // // //   println!("[{}]", unidecode("Sųłiné"));
// // // // // // // // //   println!("[{}]", unidecode("Ł - Wikipedia"));
// // // // // // // // //   println!("[{}]", unidecode("Æneid"));
// // // // // // // // //   println!("[{}]", unidecode("étude"));
// // // // // // // // //   println!("[{}]", unidecode("北亰"));
// // // // // // // // //   println!("[{}]", unidecode("ᔕᓇᓇ"));
// // // // // // // // //   println!("[{}]", unidecode("げんまい茶"));
// // // // // // // // //   println!("[{}]", unidecode("😘🤗🦀 hello ❤️❤️♥️ 👨‍🦰 👨‍👨‍👧‍👧"));

// // // // // // // // // println!("--------------------------------");

// // // // // // // // // println!("[{}]", normalize("ß   s"));
// // // // // // // // // println!("[{}]", normalize("ł   l"));
// // // // // // // // // println!("[{}]", normalize("đ   d"));
// // // // // // // // // println!("[{}]", normalize("æ   ae"));
// // // // // // // // // println!("[{}]", normalize("œ   oe"));
// // // // // // // // // println!("[{}]", normalize("ﬆ   st"));
// // // // // // // // // println!("[{}]", normalize("ﬀ	  ff"));
// // // // // // // // // println!("[{}]", normalize("ﬃ   ffi"));
// // // // // // // // // println!("[{}]", normalize("ﬄ   ffl"));
// // // // // // // // // println!("[{}]", normalize("ﬁ   fi"));
// // // // // // // // // println!("[{}]", normalize("ﬂ   fl"));
// // // // // // // // // println!("[{}]", normalize("ﬅ   st"));
// // // // // // // // // println!("[{}]", normalize("𝓬𝓸𝓹𝔂 𝒶𝓃𝒹 𝓹𝓪𝓼𝓽𝓮 ..."));
// // // // // // // // // println!("[{}]", normalize("Crème brûlée"));
// // // // // // // // // println!("[{}]", normalize("Đapps"));
// // // // // // // // // println!("[{}]", normalize("Maß"));
// // // // // // // // // println!("[{}]", normalize("Sųłiné"));
// // // // // // // // // println!("[{}]", normalize("Ł - Wikipedia"));
// // // // // // // // // println!("[{}]", normalize("Æneid"));
// // // // // // // // // println!("[{}]", normalize("étude"));
// // // // // // // // // println!("[{}]", normalize("北亰"));
// // // // // // // // // println!("[{}]", normalize("ᔕᓇᓇ"));
// // // // // // // // // println!("[{}]", normalize("げんまい茶"));
// // // // // // // // // println!("[{}]", normalize("😘🤗🦀 hello ❤️❤️♥️ 👨‍🦰 👨‍👨‍👧‍👧"));

// // // // // // // // // }

// // // // // // // // // // use std::process::Command;

// // // // // // // // // // fn main(){

// // // // // // // // // //   let source = shardSource;
// // // // // // // // // //   let destination = shardRoot;
// // // // // // // // // //   Command::new("rsync").arg(source)
// // // // // // // // // //                        .arg(destination)
// // // // // // // // // //                        .spawn()
// // // // // // // // // //                        .expect("ls command failed to start");

// // // // // // // // // //   println!("done")
// // // // // // // // // // }

// // // // // // // // // // // fn main(){
// // // // // // // // // // //   Command::new("ls")
// // // // // // // // // // //                     .arg("-l")
// // // // // // // // // // //                     .arg("-a")
// // // // // // // // // // //                     .spawn()
// // // // // // // // // // //                     .expect("ls command failed to start");
// // // // // // // // // // //   println!("done")
// // // // // // // // // // // }
