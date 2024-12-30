export type PriorityType = { priority: number; [key: string]: any };

export class PriorityQueue<T extends PriorityType> {
  private items: T[];

  constructor() {
    this.items = [];
  }

  get size(): number {
    return this.items.length;
  }

  get isEmpty(): boolean {
    return this.size === 0;
  }

  enqueue(item: T): void {
    this.items.push(item);
    this.items.sort((a, b) => a.priority - b.priority);
  }

  dequeue(): T | undefined {
    return !this.isEmpty ? this.items.shift() : undefined;
  }
}
