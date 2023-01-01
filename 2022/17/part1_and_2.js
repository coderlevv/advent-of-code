#!/bin/node

const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/)).filter(r => r)


// Base class to represent a shape with
// extent in x- and y-dim, initial spacing
// between shape and top of stacked shapes
// and starting y- and x-coordinates
class Shape {
	constructor(y_dim, x_dim, space=3) {
		this.y_dim = y_dim
		this.x_dim = x_dim
		this.space = 3
		this.x = 2
		this.y = 0
	}

	init() {
		let arr = Array(this.space + this.y_dim).fill().map(()=>Array(7).fill(0))
		return arr
	}
}


// Specific deinition of each shape
// The pixel array outlines the shape pixels
class S1 extends Shape {
	constructor() {
		super(1, 4)
		// coords are y, x
		this.pixel = [
			[0,0], [0,1], [0,2], [0,3]
		]
	}
}

class S2 extends Shape {
	constructor() {
		super(3, 3)
		// coords are y, x
		this.pixel = [
			[0,1], [1,0], [1,1], [1,2], [2,1]
		]
	}
}

class S3 extends Shape {
	constructor() {
		super(3, 3)
		// coords are y, x
		this.pixel = [
			[0,2], [1,2], [2,2], [2,1], [2,0]
		]
	}
}

class S4 extends Shape {
	constructor() {
		super(4, 1)
		// coords are y, x
		this.pixel = [
			[0,0], [1,0], [2,0], [3,0]
		]
	}
}

class S5 extends Shape {
	constructor() {
		super(2, 2)
		// coords are y, x
		this.pixel = [
			[0,0], [0,1], [1,0], [1,1]
		]
	}
}


class Game {

	// static method to stack 2 grids on top of
	// each other
	static vstack(a1, a2) {
		let a1_len = a1.length
		let a2_len = a2.length
		if (a1_len === 0) return a2
		if (a2_len === 0) return a1
		let new_arr = Array(a1_len + a2_len).fill().map(() => Array(7))
		for (let i = 0; i < a1_len; i++)
			for (let j = 0; j < 7; j++)
				new_arr[i][j] = a1[i][j]
		for (let i = 0; i < a2_len; i++)
			for (let j = 0; j < 7; j++)
				new_arr[a1_len+i][j] = a2[i][j]
		return new_arr
	}

	// static method to clip empty line from the top of
	// the stacked shape grid
	static clip(grid) {
		return grid.filter(r => r.reduce((a, b) => a + b) > 0)
	}

	constructor(jet) {
		this.jet = [...jet]
		this.gush_count = 0
		this.round_num = 0
		this.grid = []
		this.next_shape = 0
		this.shapes = [
			new S1(),
			new S2(),
			new S3(),
			new S4(),
			new S5()
		]
		this.prepare_grid(this.shapes[0])
	}

	next_gush() {
		return this.jet[this.gush_count++ % this.jet.length]
	}

	// collision detection
	has_coll(shape, y_dim=true, off=1) {
		let coll = false
		if (y_dim) {
			if ((shape.y + shape.y_dim + off) > this.grid.length)
				return true
			let pixel = shape.pixel
			for (let p of pixel) {
				let y = shape.y + off + p[0]
				let x = shape.x + p[1]
				if (this.grid[y][x] > 0) {
					coll = true
					break
				}
			}
		} else {
			if (off>0) {
				if ((shape.x + shape.x_dim + off) > 7)
					return true
			} else {
				if ((shape.x + off) < 0)
					return true
			}
			let pixel = shape.pixel
			for (let p of pixel) {
				let y = shape.y + p[0]
				let x = shape.x + off + p[1]
				if (this.grid[y][x] > 0) {
					coll = true
					break
				}
			}
		}
		return coll
	}

	// Once the final postion of the shape is reached
	// it is added to the grid
	settle(shape) {
		let pixel = shape.pixel
		for (let i = 0; i < pixel.length; i++) {
			let y = shape.y + pixel[i][0]
			let x = shape.x + pixel[i][1]
			this.grid[y][x] = 1
		}
	}

	draw() {
		let c
		let row
		for (let i=0; i<this.grid.length; i++) {
			row = []
			for (let j=0; j<this.grid[0].length; j++) {
				if (this.grid[i][j] > 0) c = '#'
				else c = '.'
				row.push(c)
			}
			console.log(row.join(''))
		}
		console.log()
	}

	prepare_grid(shape) {
		this.grid = Game.vstack(shape.init(), this.grid)
	}

	// Each round consists of picking the next shape,
	// and let the shape fall until it is blocked, while
	// applying a new jet gush on the way down
	run_round() {
		let shape = this.shapes[this.next_shape]
		shape.y = 0
		shape.x = 2
		let coll = false
		let gush
		while (!coll) {
			gush = this.next_gush()
			switch (gush) {
				case '>':
					if (!this.has_coll(shape, false, 1))
						shape.x += 1
					break
				case '<':
					if (!this.has_coll(shape, false, -1))
						shape.x -= 1
					break
				default:
					throw("Unknown gush!")
			}
			coll = this.has_coll(shape, true, 1)
			if (!coll) shape.y += 1
			if (coll)
				this.settle(shape)
		}
		this.grid = Game.clip(this.grid)
		this.round_num += 1
		this.next_shape = this.round_num % this.shapes.length
		this.prepare_grid(this.shapes[this.next_shape])
	}
}

let game = new Game(arr[0])
for (let i = 0; i < 2022; i++) {
	game.run_round()
}
console.log(Game.clip(game.grid).length)


// To solve part 2 I looked at the increase of the stacked shapes
// over a few thousand rounds. After a while the pattern of
// increases repeats in a constant cycle.
//
// I determined the height after the initial period and for each cycle.
// The answer to part 2 is the sum of these plus the height of the remaining
// rounds to complete 1000000000000 round.
