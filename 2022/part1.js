#!/bin/node
const fs=require('fs')

const input=fs.readFileSync('input', 'utf-8')
let s = 0, max = 0
for (let n of input.split(/\n/)) {
	// parse number
	n = parseInt(n);
	// if there is a number add to sum per elf	
	if (n) {
		s += n
	} else {
		// an empty line separates elf records
		// if current elf sum > max, store
		if (s > max) {
			max = s
		}
		// initialize sum for next elf
		s = 0
	}
}
console.log(max)
