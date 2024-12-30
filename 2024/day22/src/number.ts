export function* pseudo(start: number) {
    let n = start;
    while (true) {
        n = (n ^ (n * 64)) % 16777216;
        n = (n ^ Math.floor(n / 32)) % 16777216;
        n = (n ^ (n * 2048)) % 16777216;
        n = n >= 0 ? n : 16777216 + n;
        yield n;
    }
}