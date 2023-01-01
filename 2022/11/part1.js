#!/bin/node

const fs = require('fs');
const dat = fs.readFileSync('input', 'utf-8');
let arr = Array.from(dat.split(/\n/)).filter(r => r);


class Monkey {
	constructor(dat) {
		this.items = []
		this.op_func = undefined
		this.op_val = undefined
		this.test = null
		this.throw_to_true = undefined
		this.throw_to_false = undefined
		this.inspections = 0
		let s, m
		s = dat[0].split(':')
		s[1].split(',').forEach(n => this.items.push(parseInt(n)))
		s = dat[0].split(':')
		m = dat[1].match(/new = old (.+) (.+)/)	
		this.op_func = m[1]
		this.op_val = m[2]
		m = dat[2].match(/divisible by (\d+)/)
		this.test = parseInt(m[1])
		m = dat[3].match(/throw to monkey (\d+)/)
		this.throw_to_true = parseInt(m[1])
		m = dat[4].match(/throw to monkey (\d+)/)
		this.throw_to_false = parseInt(m[1])
	}

	do() {
		let res = [], item
		while (item = this.items.shift()) {
			this.inspections += 1
			let level = item
			switch (this.op_func) {
				case '+':
					if (this.op_val === 'old')
						level += item
					else
						level += parseInt(this.op_val)
					break
				case '*':
					if (this.op_val === 'old')
						level *= item
					else
						level *= parseInt(this.op_val)
					break
				default:
					throw(`Unknown function ${this.op_func}!`)
			}
			level = Math.trunc(level / 3)
			if ((level % this.test) === 0)
				res.push([level, this.throw_to_true])
			else
				res.push([level, this.throw_to_false])
		}
		return res
	}
}


// read monkey data
let monkeys = []
let tmp = [], first = true
for (let r of arr) {
	if (r.match(/^Monkey/)) {
		if (!first)
			monkeys.push(new Monkey(tmp))
		first = false
		tmp = []
	} else {
		tmp.push(r)
	}
}
if (tmp.length > 0)
	monkeys.push(new Monkey(tmp))

// do monkey business
let res
for (let k = 0; k < 20; k++) {
	for (let m of monkeys) {
		res = m.do()
		for (let r of res) 
			monkeys[r[1]].items.push(r[0])
	}
}
monkeys.sort((a, b) => b.inspections - a.inspections)
console.log(monkeys[0].inspections * monkeys[1].inspections)

