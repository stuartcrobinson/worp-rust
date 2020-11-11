
import * as fs from 'fs';
import * as path from 'path';
import { exit } from 'process';

function getLineCount(file) {
  var i;
  var count = 0;
  fs
    .createReadStream(process.argv[2])
    .on('data', function (chunk) {
      for (i = 0; i < chunk.length; ++i) {
        if (chunk[i] == 10) {
          count++;
        }
      }
    })
    .on('end', function () {
      console.log(count);
    });
  return count;

}

const filepath = "/Users/stuartrobinson/repos/worp/worp-rust/docs_generator/hidden/100k_docs_5kb_each.txt"


console.log(getLineCount(filepath))