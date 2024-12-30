enum BTYPE {
  File,
  Space,
}

class Block {
  id: number | undefined;
  btype: BTYPE;
  size: number;

  constructor(id: number | undefined, btype: BTYPE, size: number) {
    this.id = id;
    this.btype = btype;
    this.size = size;
  }
}

export class Disk {
  blocks: Block[];

  static from(str: string): Disk {
    let blocks: Block[] = [];
    let id = 0;
    [...str].forEach((size, idx) => {
      let block: Block;
      if (idx % 2 === 0) {
        block = new Block(id, BTYPE.File, Number(size));
        id++;
      } else {
        block = new Block(undefined, BTYPE.Space, Number(size));
      }
      blocks.push(block);
    });
    return new Disk(blocks);
  }

  constructor(blocks: Block[]) {
    this.blocks = blocks;
  }

  getIdx(): [number, number] {
    let start = 0;
    for (; start < this.blocks.length; start++)
      if (
        this.blocks[start].btype === BTYPE.Space &&
        this.blocks[start].size > 0
      )
        break;
    let end = this.blocks.length - 1;
    for (; end >= 0; end--)
      if (this.blocks[end].btype === BTYPE.File && this.blocks[end].size > 0)
        break;
    return [start, end];
  }

  merge() {
    let idx = 0;
    while (idx < this.blocks.length - 3) {
      if (
        this.blocks[idx].btype === BTYPE.File &&
        this.blocks[idx].id === this.blocks[idx + 2].id
      ) {
        this.blocks[idx].size += this.blocks[idx + 2].size;
        this.blocks.splice(idx + 1, 2);
      }
      idx++;
    }
  }

  defrag1(): void {
    let [startIdx, endIdx] = this.getIdx();
    while (startIdx < endIdx) {
      const spaceBlock = this.blocks[startIdx];
      const fileBlock = this.blocks[endIdx];
      // insert
      let newBlock = new Block(fileBlock.id, BTYPE.File, 0);
      this.blocks.splice(
        startIdx,
        0,
        new Block(undefined, BTYPE.Space, 0),
        newBlock,
      );
      // move
      while (fileBlock.size > 0 && spaceBlock.size > 0) {
        spaceBlock.size--;
        fileBlock.size--;
        newBlock.size++;
      }
      this.merge();
      [startIdx, endIdx] = this.getIdx();
    }
  }

  defrag2(): void {
    let id = 0;
    for (const b of this.blocks)
      if (b.id !== undefined) if (b.id > id) id = b.id;
    debugger;
    while (id > 0) {
      let fileIdx = this.blocks.length - 1;
      for (; fileIdx > 0; fileIdx--) if (this.blocks[fileIdx].id === id) break;
      let fileBlock = this.blocks[fileIdx];
      let spaceIdx = 0;
      for (; spaceIdx < fileIdx; spaceIdx++)
        if (
          this.blocks[spaceIdx].btype === BTYPE.Space &&
          this.blocks[spaceIdx].size >= fileBlock.size
        )
          break;
      // if no suitable space is found, next file
      if (spaceIdx === fileIdx) {
        id--;
        continue;
      }
      // found a suitable space
      let spaceBlock = this.blocks[spaceIdx];
      if (fileIdx - 1 === spaceIdx) {
        let newSpaceSize =
          fileBlock.size +
          (fileIdx === this.blocks.length - 1
            ? 0
            : this.blocks[fileIdx + 1].size);
        this.blocks.splice(
          fileIdx,
          2,
          new Block(undefined, BTYPE.Space, newSpaceSize),
        );
        this.blocks.splice(
          spaceIdx,
          0,
          new Block(undefined, BTYPE.Space, 0),
          new Block(fileBlock.id, BTYPE.File, fileBlock.size),
        );
        this.blocks[spaceIdx + 2].size = spaceBlock.size - fileBlock.size;
      } else {
        // replace file block with spaces
        let newSpaceSize =
          (fileIdx === 0 ? 0 : this.blocks[fileIdx - 1].size) +
          fileBlock.size +
          (fileIdx === this.blocks.length - 1
            ? 0
            : this.blocks[fileIdx + 1].size);
        this.blocks.splice(
          fileIdx - 1,
          3,
          new Block(undefined, BTYPE.Space, newSpaceSize),
        );
        this.blocks.splice(
          spaceIdx,
          0,
          new Block(undefined, BTYPE.Space, 0),
          new Block(fileBlock.id, BTYPE.File, fileBlock.size),
        );
        this.blocks[spaceIdx + 2].size = spaceBlock.size - fileBlock.size;
      }
      id--;
    }
  }

  checksum(): number {
    let sum = 0;
    let diskIdx = 0;
    this.blocks.forEach((block) => {
      if (block.id !== undefined) {
        for (let k = 0; k < block.size; k++) sum += (diskIdx + k) * block.id;
      }
      diskIdx += block.size;
    });
    return sum;
  }

  check(): boolean {
    let t = this.blocks[0].btype;
    for (let i = 1; i < this.blocks.length; i++) {
      if (t === this.blocks[i].btype) return false;
      t = this.blocks[i].btype;
    }
    return true;
  }
}
