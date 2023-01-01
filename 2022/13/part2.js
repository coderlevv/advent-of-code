#!/bin/node

const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/)).filter(r => r)

function right_order(t1, t2, k=0) {
	let res
	while (res === undefined) {
		if (typeof t1 === 'object' && typeof t2 === 'object') {
			if (k === t1.length && k < t2.length) return true
			if (k < t1.length && k === t2.length) return false
			if (k === t1.length && k === t2.length) return undefined
		}

		if (typeof t1[k] === 'number' && typeof t2[k] === 'number') {
			if (t1[k] < t2[k]) return true
			if (t1[k] > t2[k]) return false
		}
		else if (typeof t1[k] === 'number' && typeof t2[k] === 'object') {
			res = right_order([t1[k]], t2[k])
		}
		else if (typeof t1[k] === 'object' && typeof t2[k] === 'number') {
			res = right_order(t1[k], [t2[k]])
		}
		else {
			res = right_order(t1[k], t2[k])
		}
		k += 1
	}
	return res
}


function compareOrder(t1, t2) {
	if (right_order(JSON.parse(t1), JSON.parse(t2)))
		return -1
	else
		return 1
}


// insert the divider packets
arr.push('[[2]]')
arr.push('[[6]]')

arr.sort(compareOrder)

let key = 1
for (let [i, r] of arr.entries()) {
	if (r === '[[2]]' || r === '[[6]]')
		key *= (i+1)
}
console.log(key)

