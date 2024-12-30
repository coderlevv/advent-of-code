import * as fs from "fs";
import { search, Graph, findMaximalCliques } from "./connect.js";

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

  let connect = new Map<string, Set<string>>();
  for (const line of lines) {
    let [conn1, conn2] = line.split("-");
    let entry1 = connect.get(conn1);
    if (entry1 === undefined) entry1 = new Set<string>();
    entry1.add(conn2);
    connect.set(conn1, entry1);
    let entry2 = connect.get(conn2);
    if (entry2 === undefined) entry2 = new Set<string>();
    entry2.add(conn1);
    connect.set(conn2, entry2);
  }

  let maxConnect = Math.max(...[...connect.values()].map((v) => v.size));
  //console.log(maxConnect);

  let res1 = new Set<string>();
  for (const conn of connect.keys()) {
    search(conn, [], connect, res1);
  }

  const graph: Graph = {};
  for (const [key, value] of connect.entries()) {
    graph[key] = value; // Convert Set<string> to string[]
  }

  let cliques = findMaximalCliques(graph);
  const maxClique = cliques.reduce(
    (largest, clique) => (clique.size > largest.size ? clique : largest),
    new Set<string>(),
  );
  //console.log("Maximum Clique:", Array.from(maxClique));

  return [
    [...res1].filter(
      (r) => r.split(",").filter((n) => n.startsWith("t")).length > 0,
    ).length,
    Array.from(maxClique).sort().join(),
  ];
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
