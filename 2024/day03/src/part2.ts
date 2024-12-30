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

  let res = 0;
  let enabled: boolean = true;
  for (const code of lines) {
    for (let idx = 0; idx < code.length; idx++) {
      const slice = code.slice(idx);
      if (/^do\(\)/.exec(slice)) {
        enabled = true;
        continue;
      }
      if (/^don't\(\)/.exec(slice)) {
        enabled = false;
        continue;
      }
      const m = /^mul\((\d+),(\d+)\)/.exec(slice);
      if (m && enabled) res += Number(m[1]) * Number(m[2]);
    }
  }
  return res;
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
