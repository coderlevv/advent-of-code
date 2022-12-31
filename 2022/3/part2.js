#!/bin/node
const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/), r => r)

// count each item in a compartment
function counts(s) {
	let m = new Map()
	for (let c of s) {
		if (!m.has(c)) {
			m.set(c, 1)
		} else {
			m.set(c, m.get(c) + 1)
		}
	}
	return m
}

function find_badge(grp) {
	// set of all items over the group of three
	let grp_set = new Set([...grp[0], ...grp[1], ...grp[2]])
	// count items in each rucksack
	let m1 = counts(grp[0])
	let m2 = counts(grp[1])
	let m3 = counts(grp[2])
	let code
	for (let c of grp_set) {
		// a badges is included in each rucksack
		let found = ([...m1.keys()].includes(c)) &&
						([...m2.keys()].includes(c)) &&
						([...m3.keys()].includes(c))
		if (found) {
			code = c.charCodeAt(0)
			//console.log(c, code)
			break
		}
	}
	return code
}

let code, prio = []
let k = 0
let grp = []
for (let r of arr) {
	grp.push(r)
	// look at each group of three
	if (k % 3 === 2) {
		//console.log(grp)
		code = find_badge(grp)
		if (code > 90) prio.push(code - 97 + 1)
		else prio.push(code - 65 + 27)
		grp = []
	}
	k += 1
}
//console.log(prio)
console.log(prio.reduce((a, b)=> a + b))
