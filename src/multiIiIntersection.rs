#![allow(non_snake_case)]
use binary_heap_plus::BinaryHeap;
use crate::global::ascendingComparator_3ple;
use crate::global::descendingComparator_3ple;

use std::collections::HashMap;

#[macro_export]

macro_rules! getDocId {
    ($ma:expr, $mi:expr) => {(($ma[$mi + 2] as u32) << 16) + (($ma[$mi + 1] as u32) << 8) + ($ma[$mi] as u32)};
}

macro_rules! getRel {
    ($m2a:expr, $m2i:expr) => {(($m2a[$m2i + 4] as u16) << 8) + ($m2a[$m2i + 3] as u16)};
}

/// returns tuple (numTotalResults, page1_intersection, sortValue)
/// if not sorting by anything, then sortValue is 0.0
pub fn multiIiIntersection( invinds:            &mut std::vec::Vec<&std::vec::Vec<u8>>,
                            excludeInvinds:     &mut std::vec::Vec<&std::vec::Vec<u8>>,
                            useAltSort:         bool,
                            sovabydid:          &Vec<f32>,
                            getTimeCounts:      bool,
                            tivabydid:          &Vec<u64>,
                            doSortDescending:   bool,
                            numResultsToReturn: usize)      ->       (usize,  Vec<(u32, u16, f32)>,  HashMap<u32, u32>) {

  let myclosureAscending = |a: &(u32, u16, f32), b: &(u32, u16, f32)| ascendingComparator_3ple(a, b);
  let myclosureDescending = |a: &(u32, u16, f32), b: &(u32, u16, f32)| descendingComparator_3ple(a, b);
  let myclosure =  if doSortDescending { myclosureDescending } else { myclosureAscending };
  let mut altSortResultsTuple_docId_rel_sortValue = BinaryHeap::from_vec_cmp(Vec::new(), myclosure);
  let mut altSortResultTuple_size = 0;
  let mut altSortResultTuple_lastValue = if doSortDescending { std::f32::MIN } else { std::f32::MAX };
  let firstIsBetter_ascending = |a: f32, b: f32| a < b;
  let firstIsBetter_descending = |a: f32, b: f32| a > b;
  let firstIsBetter = if doSortDescending { firstIsBetter_descending } else { firstIsBetter_ascending };
  let mut bucket0: Vec<(u32, u16, f32)>  = Vec::new();
  let mut bucket1: Vec<(u32, u16, f32)>  = Vec::new();
  let mut bucket2: Vec<(u32, u16, f32)>  = Vec::new();
  let mut bucket3: Vec<(u32, u16, f32)>  = Vec::new();
  let mut bucket4: Vec<(u32, u16, f32)>  = Vec::new();
  let mut bucket5: Vec<(u32, u16, f32)>  = Vec::new();
  let mut bucket6: Vec<(u32, u16, f32)>  = Vec::new();
  let mut bucket7: Vec<(u32, u16, f32)>  = Vec::new();
  let mut bucket8: Vec<(u32, u16, f32)>  = Vec::new();
  let mut bucket9: Vec<(u32, u16, f32)>  = Vec::new();

  let mut m_timeBin_count: HashMap<u32, u32> = HashMap::new();


  let debug = false;
  ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
  ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

  invinds.sort_by(|a, b| a.len().cmp(&b.len()));    // important for speed.  first should be shortest

  if invinds.len() > 6 { panic!("sorry, max 5 tokens supported rn");}

  let numInvinds = invinds.len();

  let     v0 = invinds[0]; // inverted index 0
  let mut v1 = &Vec::new();
  let mut v2 = &Vec::new();
  let mut v3 = &Vec::new();
  let mut v4 = &Vec::new();
  let mut v5 = &Vec::new();
  if numInvinds > 1 { v1 = invinds[1];}       //could be combined into single line.  fake ternary
  if numInvinds > 2 { v2 = invinds[2];}
  if numInvinds > 3 { v3 = invinds[3];}
  if numInvinds > 4 { v4 = invinds[4];}
  if numInvinds > 5 { v5 = invinds[5];}

  let len0 = v0.len();
  let mut len1 = 0;
  let mut len2 = 0;
  let mut len3 = 0;
  let mut len4 = 0;
  let mut len5 = 0;
  if numInvinds > 1 { len1 = v1.len()}
  if numInvinds > 2 { len2 = v2.len()}
  if numInvinds > 3 { len3 = v3.len()}
  if numInvinds > 4 { len4 = v4.len()}
  if numInvinds > 5 { len5 = v5.len()}

  let ar0 = &v0[..];
  let ar1 = &v1[..];
  let ar2 = &v2[..];
  let ar3 = &v3[..];
  let ar4 = &v4[..];
  let ar5 = &v5[..];

  let mut i0 = 0; // inverted index 0 iterator
  let mut i1 = 0;
  let mut i2 = 0;
  let mut i3 = 0;
  let mut i4 = 0;
  let mut i5 = 0;

  let mut d0;     //docId0
  let mut d1;
  let mut d2;
  let mut d3;
  let mut d4;
  let mut d5;
  //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

  if excludeInvinds.len() > 5 { panic!("sorry, max 5 exclude tokens supported rn");}

  let numExcludeInvinds = excludeInvinds.len();

  let mut ev0 = &Vec::new();
  let mut ev1 = &Vec::new();
  let mut ev2 = &Vec::new();
  let mut ev3 = &Vec::new();
  let mut ev4 = &Vec::new();
  let mut ev5 = &Vec::new();
  if numExcludeInvinds > 0 { ev0 = excludeInvinds[0];}       //could be combined into single line.  fake ternary
  if numExcludeInvinds > 1 { ev1 = excludeInvinds[1];}       //could be combined into single line.  fake ternary
  if numExcludeInvinds > 2 { ev2 = excludeInvinds[2];}
  if numExcludeInvinds > 3 { ev3 = excludeInvinds[3];}
  if numExcludeInvinds > 4 { ev4 = excludeInvinds[4];}
  if numExcludeInvinds > 5 { ev5 = excludeInvinds[5];}

  let elen0 = ev0.len();
  let mut elen1 = 0;
  let mut elen2 = 0;
  let mut elen3 = 0;
  let mut elen4 = 0;
  let mut elen5 = 0;
  if numExcludeInvinds > 1 { elen1 = ev1.len()}
  if numExcludeInvinds > 2 { elen2 = ev2.len()}
  if numExcludeInvinds > 3 { elen3 = ev3.len()}
  if numExcludeInvinds > 4 { elen4 = ev4.len()}
  if numExcludeInvinds > 5 { elen5 = ev5.len()}

  let ear0 = &ev0[..];
  let ear1 = &ev1[..];
  let ear2 = &ev2[..];
  let ear3 = &ev3[..];
  let ear4 = &ev4[..];
  let ear5 = &ev5[..];

  let mut ei0 = 0; // inverted index 0 iterator
  let mut ei1 = 0;
  let mut ei2 = 0;
  let mut ei3 = 0;
  let mut ei4 = 0;
  let mut ei5 = 0;

  let mut ed0: u32;     //docId0
  let mut ed1: u32;
  let mut ed2: u32;
  let mut ed3: u32;
  let mut ed4: u32;
  let mut ed5: u32;

                                                                                        // ignore the warning.  "while true" is faster than "loop", i checked.
  'outer: while true {

    if len0 > 0 {

      d0 = getDocId!(ar0, i0);
      if debug  {if d0 > 14599 { break;}}

      if len1 > 0 {
          d1 = getDocId!(ar1, i1);
          while d1 < d0 {
              i1 += 5; if i1 >= len1 {break 'outer;}
              d1 = getDocId!(ar1, i1);
          }
          if d1 > d0 {i0+=5;  if i0 >= len0 {break 'outer;}   continue; }                       // by "continue"ing, we're startig over w/ a new d0.  otherwise, they're the same, so ... (else)
          else {
              if debug {println!("match! 2: iters: {} {}, value: {}", i0, i1, d0);}///////// matched! ///////////////

              if len2 > 0 {
                  d2 = getDocId!(ar2, i2);
                  while d2 < d0 {
                      i2 += 5; if i2 >= len2 {break 'outer;}
                      d2 = getDocId!(ar2, i2);
                  }
                  if d2 > d0 {i0+=5; if i0 >= len0 {break 'outer;} /**/ i1+=5; if i1 >= len1 {break 'outer;} continue;}
                  else {
                      if debug {println!("match! 3: iters: {} {}, value: {}", i0, i2, d0);}///////// matched! ///////////////

                      if len3 > 0 {
                          d3 = getDocId!(ar3, i3);
                          while d3 < d0 {
                              i3 += 5; if i3 >= len3 {break 'outer;}
                              d3 = getDocId!(ar3, i3);
                          }
                          if d3 > d0 {i0+=5; if i0 >= len0 {break 'outer;} /**/ i1+=5; if i1 >= len1 {break 'outer;} /**/ i2+=5; if i2 >= len2 {break 'outer;} continue;}
                          else {
                              if debug {println!("match! 4: iters: {} {}, value: {}", i0, i3, d0);}///////// matched! ///////////////

                              if len4 > 0 {
                                  d4 = getDocId!(ar4, i4);
                                  while d4 < d0 {
                                      i4 += 5; if i4 >= len4 {break 'outer;}
                                      d4 = getDocId!(ar4, i4);
                                  }
                                  if d4 > d0  {i0+=5; if i0 >= len0 {break 'outer;} /**/ i1+=5; if i1 >= len1 {break 'outer;} /**/ i2+=5; if i2 >= len2 {break 'outer;} /**/ i3+=5; if i3 >= len3 {break 'outer;} continue;}
                                  else {
                                      if debug {println!("match! 5: iters: {} {}, value: {}", i0, i4, d0);}///////// matched! ///////////////

                                      if len5 > 0 {
                                          d5 = getDocId!(ar5, i5);
                                          while d5 < d0 {
                                              i5 += 5; if i5 >= len5 {break 'outer;}
                                              d5 = getDocId!(ar5, i5);
                                          }
                                          if d5 > d0 {i0+=5; if i0 >= len0 {break 'outer;} /**/ i1+=5; if i1 >= len1 {break 'outer;} /**/ i2+=5; if i2 >= len2 {break 'outer;} /**/ i3+=5; if i3 >= len3 {break 'outer;} /**/ i4+=5; if i4 >= len4 {break 'outer;} continue;}
                                          else {
                                              if debug {println!("match! 6: iters: {} {}, value: {}", i0, i5, d0);}///////// matched! ///////////////
                                          }
                                      }
                                  }
                              }
                          }
                      }
                  }
              }
          }
      }
      //now make sure document w id "d0"  is not in any of the exludeInvinds. "e" for exclude

      if ei0 < elen0  {
        ed0 = getDocId!(ear0, ei0);
        while ed0 < d0  &&  ei0 < elen0 - 5 {         //TODO these "- 5"'s are wasteful!  rewrite the logic to avoid this - actually the diff is prob trivial or DNE
            ei0 += 5;
            ed0 = getDocId!(ear0, ei0);
        }
        if ed0 == d0 {
          ei0 += 5;
          if len0 > 0 { i0 += 5; if i0 >= len0 {break;}}
          continue;
        }
      }
      if ei1 < elen1 {
        ed1 = getDocId!(ear1, ei1);
        while ed1 < d0  &&  ei1 < elen1 - 5 {
            ei1 += 5;
            ed1 = getDocId!(ear1, ei1);
        }
        if ed1 == d0 {
          ei1 += 5;
          if len0 > 0 { i0 += 5; if i0 >= len0 {break;}}
          continue;
        }
      }
      if ei2 < elen2 {
        ed2 = getDocId!(ear2, ei2);
        while ed2 < d0  &&  ei2 < elen2 - 5 {
            ei2 += 5;
            ed2 = getDocId!(ear2, ei2);
        }
        if ed2 == d0 {
          ei2 += 5;
          if len0 > 0 { i0 += 5; if i0 >= len0 {break;}}
          continue;
        }
      }
      if ei3 < elen3 {
        ed3 = getDocId!(ear3, ei3);
        while ed3 < d0  &&  ei3 < elen3 - 5 {
            ei3 += 5;
            ed3 = getDocId!(ear3, ei3);
        }
        if ed3 == d0 {
          ei3 += 5;
          if len0 > 0 { i0 += 5; if i0 >= len0 {break;}}
          continue;
        }
      }
      if ei4 < elen4 {
        ed4 = getDocId!(ear4, ei4);
        while ed4 < d0  &&  ei4 < elen4 - 5 {
            ei4 += 5;
            ed4 = getDocId!(ear4, ei4);
        }
        if ed4 == d0 {
          ei4 += 5;
          if len0 > 0 { i0 += 5; if i0 >= len0 {break;}}
          continue;
        }
      }
      if ei5 < elen5 {
        ed5 = getDocId!(ear5, ei5);
        while ed5 < d0  &&  ei5 < elen5 - 5 {
            ei5 += 5;
            ed5 = getDocId!(ear5, ei5);
        }
        if ed5 == d0 {
          ei5 += 5;
          if len0 > 0 { i0 += 5; if i0 >= len0 {break;}}
          continue;
        }
      }

      if getTimeCounts {
        // println!("if getTimeCounts ... tivabydid: {:?}", tivabydid);
        let docTimestampMillis = tivabydid[d0 as usize];
        let tenMinuteBinStart = (docTimestampMillis / 600000) as u32;
        *m_timeBin_count.entry(tenMinuteBinStart).or_insert(0) += 1;                                   //https://users.rust-lang.org/t/efficient-string-hashmaps-for-a-frequency-count/7752/2
      }

      if useAltSort {
        let mut relevance: f32 = getRel!(ar0, i0) as f32;
        if numInvinds > 1 { relevance *= getRel!(ar1, i1) as f32; }
        if numInvinds > 2 { relevance *= getRel!(ar2, i2) as f32; }
        if numInvinds > 3 { relevance *= getRel!(ar3, i3) as f32; }
        if numInvinds > 4 { relevance *= getRel!(ar4, i4) as f32; }
        if numInvinds > 5 { relevance *= getRel!(ar5, i5) as f32; }
        //if 5 vecs intersected: normalize from ... 256^10 to 256^2 - how? divide by 256^8
        //if n vecs intersected: normalize from ... 256^(n*2) to 256^2 - how?  rel_normalized = rel / 256^(n*2 - 2)
        //normalize
        let normalizedRelevance: f32 = relevance/f32::powf(256 as f32, (numInvinds as f32)*2.0 - 1.0);
        let temp1 =  (normalizedRelevance.log(10.0) + 6.0)*10.0;/////////numInvinds
        let relNormAdjusted = temp1 as u16;
        if temp1 > 65535.0 {
            panic!("too big ???!?!?!?! no idea");                                                   // wont the prev line fail if this is true? maybe not
        }

        // keep vars with: altSortResultTuple_size, altSortResultTuple_lastValue,  (last value is like ... the biggest if sorted ascending.  smallest if descending)
        let sortValue = sovabydid[d0 as usize];

        // println!("d0: {}, sovabydid.len(): {}, sortValue: {}", d0, sovabydid.len(), sortValue);

        if altSortResultTuple_size < numResultsToReturn {
          altSortResultsTuple_docId_rel_sortValue.push((d0, relNormAdjusted, sortValue));
          altSortResultTuple_size = altSortResultsTuple_docId_rel_sortValue.len();
        }
        else if firstIsBetter(sortValue, altSortResultTuple_lastValue) {
          altSortResultsTuple_docId_rel_sortValue.push((d0, relNormAdjusted, sortValue));
          altSortResultsTuple_docId_rel_sortValue.pop();
          altSortResultTuple_lastValue = altSortResultsTuple_docId_rel_sortValue.peek().unwrap().2;
        }


      } else {

        // multiply the relevances for all the tokens for this one docId.  big number but next we'll normalize

        let mut relevance: f32 = getRel!(ar0, i0) as f32;

        if numInvinds > 1 { relevance *= getRel!(ar1, i1) as f32; }
        if numInvinds > 2 { relevance *= getRel!(ar2, i2) as f32; }
        if numInvinds > 3 { relevance *= getRel!(ar3, i3) as f32; }
        if numInvinds > 4 { relevance *= getRel!(ar4, i4) as f32; }
        if numInvinds > 5 { relevance *= getRel!(ar5, i5) as f32; }

        //if 5 vecs intersected: normalize from ... 256^10 to 256^2 - how? divide by 256^8
        //if n vecs intersected: normalize from ... 256^(n*2) to 256^2 - how?  rel_normalized = rel / 256^(n*2 - 2)

        //normalize
        let normalizedRelevance: f32 = relevance/f32::powf(256 as f32, (numInvinds as f32)*2.0 - 1.0);
        let temp1 =  (normalizedRelevance.log(10.0) + 6.0)*10.0;/////////numInvinds
        let relNormAdjusted = temp1 as u16;
        if temp1 > 65535.0 {
            panic!("too big ???!?!?!?! no idea");                                                   // wont the prev line fail if this is true? maybe not
        }
        if      relNormAdjusted as f32 >= 80.0 {bucket0.push((d0, getRel!(ar0, i0), 0.0));}              // shouldn't these be pushing relNormAdjusted ??? instead of relevance for ar0,i0 ?
        else if relNormAdjusted as f32 >= 75.0 {bucket1.push((d0, getRel!(ar0, i0), 0.0));}
        else if relNormAdjusted as f32 >= 70.0 {bucket2.push((d0, getRel!(ar0, i0), 0.0));}
        else if relNormAdjusted as f32 >= 60.0 {bucket3.push((d0, getRel!(ar0, i0), 0.0));}
        else if relNormAdjusted as f32 >= 50.0 {bucket4.push((d0, getRel!(ar0, i0), 0.0));}
        else if relNormAdjusted as f32 >= 40.0 {bucket5.push((d0, getRel!(ar0, i0), 0.0));}
        else if relNormAdjusted as f32 >= 30.0 {bucket6.push((d0, getRel!(ar0, i0), 0.0));}
        else if relNormAdjusted as f32 >= 20.0 {bucket7.push((d0, getRel!(ar0, i0), 0.0));}
        else if relNormAdjusted as f32 >= 10.0 {bucket8.push((d0, getRel!(ar0, i0), 0.0));}
        else if relNormAdjusted as f32 >= 00.0 {bucket9.push((d0, getRel!(ar0, i0), 0.0));}
      }
    }
    else {break;}

    if len0 > 0 { i0 += 5; if i0 >= len0 {break;}}
    if len1 > 0 { i1 += 5; if i1 >= len1 {break;}}
    if len2 > 0 { i2 += 5; if i2 >= len2 {break;}}
    if len3 > 0 { i3 += 5; if i3 >= len3 {break;}}
    if len4 > 0 { i4 += 5; if i4 >= len4 {break;}}
    if len5 > 0 { i5 += 5; if i5 >= len5 {break;}}
  }


  if useAltSort {

    //TODO dont strip out sort value!  return it to the owl
    for (docId, rel, sortValue) in altSortResultsTuple_docId_rel_sortValue {
      bucket0.push((docId, rel, sortValue));
    }

    return (  bucket0.len(),  bucket0,  m_timeBin_count  );
  }
  else {

    let mut numTotalResults = 0;
    let mut page1: Vec<(u32, u16, f32)> = Vec::new();
    let mut buckets: Vec<Vec<(u32, u16, f32)>> = vec![bucket0,
                                                      bucket1,
                                                      bucket2,
                                                      bucket3,
                                                      bucket4,
                                                      bucket5,
                                                      bucket6,
                                                      bucket7,
                                                      bucket8,
                                                      bucket9];
    for i in 0..buckets.len(){
        numTotalResults += buckets[i].len();                          // println!("bucket{} len: {}", i, buckets[i].len());
        if page1.len() < numResultsToReturn {
            page1.append(&mut buckets[i]);
        }
    }
    return (  numTotalResults,  page1[0..std::cmp::min(numResultsToReturn, page1.len())].to_vec(),  m_timeBin_count  );
  }
}