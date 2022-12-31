#!/bin/node

const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/)).filter(r => r)

class Task {
	constructor(t) {
		let [s1, s2] = t.split('-')
		let a = parseInt(s1)
		let b = parseInt(s2)
		if (a <= b) {
			this.a = parseInt(a)
			this.b = parseInt(b)
		} else {
			this.a = parseInt(b)
			this.b = parseInt(a)
		}
	}
}

function contains(x, y) {
	if ((x.a >= y.a && x.b <= y.b) ||
		(y.a >= x.a && y.b <= x.b)) return true
	return false
}

function overlap(x, y) {
	// interval limits need to be sorted!
	// i.e. in x & y always a < b
	// taken care of in the Task constructor
	if ((x.a < y.a) && (x.b < y.a)) return false
	if (x.a > y.b) return false
	if ((y.a < x.a) && (y.b < x.a)) return false
	if (y.a > x.b) return false
	return true
}

let overlap_count = 0
let contains_count = 0
for (let r of arr) {
	let [x, y] = r.split(',')
	if (x && y) {
		let t1 = new Task(x)
		let t2 = new Task(y)
		if (contains(t1, t2)) contains_count += 1
		if (overlap(t1, t2)) overlap_count += 1
	}
}
console.log(contains_count)
console.log(overlap_count)

