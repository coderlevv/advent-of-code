export class Device {
  A: number;
  B: number;
  C: number;
  prg: number[];
  out: number[];
  ip: number;

  static from(lines: string[]): Device {
    let A: number = 0;
    let B: number = 0;
    let C: number = 0;
    let prg: number[] = [];
    for (const line of lines) {
      if (line.startsWith("Register A:")) A = Number(line.split(":")[1]);
      if (line.startsWith("Register B:")) B = Number(line.split(":")[1]);
      if (line.startsWith("Register C:")) C = Number(line.split(":")[1]);
      if (line.startsWith("Program:")) {
        let tmp = line.split(":")[1];
        prg = tmp.split(",").map(Number);
      }
    }
    return new Device(A, B, C, prg);
  }

  constructor(A: number, B: number, C: number, prg: number[] = []) {
    this.A = A;
    this.B = B;
    this.C = C;
    this.prg = prg;
    this.out = [];
    this.ip = 0;
  }

  comboVal(op: number): number {
    if (op >= 0 && op <= 3) return op;
    switch (op) {
      case 4:
        return this.A;
      case 5:
        return this.B;
      case 6:
        return this.C;
      default:
        throw "Invalid combo operand!";
    }
  }

  output(): string {
    return this.out.join(",");
  }

  reset(): void {
    this.ip = 0;
    this.out = [];
  }

  run() {
    while (this.ip < this.prg.length) {
      const instr = this.prg[this.ip];
      const op = this.prg[this.ip + 1];
      switch (instr) {
        case 0:
          this.A = Math.floor(this.A / 2 ** this.comboVal(op));
          break;
        case 1:
          this.B = this.B ^ op;
          break;
        case 2:
          this.B = this.comboVal(op) % 8;
          break;
        case 3:
          this.ip = this.A === 0 ? this.ip : op;
          if (this.A !== 0) continue;
          break;
        case 4:
          this.B = this.B ^ this.C;
          break;
        case 5:
          this.out.push(this.comboVal(op) % 8);
          break;
        case 6:
          this.B = Math.floor(this.A / 2 ** this.comboVal(op));
          break;
        case 7:
          this.C = Math.floor(this.A / 2 ** this.comboVal(op));
          break;
        default:
          throw "Invalid instruction!";
      }
      this.ip += 2;
    }
  }
}
