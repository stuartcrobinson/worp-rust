#![allow(non_snake_case)]
#![allow(while_true)]
// https://stackoverflow.com/questions/32257273/split-a-string-keeping-the-separators
// https://stackoverflow.com/questions/56921637/how-do-i-split-a-string-using-a-rust-regex-and-keep-the-delimiters?rq=1 doesnt work
// use rocksdb::{prelude::*, IteratorMode};
use std::collections::HashMap;
use std::collections::HashSet;
// use crate::getDocId;
// extern crate unidecode;
extern crate uuid;
use crate::global::*;
// use crate::rocksdb_tools::*;
use crate::h::st;
extern crate snips_nlu_utils;
use snips_nlu_utils::string::normalize;
// use std::iter::FromIterator;

use crate::stateManagement::SPLIT_KEEP_CHARS;
use num_format::{Locale, ToFormattedString};


extern crate rand;

use rand::Rng;


/// note: docIds and shardNumbers start at 1 !!!!!!!!!!!!!!!  there is no docId == 0
#[derive(Debug)]
pub struct DocTok {
    pub rawTok: String,
    pub cookedTok: String,
    pub positions: Vec<u16>,
    pub locations: Vec<u16>,
}

/// note: docIds and shardNumbers start at 1 !!!!!!!!!!!!!!!  there is no docId == 0
#[derive(Debug)]
pub struct DocTok_noRaw {
    pub cookedTok: String,
    pub positions: Vec<u16>,
    pub locations: Vec<u16>,
}

/// ß   ss
/// ł   l
/// đ   d
/// æ   ae
/// œ   oe
/// ﬆ   st
/// ﬀ	  ff
/// ﬃ   ffi
/// ﬄ   ffl
/// ﬁ   fi
/// ﬂ   fl
/// ﬅ   st
///
/// 𝓬𝓸𝓹𝔂 𝒶𝓃𝒹 𝓹𝓪𝓼𝓽𝓮 ...      //algolia doesn't do these.
/// https://www.reddit.com/r/rust/comments/39jcz8/how_can_i_check_if_a_string_contains_specific/
/// https://play.rust-lang.org/?code=use%20std%3A%3Aerror%3A%3AError%3B%0Ause%20std%3A%3Afs%3A%3AFile%3B%0Ause%20std%3A%3Aio%3A%3ABufReader%3B%0Ause%20std%3A%3Aio%3A%3Aprelude%3A%3A*%3B%0Ause%20std%3A%3Apath%3A%3APath%3B%0A%0Afn%20is_aeiou(x%3A%20%26char)%20-%3E%20bool%20%7B%0A%20%20%20%20%22aeiou%22.chars().any(%7Cy%7C%20y%20%3D%3D%20*x)%0A%7D%0A%0Afn%20is_weird_auo(x%3A%20%26char)%20-%3E%20bool%20%7B%0A%20%20%20%20%22%C3%A4%C3%BC%C3%B6%22.chars().any(%7Cy%7C%20y%20%3D%3D%20*x)%0A%7D%0A%0Afn%20valid(line%3A%20%26str)%20-%3E%20bool%20%7B%0A%20%20%20%20line.chars().any(%7Cc%7C%20is_aeiou(%26c))%20%26%26%0A%20%20%20%20line.chars().filter(is_weird_auo).fuse().nth(1).is_some()%0A%7D%0A%0Afn%20main()%20%7B%0A%0A%20%20%20%20%2F%2F%20Create%20a%20path%20to%20the%20desired%20file%0A%20%20%20%20let%20path%20%3D%20Path%3A%3Anew(%22foo.txt%22)%3B%0A%20%20%20%20let%20display%20%3D%20path.display()%3B%0A%20%20%20%20%0A%20%20%20%20%2F%2F%20Open%20the%20path%20in%20read-only%20mode%2C%20returns%20%60io%3A%3AResult%3CFile%3E%60%0A%20%20%20%20let%20file%20%3D%20match%20File%3A%3Aopen(%26path)%20%7B%0A%20%20%20%20%20%20%20%20%2F%2F%20The%20%60description%60%20method%20of%20%60io%3A%3AError%60%20returns%20a%20string%20that%20describes%20the%20error%0A%20%20%20%20%20%20%20%20Err(why)%20%3D%3E%20panic!(%22couldn%27t%20open%20%7B%7D%3A%20%7B%7D%22%2C%20display%2C%20Error%3A%3Adescription(%26why))%2C%0A%20%20%20%20%20%20%20%20Ok(file)%20%3D%3E%20file%2C%0A%20%20%20%20%7D%3B%0A%20%20%20%20%0A%20%20%20%20let%20reader%20%3D%20BufReader%3A%3Anew(file)%3B%0A%20%20%20%20let%20lines%20%3D%20reader.lines()%3B%0A%20%20%20%20%0A%20%20%20%20let%20bad_line%20%3D%20lines.map(%7Cl%7C%20l.unwrap()).filter(%7Cline%7C%20!valid(line)).next()%3B%0A%20%20%20%20match%20bad_line%20%7B%0A%20%20%20%20%20%20%20%20Some(line_n)%20%3D%3E%20println!(%22Line%20%7B%7D%20doesn%27t%20pass%20the%20test%22%2C%20line_n)%2C%0A%20%20%20%20%20%20%20%20None%20%3D%3E%20println!(%22All%20lines%20are%20good!%22)%2C%0A%20%20%20%20%7D%0A%20%20%20%20%0A%20%20%20%20%2F%2F%20Alternate%20way%20if%20you%20don%27t%20need%20the%20line%20number.%20More%20readable%0A%20%20%20%20%2F%2Flet%20all_good%20%3D%20lines.map(%7Cl%7C%20l.unwrap()).all(valid)%3B%0A%7D&version=stable
fn is_ligand(x: &char) -> bool {
  "ßłđæœﬆﬀﬃﬄﬁﬂﬅ".chars().any(|y| y == *x)
}

fn contains_ligand(tok: &str) -> bool {
  tok.chars().any(|c| is_ligand(&c))
}

/// TODO deal with fancy text https://github.com/stuartcrobinson/ZeppelinBot/blob/master/backend/src/utils/normalizeText.ts
///
/// lots of great crates here for better parsing like dealing w/ chinese chars https://lib.rs/text-processing
pub fn normalizeToken(rawTok: &str) -> String {
  let mut cookedTok = rawTok.replace("\'", "").to_lowercase();

  // println!("1 in normalizeToken. raw, cooked: {}  {}", rawTok, cookedTok);
  cookedTok = normalize(&cookedTok);

  // println!("2 in normalizeToken. raw, cooked: {}  {}", rawTok, cookedTok);
  if contains_ligand(&cookedTok){                     //this seems like it could save a little time but idk

    cookedTok = cookedTok.replace("ß",   "ss");
    cookedTok = cookedTok.replace("ł",   "l");
    cookedTok = cookedTok.replace("đ",   "d");
    cookedTok = cookedTok.replace("æ",   "ae");
    cookedTok = cookedTok.replace("œ",   "oe");
    cookedTok = cookedTok.replace("ﬆ",   "st");
    cookedTok = cookedTok.replace("ﬀ",	  "ff");
    cookedTok = cookedTok.replace("ﬃ",   "ffi");
    cookedTok = cookedTok.replace("ﬄ",   "ffl");
    cookedTok = cookedTok.replace("ﬁ",   "fi");
    cookedTok = cookedTok.replace("ﬂ",   "fl");
    cookedTok = cookedTok.replace("ﬅ",   "st");
  }

  // println!("3 in normalizeToken. raw, cooked: {}  {}", rawTok, cookedTok);
  cookedTok
}

/// to_lowercase happens here
fn get__m_rawTok_docTok(rawToks: &Vec<String>) -> (HashMap<String, DocTok>, Vec<String>) {
    let mut m_rawTok_docTok = HashMap::new();
    let mut cookedToks_ordered: Vec<String> = Vec::new();
    for rawTok in rawToks{
      let cookedTok = normalizeToken(rawTok);
      m_rawTok_docTok.insert(
        rawTok.to_string(),
          DocTok {rawTok: rawTok.to_string(), cookedTok: cookedTok.to_string(), positions: Vec::new(), locations: Vec::new()}
      );
      cookedToks_ordered.push(cookedTok);
    }
    (m_rawTok_docTok, cookedToks_ordered)
}

fn insertLocations_new(mut m_rawTok_docTok: HashMap<String, DocTok>, m_rawTok_locations: HashMap<String, Vec<u16>>) -> HashMap<String, DocTok>{

  let mut rawToksSet = Vec::new();
  for (rawTok, _) in &m_rawTok_docTok {
    rawToksSet.push(rawTok.to_string());
  }

  for rawTok in rawToksSet {
    let locations = m_rawTok_locations.get(&rawTok).unwrap().to_vec();

    let docTok = m_rawTok_docTok.get(&rawTok).unwrap();

    let positions = docTok.positions.clone();
    let cookedTok = docTok.cookedTok.clone();
    // println!("insertLocations: {}       {}       {:?}       {}", rawTok, cookedTok, locations, &doc[0..std::cmp::min(doc.len(), 20)]);
    m_rawTok_docTok.insert(
      rawTok.to_string(),
      DocTok {rawTok: rawTok, cookedTok: cookedTok, positions: positions, locations: locations}
    );
  }

  return m_rawTok_docTok;
}

#[allow(mutable_borrow_reservation_conflict)]
fn insertPositions(mut m_rawTok_docTok: HashMap<String, DocTok>, rawToks: &Vec<String>) -> HashMap<String, DocTok>{
    for i in 0..rawToks.len() {
        let rawTok = &rawToks[i];
        let docTok = m_rawTok_docTok.get(rawTok).unwrap();
        let position = i;
        let positions = &docTok.positions;
        let mut positionsCopy = positions.clone();
        positionsCopy.push(position as u16);
        // TODO how to fix this warning below???
        m_rawTok_docTok.insert(rawTok.to_string(), DocTok {rawTok: docTok.rawTok.to_string(), cookedTok: docTok.cookedTok.to_string(), positions: positionsCopy, locations: Vec::new()});
    }

    return m_rawTok_docTok;
}

fn make_sumPriorLengths(properlySpacedRawToks: &Vec<String>) -> Vec<usize> {
  let mut sumPriorLengths:  Vec<usize> = vec![0; properlySpacedRawToks.len()];

  for i in 0..properlySpacedRawToks.len() {
    let priorLens = if i == 0 { 0 } else { sumPriorLengths[i - 1]};
    let prevLen = if i == 0 { 0 } else { properlySpacedRawToks[i - 1].len()};
    sumPriorLengths[i] = priorLens + prevLen;
  }
  sumPriorLengths
}

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

/** wtf is relevance ... it's like how common it is in the doc sept2020 */
pub fn getRelevance(numTotalToks: usize, count: usize) -> u16{

    if count > numTotalToks { panic!(" count > numTotalToks");}

    let rel_float = ((count as f32) / (numTotalToks as f32)) * (std::u16::MAX as f32 - 1.0);

    return rel_float as u16;
}

/// this is stupid slow.  TODO optimize ??????????????? --- wait ... but is it? or is slowness coming from disc reads ...
///
/// if a phrase word len is > 3, split into phrases len 3
///
/// in the future this will probably need the schema.  for customizable tokenization
pub fn tokenizeQuery(query: &str) -> (Vec<String>, Vec<String>, Vec<String>) {

  let (rawQueryPhrases, excludedRawToks) = parsePhrasesAndExcluders(query);    //a "phrase" here could be just 1 word right?

  println!("in tokenizeQuery, queryPhrases: {:?} ", rawQueryPhrases);
  println!("in tokenizeQuery, excludedRawToks: {:?} ", excludedRawToks);

  let mut queryOrderedPhrasesCooked: Vec<String> = Vec::new();

  for i in 0..rawQueryPhrases.len() {
    let rawQueryPhrase = &rawQueryPhrases[i];
    let  (m__toksType__num_mCookedtokLocations, cookedToks_ordered) = tokenize(rawQueryPhrase, vec![1], false);
    println!("in tokenizeQuery, cookedToks_ordered: {:?} ", cookedToks_ordered);
    // let queryPhraseTokens: Vec<String> = m__toksType__num_mCookedtokLocations.1;
    let cookedQueryPhrase = cookedToks_ordered.join(" ");                                               // this makes an empty string if empty vec input!  ...obv
    println!("in tokenizeQuery, cookedQueryPhrase: {:?} ", cookedQueryPhrase);
    if cookedQueryPhrase.len() > 0 {
      queryOrderedPhrasesCooked.push(cookedQueryPhrase);
    }
  }
  println!("in tokenizeQuery, queryOrderedPhrasesCooked: {:?} ", queryOrderedPhrasesCooked);

  let queryPhrasesMaxWordLen3 = splitUpLongQueryPhrases(&queryOrderedPhrasesCooked);

  let queryOrderedPhrasesCookedOriginal = queryOrderedPhrasesCooked;

  //NOTE - we need a way of keeping track of whether a phrase is a subphrase or original phrase. we also need to remember what the original phrase was, like if it was long....
  //          how ?

  let mut excludedCookedToks: Vec<String> = Vec::new();

  for i in 0..excludedRawToks.len() {
    let excludedRawTok = &excludedRawToks[i];
    let m__toksType__num_mCookedtokLocations = tokenize(excludedRawTok, vec![1], false);
    let excludedTokens: Vec<String> = m__toksType__num_mCookedtokLocations.1;
    let excludedToken = excludedTokens.join(" ");                               //there should be only 1
    excludedCookedToks.push(excludedToken);
  }
  println!("in tokenizeQuery, queryPhrasesMaxWordLen3: {:?} ", queryPhrasesMaxWordLen3);
  println!("in tokenizeQuery, excludedCookedToks: {:?} ", excludedCookedToks);
  println!("in tokenizeQuery, queryOrderedPhrasesCookedOriginal: {:?} ", queryOrderedPhrasesCookedOriginal);
  (queryPhrasesMaxWordLen3, excludedCookedToks, queryOrderedPhrasesCookedOriginal)
}



#[allow(dead_code)]
fn generateNumbers(numDocs: u32) -> Vec<String>{

    let mut lines:Vec<String> = Vec::new();

    for i in 0..numDocs{
        lines.push(i.to_formatted_string(&Locale::en));
        if i % 10000 == 0{
            println!("generating lipsum list, on line {}", i);
        }
    }

    return lines;
}

#[allow(dead_code)]
fn getNx100kemails(n: usize) -> Vec<String>{
    let mut docs_master: Vec<String> = Vec::new();
    for i in 0..n{
        let path = format!("resources/100,000 USA Email List/100,000 USA Email Address copy {}.TXT", i);
        let mut lines = lines_from_file(path).expect("Could not load lines");
        docs_master.append(&mut lines);
    }
    return docs_master;
}

#[allow(dead_code)]
fn generateDocs_numStrings(initDocId: usize, numDocs: usize, maxNumSize: usize, wordsPerDoc: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut docs: Vec<String> = Vec::new();

    for i in 0..numDocs {
        let mut tempNumbersVec: Vec<usize> = Vec::new();
        for _k in 0..(wordsPerDoc/3) {
            tempNumbersVec.push(rng.gen_range(0, 50));
            tempNumbersVec.push(rng.gen_range(0, 2000));
            tempNumbersVec.push(rng.gen_range(0, maxNumSize));
        }
        let globalDocId = initDocId + i;
        let doc = format!("doc {} {}  {:?} {}  {:?}", i.to_formatted_string(&Locale::en), i, globalDocId, globalDocId.to_formatted_string(&Locale::en), tempNumbersVec);
        // println!("generated doc: {}",doc );
        docs.push(doc);
    }

    return docs;
}


#[inline(always)]
pub fn getBytesFromDocIdAndRelevance(docId: usize, relevance: u16) -> [u8; 5]{
  let docId_u32 = docId as u32;
  let docId_bytes = docId_u32.to_be_bytes();
  let relav_bytes = relevance.to_be_bytes();
  let slice = [docId_bytes[3], docId_bytes[2], docId_bytes[1], relav_bytes[1], relav_bytes[0]];
  return slice;
}

/// have to account for byte distance of removed chars like smart quotes
///
/// this is very sloppy and stupid
///
/// return it as a map cos later we might want phrases stored in separate rocksdbs per word count
pub fn tokenize(fieldValue: &str, phraseTokWordLengths: Vec<usize>, doIndexPrefixes: bool)-> (HashMap<String, (usize,  HashMap<String, HashSet<u16>>)>, Vec<String>) {
  // println!("in tokenize, fieldValue: {:?}", fieldValue);

  /* what is toksType?  -->  either "PREFIX" or a number corresponding to the length of the token=phrase.  */

  let mut m__toksType__num_mCookedtokLocations: HashMap<String, (usize,  HashMap<String, HashSet<u16>>)> = HashMap::new();
  let (rawToksOrdered, m_rawTok_locations, rawTokLocations) = get_rawToks_vec_and_locations_map(fieldValue, SPLIT_KEEP_CHARS);

  let (m_rawTok_docTok, cookedToks_ordered) = get__m_rawTok_docTok(&rawToksOrdered);                                       // why are all these maps keyed by rawTok instead of cookedTok?
  let m_rawTok_docTok = insertPositions(m_rawTok_docTok, &rawToksOrdered);
  let m_rawTok_docTok = insertLocations_new(m_rawTok_docTok, m_rawTok_locations);
  let mut m_cookedTok_locations: HashMap<String, HashSet<u16>> = HashMap::new();
  for (_rawTok, docTok) in &m_rawTok_docTok {
    let cookedTok = docTok.cookedTok.to_string();

    m_cookedTok_locations.entry(cookedTok)
                          .or_insert_with(HashSet::new)
                          .extend(&docTok.locations);
  }
  m__toksType__num_mCookedtokLocations.insert(st("1"), (rawToksOrdered.len(), m_cookedTok_locations ));

  // println!("rawToksOrdered: {:?}", rawToksOrdered);
  // println!("sumPriorLengths: {:?}", rawTokLocations);
  assert!(rawToksOrdered.len() == rawTokLocations.len());
  assert!(rawTokLocations.len() == cookedToks_ordered.len());          // sanity checks

  // WHAT IS HAPPENING HERE ?????? this is a nightmare

  if phraseTokWordLengths.contains(&2){
    let mut m_phrase_locations: HashMap<String, HashSet<u16>> = HashMap::new();
    for i in 1..cookedToks_ordered.len() {
      let phrase: String = format!("{} {}", cookedToks_ordered[i - 1], cookedToks_ordered[i]);
      let location = rawTokLocations[i - 1];
      m_phrase_locations.entry(phrase)
                        .or_insert_with(HashSet::new)
                        .insert(location as u16);
    }
    m__toksType__num_mCookedtokLocations.insert(st("2"), (std::cmp::max((rawToksOrdered.len() as i32 - 1) as usize, 0), m_phrase_locations ));
  }
  if phraseTokWordLengths.contains(&3){
    let mut m_phrase_locations: HashMap<String, HashSet<u16>> = HashMap::new();
    for i in 2..cookedToks_ordered.len() {
      let phrase: String = format!("{} {} {}", cookedToks_ordered[i-2], cookedToks_ordered[i-1], cookedToks_ordered[i]);
      let location = rawTokLocations[i - 2];
      m_phrase_locations.entry(phrase)
                        .or_insert_with(HashSet::new)
                        .insert(location as u16);
    }
    m__toksType__num_mCookedtokLocations.insert(st("3"), (std::cmp::max((rawToksOrdered.len() as i32 - 2) as usize, 0), m_phrase_locations ));
  }

  if doIndexPrefixes {
    m__toksType__num_mCookedtokLocations.insert(
      "PREFIX".to_string(),
      (
        m__toksType__num_mCookedtokLocations.get("1").unwrap().0,
        convertToPrefixes(&m__toksType__num_mCookedtokLocations.get("1").unwrap().1)
      )
    );
  }
  (m__toksType__num_mCookedtokLocations, cookedToks_ordered)
}

fn split_non_internal_apostrophes(rawToks: Vec<&str>) -> Vec<String> {
  let patterns : &[_] = &['\'', '’', '‘'];
  let apostrophes = "'’‘";
  let mut rawTokStrings: Vec<String> = Vec::new();
  for _tok in rawToks {
    let mut tok = _tok.to_string();
    if tok.len() == 0 {                                                                                            // why ?
      continue;
    }
    if tok == "'" || tok == "’" || tok == "‘" {                                                                   // why ?
      rawTokStrings.push(tok.to_string());
      continue;
    }
    // remove leading apostrophe
    let firstChar = tok.chars().nth(0).unwrap();
    if apostrophes.contains(firstChar){
      rawTokStrings.push(firstChar.to_string());                                                         //gotta keep track of spacing for location calculations
      tok = tok.trim_start_matches(patterns).to_string();
    }
    // remove trailing apostrophe
    let mut trailingApostropheWasRemoved = false;
    let lastChar = tok.chars().last().unwrap();
    if apostrophes.contains(lastChar) {
      trailingApostropheWasRemoved = true;                                                        //gotta keep track of spacing for location calculations
      tok = tok.trim_end_matches(patterns).to_string();
    }
    rawTokStrings.push(tok);
    if trailingApostropheWasRemoved {
      rawTokStrings.push(lastChar.to_string());
    }
  }
  return rawTokStrings;
}


///     let doc = "won't you #deleteFacebook 'no'? okay! fine c++ 123.345.456 $99.00 44$55";
///     ["won\'t", "you", "#", "deleteFacebook", "\'no\'", "okay", "fine", "c", "+", "+", "123", "345", "456", "$", "99", "00", "44", "$", "55"]
///     TODO: only keep apost if surrounded by alpha   ----         (sept 2020 - wtf is alpha???)
///
/// UPDATE: get locations WHILE parsing for raw toks.  how?
/// 1.  split the string by EVERYTHING we'll want to split on.  to delete or keep. (so, all punctuation and symbols (like $) except apostrophes)
/// 2.  re-split any items with trailing apostrophes (smart or dumb)
/// 3.  calculate locations array (sumPriorLens).  this is BYTE lengths
/// 4.  now delete any elements containing chars we dont want.  like '(' etc.
///
/// so we should have an input that's just "splitKeepChars"
fn get_rawToks_vec_and_locations_map<'a>(fieldValue: &'a str, splitKeepChars: &str) -> (Vec<String>, HashMap<String, Vec<u16>>, Vec<u16>) {

    let split1 = splitKeep_by_everything_except_apostrophes(fieldValue);

    let split2 = split_non_internal_apostrophes(split1);

    let sumPriorLengths = make_sumPriorLengths(&split2);    // this is not sumPriorLenghts for cookedToks!!!! it's for intermediary split stuff

    // now we can wipe all the unneeded characters.  just replace those elements with an empty string

    let splitCleaned = delete_all_nonWord_chars_except_splitKeepChars(split2, splitKeepChars);

    let (m_rawTok_locations, rawToksOrdered, rawTokLocations) = build__m_rawTok_locations__worker(&sumPriorLengths, &splitCleaned);

    (rawToksOrdered, m_rawTok_locations, rawTokLocations)
}

/// is a space or "punctuation"
///
/// scalar = unicode scalar = rust char
///
/// so a "word scalar" is anything that's part of a description of like... meaning. like '👍' or 'q' or '漢' or 'ي '  ... but not "punctuation" or, ideally, pronunciation guides (like in arabic?)
///
/// https://www.algolia.com/doc/guides/managing-results/optimize-search-results/handling-natural-languages-nlp/in-depth/normalization/
pub fn not_a_word_scalar(c: char) -> bool {
  let otherPunctuation = "。「」“”’‘—";    //https://github.com/rust-lang/rfcs/issues/1655
  c.is_whitespace() || c.is_ascii_punctuation() ||  otherPunctuation.contains(c)
}

/// split by emojis too.  by using char.is_alphabetic (emojis are not.  chinese chars are tho.  deal w/ splitting chinese chars later)
pub fn splitKeep_by_everything_except_apostrophes<'a>(text: &'a str) -> Vec<&'a str>{
  let apostrophes = "'’‘";

  //no idea how this works
  let mut result = Vec::new();
  let mut last = 0;
  // for (index, matched) in text.match_indices(|c: char| (not_a_word_scalar(c) || !c.is_alphabetic() ) && !apostrophes.contains(c)) { //is_alphabetic is very broad. splits emojis
  for (index, matched) in text.match_indices(|c: char| (not_a_word_scalar(c) || !c.is_alphanumeric() ) && !apostrophes.contains(c)) { //is_alphabetic is very broad. splits emojis
      // println!("last: {},  index: {},  matched: {},  ", last, index, matched);
      if last != index {
          result.push(&text[last..index]);
      }
      result.push(matched);
      last = index + matched.len();
  }
  if last < text.len() {
      result.push(&text[last..]);
  }

  // println!("{:?}", result);
  return result;

}

fn build__m_rawTok_locations__worker(sumPriorLengths: &Vec<usize>, splitCleaned: &Vec<String>)   -> (HashMap<String, Vec<u16>>, Vec<String>, Vec<u16>)   {
  let mut m_rawTok_locations: HashMap<String, Vec<u16>> = HashMap::new();

  let mut rawToksOrdered: Vec<String> = Vec::new();
  let mut rawTokLocations: Vec<u16> = Vec::new();

  for i in 0..splitCleaned.len() {
    let rawTok = &splitCleaned[i];
    if rawTok == "" {
      continue;
    }
    let sumPriorLen = sumPriorLengths[i];

    rawToksOrdered.push(rawTok.to_string());
    rawTokLocations.push(sumPriorLen as u16);

    m_rawTok_locations.entry(rawTok.to_string())
                      .or_insert_with(Vec::new)
                      .push(sumPriorLen as u16);
  }

  (m_rawTok_locations, rawToksOrdered, rawTokLocations)
}

fn delete_all_nonWord_chars_except_splitKeepChars<'a>(mut split2: Vec<String>, splitKeepChars: &str) -> Vec<String>{
  for i in 0..split2.len() {
    let tok = &split2[i];
    if tok.chars().count() == 1 {
      let c = tok.chars().next().unwrap();
      if not_a_word_scalar(c) && !splitKeepChars.contains(c) {
        split2[i] = "".to_string();
      }
    }
  }
  split2
}

pub fn divideInto3WordPhrases(phrase: &str) -> Vec<String> {

  let mut subPhrases: Vec<String> = Vec::new();

  let split:  Vec<&str>  = phrase.split(" ").collect();

  for i in 2..split.len() {
    subPhrases.push(format!("{} {} {}", split[i-2], split[i-1], split[i]));
  }
  subPhrases
}


pub fn splitUpLongQueryPhrase(origPhrase: &str) -> Vec<String> {
  let mut splittedQueryPhrases: Vec<String> = Vec::new();
  let numSpaces = origPhrase.matches(" ").count();
  if numSpaces > 2 {
    splittedQueryPhrases.extend(divideInto3WordPhrases(&origPhrase));
  }
  else {
    splittedQueryPhrases.push(origPhrase.to_string());
  }
  splittedQueryPhrases
}

pub fn splitUpLongQueryPhrases(origCookedPhrases: &Vec<String>) -> Vec<String> {

  let mut splittedQueryPhrases: Vec<String> = Vec::new();

  for origPhrase in origCookedPhrases {
    splittedQueryPhrases.extend(splitUpLongQueryPhrase(origPhrase));
  }
  splittedQueryPhrases
}


pub fn parsePhrasesAndExcluders(s: &str) -> (Vec<String>, Vec<String>) {

  let mut query = s.replace("\"", " \" ");
  // println!("{:?}", query);

  let numDubquotes = query.matches("\"").count();
  // println!("numDubquotes: {:?}", numDubquotes);
  if numDubquotes % 2 == 1 {
    let dubQuoteIndex = query.rfind("\"").unwrap();
    query.replace_range(dubQuoteIndex..=dubQuoteIndex, "");
  }
  let splitIntoPhrases = shellwords::split(&query).unwrap();

  let mut includePhrases: Vec<String> = Vec::new();
  let mut excludePhrases: Vec<String> = Vec::new();
  for i in 0..splitIntoPhrases.len() {
    let phrase = splitIntoPhrases[i].trim().to_string();
    if phrase.starts_with("-") {
      excludePhrases.push(phrase);
    } else {
      includePhrases.push(phrase);
    }
  }
  (includePhrases, excludePhrases)
}
