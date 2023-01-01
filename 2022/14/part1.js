#!/bin/node

const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/)).filter(r => r)


// defines coordinates of the line segments
// a path of rocks consists of
class Seg {
	constructor(y0, x0, y1, x1) {
		this.y0 = y0
		this.x0 = x0
		this.y1 = y1
		this.x1 = x1
	}
}


// The cave with source position
// line segments of rocks read from input,
// x-dim range and maximal y ccordinate (below this
// there is only void)
class Cave {
	constructor(segs, y_max, x_min, x_max) {
		this.y_max = y_max
		this.x_min = x_min
		this.x_max = x_max
		this.source_x = 500 - x_min
		this.count = 0
		// has a unit of sand ended up in the void yet?
		this.void = false 
		this.grid = []
		this.segs = segs
		this.init_grid()
	}

	// Given the cave data, construct a grid
	// 0 = empty
	// 1 = rock
	// The simulation then uses 2 to represent sand
	init_grid() {
		let x_range = (this.x_max) - (this.x_min)
		let y_range = this.y_max + 1
		for (let y = 0; y < y_range; y++)
			this.grid.push(Array(x_range + 1).fill(0))
		for (let s of this.segs) {
			if (s.y0 === s.y1) {
				let x0 = s.x0 < s.x1 ? s.x0 : s.x1
				let x1 = s.x1 > s.x0 ? s.x1 : s.x0
				x0 = x0 - this.x_min
				x1 = x1 - this.x_min
				x_range = x1 - x0
				for (let x = x0; x < x1+1; x++)
					this.grid[s.y0][x] = 1
			}
			if (s.x0 === s.x1) {
				let y0 = s.y0 < s.y1 ? s.y0 : s.y1
				let y1 = s.y1 > s.y0 ? s.y1 : s.y0
				y_range = y1 - y0
				for (let y = y0; y < y1+1; y++)
					this.grid[y][s.x0-this.x_min] = 1
			}
		}
	}

	draw() {
		let c
		for (let y = 0; y < this.grid.length; y ++) {
			for (let x = 0; x < this.grid[0].length; x++) {
				if (this.grid[y][x] === 0) c = '.'
				if (this.grid[y][x] === 1) c = '#'
				if (this.grid[y][x] === 2) c = 'o'
				if (y === 0 && x === this.source_x) c = '+'
				process.stdout.write(c)
			}
			process.stdout.write('\n')
		}
	}


	// drop a unit of sand starting at the source
	drop() {
		let x = this.source_x
		let y = 0
		let c
		while (true) {
			y += 1
			// if void, stop
			if (y > this.y_max) break
			// look at the different possible places in turn
			c = this.grid[y][x]
			// is straigt down blocked?
			if (c > 0) {
				// if yes, is down-left blocked?
				x -= 1
				c = this.grid[y][x]
				if (c > 0) {
					// if yes, is down-right blocked?
					x += 2
					c = this.grid[y][x]
					// if all is blocked, stop	
					if (c > 0) break
				}
			}
		}
		if (y <= this.y_max) {
			// 2 indicates a unit of sand in the grid position
			this.grid[y-1][x-1] = 2
			this.count += 1
		} else {
			this.void = true
		}
	}
}


// parse segments of rock from input
let segs = []
let y0, x0, y1, x1
let y_min=0, y_max, x_min, x_max
for (let r of arr) {
	let spl = r.split(/->/)
	for (let k = 0; k < spl.length-1; k++) {
		let p1 = spl[k].split(/,/).map(x => parseInt(x))
		let p2 = spl[k+1].split(/,/).map(x => parseInt(x))
		segs.push(new Seg(p1[1], p1[0], p2[1], p2[0]))
		if (y_max === undefined) y_max = p1[1] > p2[1] ? p1[1] : p2[1]
		if (x_min === undefined) x_min = p1[0] < p2[0] ? p1[0] : p2[0]
		if (x_max === undefined) x_max = p1[0] > p2[0] ? p1[0] : p2[0]
		if (p1[0] < y_min) y_min = p1[0]
		if (p1[0] < x_min) x_min = p1[0]
		if (p2[0] < x_min) x_min = p2[0]
		if (p1[1] > y_max) y_max = p1[1]
		if (p2[1] > y_max) y_max = p2[1]
		if (p1[0] > x_max) x_max = p1[0]
		if (p2[0] > x_max) x_max = p2[0]
	}
}

let cave = new Cave(segs, y_max, x_min, x_max, segs)
while(true) {
	cave.drop()
	if (cave.void)
		break
}
console.log(cave.count)

