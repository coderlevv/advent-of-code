import * as fs from "fs";
import { Circuit } from "./circuit.js";

const readInput = (inputFile: string) => {
  return new Promise((resolve, reject) => {
    fs.readFile(inputFile, (err, data) => {
      if (err) {
        reject(err);
      }
      const lines = data.toString().trim().split("\n");
      resolve(lines);
    });
  });
};

async function solve() {
  let lines: string[] = [];
  try {
    lines = (await readInput("input")) as string[];
  } catch (err) {
    console.error(err);
  }

  let circuit = Circuit.from(lines);
  let [xDim, yDim] = circuit.dim();

  // swapped gates were determined by
  // 1) by using x and y input to run bitwise addition
  // and checking the result
  // 2) checking the gates at and around the bits with
  // wrong results with paper & pencil
  const swap = (o1: string, o2: string) => {
    let g1 = circuit.gates.filter((g) => g.output === o1);
    let g2 = circuit.gates.filter((g) => g.output === o2);
    g1[0].output = o2;
    g2[0].output = o1;
  };
  swap("z32", "tbt");
  swap("kth", "z12");
  swap("qnf", "vpm");
  swap("z26", "gsd");

  // run bitwise addition to determine where the results
  // are wrong (before doing the swaps)
  let xs = [...Number(0).toString(2).padStart(xDim, "0")];
  let ys = [...Number(0).toString(2).padStart(yDim, "0")];
  for (let i = xDim - 1; i >= 0; i--) {
    ys[i] = "1";
    xs[i] = "1";
    circuit.setInput(xs.join(""), ys.join(""));
    circuit.apply();
    let out = circuit.output();
    let check =
      Number.parseInt(xs.join(""), 2) + Number.parseInt(ys.join(""), 2);
    if (out !== check) {
      console.log(`${i}: ${out}, ${check}`);
      console.log(`${xs.join("")}, ${ys.join("")}`);
    }
    ys[i] = "0";
    xs[i] = "0";
  }
  return ["z32", "tbt", "kth", "z12", "qnf", "vpm", "z26", "gsd"].sort().join();
}

solve().then(console.log);
