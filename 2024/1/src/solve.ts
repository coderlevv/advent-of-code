import * as fs from "fs";

const readInput = (inputFile: string) => {
  return new Promise((resolve, reject) => {
    fs.readFile(inputFile, (err, data) => {
      if (err) {
        reject(err);
      }
      const lines = data.toString().trim().split("\n");
      const seq1: number[] = [];
      const seq2: number[] = [];
      for (const line of lines) {
        const [n1, n2] = line.split(/\s+/).map(Number);
        seq1.push(n1);
        seq2.push(n2);
      }
      resolve([seq1.sort(), seq2.sort()]);
    });
  });
};

async function solve(): Promise<[number, number]> {
  let nums: number[][] = [];
  try {
    nums = (await readInput("input")) as number[][];
  } catch (err) {
    console.error(err);
  }

  let dist = 0;
  for (let i = 0; i < nums[0].length; i++)
    dist += Math.abs(nums[1][i] - nums[0][i]);

  let sim = 0;
  for (const num of nums[0])
    sim += num * nums[1].filter((n) => n === num).length;
  return [dist, sim];
}

solve()
  .then((res) => console.log(res))
  .catch((err) => console.log(err));
