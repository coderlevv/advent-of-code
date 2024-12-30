export function fit(design: string, patterns: string[], idx: number = 0): boolean {
    debugger;
    if (idx > design.length) return false;
    if (idx === design.length) return true;
    let seq = design.slice(idx);
    let found = false;
    for (const pattern of patterns) {
        if (seq.startsWith(pattern)) {
            found = fit(design, patterns, idx+pattern.length);
            if (found) break;
        }
    }
    return found;
}

const memo = new Map<string, number>();

export function countFits(design: string, patterns: string[], idx: number = 0): number {
    debugger;
    let memoCount = memo.get(`${design}_${idx}`);
    if (memoCount) return memoCount;
    let count = 0;
    if (idx > design.length) return 0;
    if (idx === design.length) return 1;
    let seq = design.slice(idx);
    for (const pattern of patterns) {
        if (seq.startsWith(pattern)) {
            count += countFits(design, patterns, idx+pattern.length);
        }
    }
    memo.set(`${design}_${idx}`, count);
    return count;
}