
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

/**
 * https://stackoverflow.com/questions/6274339/how-can-i-shuffle-an-array
 * fisher-yates
 * Shuffles array in place.
 * @param {Array} a items An array containing the items.
 */
function shuffle(a) {
  var j, x, i;
  for (i = a.length - 1; i > 0; i--) {
    j = Math.floor(Math.random() * (i + 1));
    x = a[i];
    a[i] = a[j];
    a[j] = x;
  }
  return a;
}

function chunkSubstr(str, size) {
  const numChunks = Math.ceil(str.length / size)
  const chunks = new Array(numChunks)

  for (let i = 0, o = 0; i < numChunks; ++i, o += size) {
    chunks[i] = str.substr(o, size)
  }

  return chunks
}

/**
 * https://stackoverflow.com/questions/10049557/reading-all-files-in-a-directory-store-them-in-objects-and-send-the-object
 * @description Read files synchronously from a folder, with natural sorting
 * @param {String} dir Absolute path to directory
 * @returns {Object[]} List of object, each object represent a file
 * structured like so: `{ filepath, name, ext, stat }`
 */
function readFilesSync(dir) {
  let files = "";

  fs.readdirSync(dir).forEach(filename => {

    const filepath = path.resolve(dir, filename);

    console.log(filepath);

    const content = fs.readFileSync(filepath).toString().replace(/[\r\n]+/g, " ");
    // const content = fs.readFileSync(filepath).toString()
    // const content = fs.readFileSync(filepath).toString();//.replace("\r", '').replace('\n', '').replace('\r\n', '');
    // console.log(content)
    // console.log(typeof content)
    // exit();

    // const name = path.parse(filename).name;
    // const ext = path.parse(filename).ext;
    const stat = fs.statSync(filepath);
    const isFile = stat.isFile();

    // if (isFile) files.push({ filepath, name, ext, stat });
    // if (isFile) files.push(content);
    if (isFile) files = files.concat(content);
  });

  // files.sort((a, b) => {
  //   // natural sort alphanumeric strings
  //   // https://stackoverflow.com/a/38641281
  //   return a.name.localeCompare(b.name, undefined, { numeric: true, sensitivity: 'base' });
  // });

  return files;
}

//read all text files in dir into a single ... string?
//split into 5kb chunks and write each as a single line to a file
//scramble the huge string, do it again?

const dir = "/Users/stuartrobinson/repos/corpa/proj_guttenberg_novels"

// return an array list of objects
// each object represent a file
let files = readFilesSync(dir);

// console.log(files)

console.log(files.length)

// let chunks = chunkSubstr(files, 5000)

fs.unlinkSync("hidden/100k_docs.txt")

for (let i = 0; i < 14; i++) {
  console.log(i);

  let chunks = chunkSubstr(files, 5000)

  // chunks.forEach(c => fs.appendFileSync("hidden/100k_docs.txt", c.substr(0, 30) + '\n'))
  chunks.forEach(c => fs.appendFileSync("hidden/100k_docs.txt", c + '\n'))

  let filesAr = files.split(' ');
  filesAr = shuffle(filesAr);
  files = filesAr.join(" ");


}

// /Users/stuartrobinson/repos/worp/worp-rust/docs_generator/hidden/100k_docs_5kb_each.txt


// console.log(chunks.length)


// fs.writeFileSync("hidden/allBooksConcatted.txt", files)

