import * as fs from "fs";

const readInput = (inputFile: string) => {
  return new Promise((resolve, reject) => {
    fs.readFile(inputFile, (err, data) => {
      if (err) {
        reject(err);
      } else {
        const lines = data.toString().trim().split("\n");
        resolve(lines);
      }
    });
  });
};

async function solve() {
  let lines: string[] = [];
  try {
    lines = (await readInput("input")) as string[];
  } catch (err) {
    throw err;
  }

  let seq = lines[0].split(" ").map(Number);
  let blinks = 25;
  let len: number[] = [seq.length];
  while (blinks > 0) {
    let newSeq: number[] = [];
    for (let i = 0; i < seq.length; i++) {
      if (seq[i] === 0) {
        newSeq.push(1);
        continue;
      }
      let digits = seq[i].toString();
      if (digits.length % 2 === 0) {
        let mid = digits.length / 2;
        newSeq.push(Number(digits.slice(0, mid)));
        newSeq.push(Number(digits.slice(mid)));
        continue;
      }
      newSeq.push(seq[i] * 2024);
    }
    seq = newSeq;
    len.push(seq.length);
    blinks--;
  }
  //let f = fs.createWriteStream("length.txt");
  //len.forEach((n) => f.write(n.toString() + "\n"));
  //f.end();
  return seq.length;
}

solve()
  .then(console.log)
  .catch((err) => {
    if ((err as NodeJS.ErrnoException).code === "ENOENT") {
      console.error("Please provide input file!");
    } else {
      console.error("An unexpected error occurred:", err);
    }
  });
