#!/bin/node
const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/), r => r)
//console.log(arr)

function find_common(s1, s2) {
	let common = undefined
	for (let c of s1) {
		if (s2.includes(c)) {
			common = c
			break
		}
	}
	return common
}

let prio = []
for (let r of arr) {
	// split input in half to get content
	// of 1st and 2nd compartment
	let l = r.length / 2
	let s1 = r.slice(0, l)
	let s2 = r.slice(l)
	// which item is common to both compartments
	let common = find_common(s1, s2)
	let code
	if (common) {
		code = common.charCodeAt(0)
		if (code > 90) prio.push(code - 97 + 1)
		else prio.push(code - 65 + 27)
	} else {
		console.log(r, s1, s2)
	}
}
console.log(prio.reduce((a, b) =>  a + b))	
