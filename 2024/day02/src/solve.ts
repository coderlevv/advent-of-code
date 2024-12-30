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

function isSafe(nums: number[]): boolean {
  let inc: boolean | undefined = undefined;
  for (let i = 1; i < nums.length; i++) {
    const diff = nums[i] - nums[i - 1];
    if (inc === undefined) inc = diff > 0 ? true : false;
    if (diff === 0) return false;
    if ((diff < 0 && inc) || (diff > 0 && !inc)) return false;
    if (Math.abs(diff) < 1 || Math.abs(diff) > 3) return false;
  }
  return true;
}

async function solve(): Promise<[number, number]> {
  let lines: string[] = [];
  try {
    lines = (await readInput("input")) as string[];
  } catch (err) {
    throw err;
  }

  let count1 = 0;
  let count2 = 0;
  for (const line of lines) {
    const nums: number[] = line.split(" ").map(Number);
    let safe = isSafe(nums);
    if (safe) {
      count1++;
      count2++;
    } else {
      let safeIdx: number[] = [];
      for (let rm = 0; rm < nums.length; rm++) {
        const rmNums = nums
          .map((n, i) => [n, i])
          .filter(([n, i]) => i !== rm)
          .map(([n, i]) => n);
        if (isSafe(rmNums)) safeIdx.push(rm);
      }
      if (safeIdx.length > 0) count2++;
    }
  }
  return [count1, count2];
}

solve()
  .then((res) => console.log(res))
  .catch((err) => {
    if ((err as NodeJS.ErrnoException).code === "ENOENT") {
      console.error("Please provide input file!");
    } else {
      console.error("An unexpected error occurred:", err);
    }
  });
