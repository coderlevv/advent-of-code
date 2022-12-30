#!/bin/node
const fs = require('fs')

const input = fs.readFileSync('input', 'utf-8')
let s = 0, max = 0
let arr = []
for (let n of input.split(/\n/)) {
	n = parseInt(n)
	if (n) {
		s += n
	} else {
		if (s > max) {
			max = s
		}
		// store elf sums in array
		arr.push(s)
		s = 0
	}
}
// sort elf sums in decreasing order
arr.sort((a, b) => a - b).reverse()
console.log(arr[0] + arr[1] + arr[2])
