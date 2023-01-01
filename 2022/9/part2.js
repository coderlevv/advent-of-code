#!/bin/node

const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/)).filter(r => r)


// Each knot is models by a seg instance
// The knots form a chain with the first
// being the head knot
class Seg {
	constructor(x=0, y=0, next=null) {
		this.next_seg = next
		this.x = x
		this.y = y
	}

	get next() {
		return this.next_seg
	}

	set next(next) {
		this.next_seg = next
	}
}


class Bridge {
	constructor(n) {
		// A bridge with n knot segments
		// segs models the different knots with the first
		// entry being the head
		this.segs = []
		this.trace = [] // trace visited tail positions
		// initialize knots
		let s
		let p = new Seg()
		this.segs.push(p)
		while (--n > 0) {
			s = new Seg()
			this.segs.push(s)
			p.next = s
			p = s
		}
		// all knots including tail start at Pos 0,0
		this.trace.push(new Seg(0, 0, null))
	}
	
	// move the head one step with direction given by d
	move_head(d) {
		let head = this.segs[0]
		switch (d) {
			case 'U':
				head.y -= 1
				break
			case 'D':
				head.y += 1
				break
			case 'R':
				head.x += 1
				break
			case 'L':
				head.x -= 1
				break
			default:
				throw('Unknown direction')
		}
	}


	// determines if the chain of knots has a gap
	has_gap() {
		let k = 0
		let s1, s2, dx, dy, d
		for (let k = 0; k < this.segs.length-1; k++) {
			s1 = this.segs[k]
			s2 = this.segs[k+1]
			dx = s1.x - s2.x
			dy = s1.y - s2.y
			if (Math.abs(dx) >= 2 || Math.abs(dy) >= 2) return k
		}
		return -1
	}


	// if there is a gab between two knots, this
	// function moves the 2nd knot in the correct
	// new position
	catchup(k) {
		let s1 = this.segs[k]
		let s2 = this.segs[k+1]
		let dx = s2.x - s1.x
		let dy = s2.y - s1.y
		if (s2.x === s1.x)
			if (dy < 0) s2.y += 1
			else s2.y -= 1
		if (s2.y === s1.y)
			if (dx < 0) s2.x += 1
			else s2.x -= 1
		if (s2.x !== s1.x && s2.y !== s1.y) {
			if (dx > 0 && dy < 0) { s2.x -= 1; s2.y += 1 }
			if (dx < 0 && dy < 0) { s2.x += 1; s2.y += 1 }
			if (dx < 0 && dy > 0) { s2.x += 1; s2.y -= 1 }
			if (dx > 0 && dy > 0) { s2.x -= 1; s2.y -= 1 }
		}
	}


	// Move the whole chain of segments, i.e. knots.
	// First the head is moved, then the chain is checked
	// for gaps. If there is a gap between 2 knots it is
	// closed, which can create a new gap, which is closed
	// etc. Movement stops if there is no gap anymore
	move(m) {
		let [d, s] = m.split(/\s/)
		for (let i=0; i < parseInt(s); i++) {
			this.move_head(d)
			let last = false
			while (true) {
				let k = this.has_gap()
				if (k < 0) break
				this.catchup(k)
				if (k === this.segs.length-2) last = true
			}
			// if the last knot, i.e. tail was moved
			// trace it's new position
			if (last) {
				this.trace.push(new Seg(
					this.segs[this.segs.length-1].x,
					this.segs[this.segs.length-1].y, null))
			}
		}
	}


	// count visited tail positions
	count_tail_pos(display = false) {
		let x_pos = [], y_pos = []
		for (let p of this.trace) {
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
				let constains = false
				let c = '.'
				for (let p of this.trace) {
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

b = new Bridge(10)
for (let m of arr) {
	b.move(m)
}
let count = b.count_tail_pos()
console.log(count)

