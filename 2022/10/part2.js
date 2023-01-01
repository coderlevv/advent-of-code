const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/)).filter(r => r)


class Crt {
	constructor() {
		this.screen = new Array(6).fill(0)
			.map(() => new Array(40).fill(0))
	}

	draw(cycle, X) {
		let c
		let row = Math.trunc((cycle - 1) / 40)
		let col = (cycle-1) % 40
		if (col === X-1 || col === X || col === X+1)
			this.screen[row][col] = 1
	}

	flush() {
		let c
		for (let y=0; y < 6; y++) {
			for (let x=0; x < 40; x++) {
				if (this.screen[y][x] === 1) c = '#'
				else c = '.'
				process.stdout.write(c)
			}
			process.stdout.write('\n')
		}
	}
}


class Cpu {
	constructor() {
		this.crt = new Crt()
		this.reset()
	}

	reset() {
		this.X = 1
		this.cycle = 0
	}

	tick() {
		this.cycle += 1
		this.crt.draw(this.cycle, this.X)
	}

	step(s) {
		let [op, val] = s.split(/\s/)
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

	display() {
		this.crt.flush()
	}

	run(prg) {
		for (let s of prg) {
			this.step(s)
		}
	}
}

cpu = new Cpu()
cpu.run(arr)
cpu.display()
