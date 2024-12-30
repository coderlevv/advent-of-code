export class Robot {
  px: number;
  py: number;
  vx: number;
  vy: number;

  constructor(px: number, py: number, vx: number, vy: number) {
    this.px = px;
    this.py = py;
    this.vx = vx;
    this.vy = vy;
  }

  move(xDim: number, yDim: number): void {
    this.px = (this.px + this.vx) % xDim;
    this.py = (this.py + this.vy) % yDim;
    if (this.px < 0) this.px = xDim + this.px;
    if (this.py < 0) this.py = yDim + this.py;
  }
}
