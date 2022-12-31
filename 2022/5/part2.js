#!/bin/node

const fs = require('fs')

class Move{
	constructor(m) {
		let [, num, from, to] = m.match(/move (\d+) from (\d+) to (\d+)/i)
		this.num = parseInt(num)
		this.from = parseInt(from)
		this.to = parseInt(to)
	}
}

class Dock {
	constructor() {
		const dat = fs.readFileSync('input', 'utf-8')
		let arr = Array.from(dat.split(/\n/))
		this.arr = arr
		this.stacks = []
		this.moves = []
		let div = this.init_stacks()
		this.init_moves(div)
	}
	
	init_stacks(d=0) {
		/* reads input array recursively until the
		 * dividing empty line is found. On rewinding the
		 * crate letters are stored in the respective stacks.
		 * The function returns the line number corresponding
		 * to the dividing empty line
		 */
		if (this.arr[d] === '') return d
		let div = this.init_stacks(d+1)
		let r = this.arr[d]
		for (let i=0; i < r.length; i++) {
			let c = r.codePointAt(i)
			if (c >= 65 && c <= 90) {
				// Each input line with stack data can have a
				// letter at specific positions. These can be
				// used to find the corresponding stack
				let idx = parseInt((i-1)/4)
				if (this.stacks[idx] === undefined)
					this.stacks[idx] = []
				this.stacks[idx].push(r[i])
			}
		}
		return div
	}

	init_moves(div) {
		// reads the move instructions from the input array.
		// div is the line number of the empty line separating
		// the instructions from the stack outline above it
		for (let m of this.arr.slice(div+1)) {
			if (m.length > 0)
				this.moves.push(new Move(m))
		}
	}

	run() {
		// runs the instructions
		for (let m of this.moves) {
			// in part 2, multiple crates can now be moved at once
			// without changinge the order
			let crates = this.stacks[m.from-1].splice(-m.num)
			this.stacks[m.to-1] = this.stacks[m.to-1].concat(crates)
		}
	}
}	

dock = new Dock()
dock.run()
let word = []
for (let s of dock.stacks) {
	word.push(s[s.length-1])
}
console.log(word.join(''))
