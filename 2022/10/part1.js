const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/)).filter(r => r)


class Cpu {
	// watch is suppose to be an array of cycle numbers
	// at which the content of the X register is checked
	// and stored in the out array
	constructor(watch) {
		this.watch = watch
		this.out = []
		this.reset()
	}

	reset() {
		this.X = 1
		this.cycle = 0
	}

	tick(display = false) {
		this.cycle += 1
		if (this.watch.includes(this.cycle)) {
			if (display)
				console.log(`out=${this.X}`)
			this.out.push(this.X)
		}
	}

	step(s) {
		let [op, val] = s.split(/\s/)
		//console.log(`X=${this.X} cycle=${this.cycle}: ${op} ${val}`)
		switch (op) {
			case "noop":
				this.tick()
				break
			case "addx":
				this.tick()
				this.tick()
				this.X += parseInt(val)
				break
			default:
				throw("Unknown operation!")
		}
	}

	run(prg) {
		for (let s of prg) {
			this.step(s)
		}
	}
}

let watchlist = [20, 60, 100, 140, 180, 220]
cpu = new Cpu(watchlist)
cpu.run(arr)
let sum = 0
for (let i = 0; i < watchlist.length; i++) {
	sum += watchlist[i] * cpu.out[i]
}
console.log(sum)

