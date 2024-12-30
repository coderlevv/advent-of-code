export function hasSolution(
  val: number,
  sol: number,
  num: number[],
  idx: number = 1,
): boolean {
  debugger;
  if (idx === num.length) return false;
  let solved: boolean = false;
  let newVal: number;
  for (const op of ["+", "*", "||"]) {
    switch (op) {
      case "+":
        newVal = val + num[idx];
        break;
      case "*":
        newVal = val * num[idx];
        break;
      case "||":
        newVal = Number(`${val}${num[idx]}`);
        break;
      default:
        throw "Unknown operand!";
    }
    if (idx === num.length - 1 && newVal === sol) return true;
    solved = hasSolution(newVal, sol, num, idx + 1);
    if (solved) break;
  }
  return solved;
}

export function split(arr: number[]): number[][] {
  if (arr.length === 1) return [arr];
  let res: number[][] = [];
  for (let idx = 1; idx < arr.length; idx++)
    res = [arr.slice(0, idx), arr.slice(idx)];
  return res;
}

export function partition(arr: number[]): number[][][] {
  const result: number[][][] = [];
  const backtrack = (start: number, currentPartition: number[][]) => {
    if (start === arr.length) {
      result.push(currentPartition.map((part) => [...part]));
      return;
    }
    for (let i = start; i < arr.length; i++) {
      const newPartition = arr.slice(start, i + 1);
      currentPartition.push(newPartition);
      backtrack(i + 1, currentPartition);
      currentPartition.pop();
    }
  };

  backtrack(0, []);

  return result;
}

export function aggregate(arr: number[]): number[] {
  const results: Set<number> = new Set<number>();
  const backtrack = (index: number, currentResult: number) => {
    if (index >= arr.length) {
      results.add(currentResult);
      return;
    }
    backtrack(index + 1, currentResult + arr[index]);
    if (index === 0) {
      backtrack(index + 1, arr[index]);
    } else {
      backtrack(index + 1, currentResult * arr[index]);
    }
  };
  backtrack(0, 0);
  return Array.from(results); // Convert Set to Array for output
}
