#!/bin/node

const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/)).filter(r => r)


class Pos {
	constructor(x=0, y=0) {
		this.x = x
		this.y = y
	}
}

class Bridge {
	constructor() {
		this.head = new Pos()
		this.tail = new Pos()
		this.tail_pos = []
		this.tail_pos.push(new Pos(0, 0))
	}
	
	delta() {
		let dx = this.tail.x - this.head.x
		let dy = this.tail.y - this.head.y
		return [dx, dy]
	}

	// move the head one step with direction given by d
	// and imediately move the second knot in it's
	// correct position if necessary
	move(d) {
		switch (d) {
			case 'U':
				this.head.y -= 1
				break
			case 'D':
				this.head.y +=1
				break
			case 'R':
				this.head.x += 1
				break
			case 'L':
				this.head.x -= 1
				break
			default:
				throw('Unknown direction')
		}
		
		// determine whether 2nd knot needs to be moved
		// and if so, move it to the right position
		let dx, dy
		[dx, dy] = this.delta()
		if (Math.abs(dx) >= 2 || Math.abs(dy) >= 2) {
			if (this.tail.x === this.head.x)
				if (dy < 0) this.tail.y += 1
				else this.tail.y -= 1
			if (this.tail.y === this.head.y)
				if (dx < 0) this.tail.x += 1
				else this.tail.x -= 1
			if (this.tail.x !== this.head.x && this.tail.y !== this.head.y) {
				if (dx > 0 && dy < 0) { this.tail.x -= 1; this.tail.y += 1 }
				if (dx < 0 && dy < 0) { this.tail.x += 1; this.tail.y += 1 }
				if (dx < 0 && dy > 0) { this.tail.x += 1; this.tail.y -= 1 }
				if (dx > 0 && dy > 0) { this.tail.x -= 1; this.tail.y -= 1 }
			}
			this.tail_pos.push(new Pos(this.tail.x, this.tail.y))
		}
	}

	// count visited tail positions
	count_tail_pos(display = false) {
		let x_pos = [], y_pos = []
		for (let p of this.tail_pos) {
			x_pos.push(p.x)
			y_pos.push(p.y)
		}
		let x_min = Math.min(...x_pos)
		let x_max = Math.max(...x_pos)
		let y_min = Math.min(...y_pos)
		let y_max = Math.max(...y_pos)
		let c, count = 0
		for (let y = y_min; y <= y_max; y++) {
			for (let x = x_min; x <= x_max; x++) {
				let c = '.'
				for (let p of this.tail_pos) {
					if (p.x === x && p.y === y) {
						c = '#'
						count += 1
						break
					}
				}
				if (display) process.stdout.write(c)
			}
			if (display) process.stdout.write('\n')
		}
		return count
	}
}

b = new Bridge()
for (let m of arr) {
	let [d, s] = m.split(/\s/)
	for (let i = 0; i < parseInt(s); i++)
		b.move(d)
}
let count = b.count_tail_pos()
console.log(count)

