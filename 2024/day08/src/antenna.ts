type Position = [number, number];

export class Antenna {
  pos: Position;
  freq: string;

  constructor(x: number, y: number, freq: string) {
    this.pos = [x, y];
    this.freq = freq;
  }
}

export function antinodes(a: Antenna, b: Antenna): [Position, Position] {
  const dx = b.pos[0] - a.pos[0];
  const dy = b.pos[1] - a.pos[1];
  return [
    [a.pos[0] - dx, a.pos[1] - dy],
    [a.pos[0] + 2 * dx, a.pos[1] + 2 * dy],
  ];
}

export function harmonics(
  a: Antenna,
  b: Antenna,
  xdim: number,
  ydim: number,
): Position[] {
  let nodes: Position[] = [];
  const dx = b.pos[0] - a.pos[0];
  const dy = b.pos[1] - a.pos[1];
  if (dx !== 0) {
    for (let s = 1; a.pos[0] - s >= 0; s++)
      nodes.push([a.pos[0] - s * dx, a.pos[1] - s * dy]);
    for (let s = 1; a.pos[0] + s < xdim; s++)
      nodes.push([a.pos[0] + s * dx, a.pos[1] + s * dy]);
  } else {
    for (let s = 1; a.pos[1] - s >= 0; s++)
      nodes.push([a.pos[0], a.pos[1] - s * dy]);
    for (let s = 1; a.pos[0] + s < ydim; s++)
      nodes.push([a.pos[0], a.pos[1] + s * dy]);
  }
  return nodes;
}
