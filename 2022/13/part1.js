const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/))


// Recursively compare terms
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

let right_order_idx = []
let tmp = []
let pair_idx = 1
for (let r of arr) {
	if (r.trim() === '') {
		if (right_order(tmp[0], tmp[1])) right_order_idx.push(pair_idx)
		pair_idx += 1
		tmp = []
	} else {
		tmp.push(JSON.parse(r))
	}
}
console.log(right_order_idx.reduce((a,b) => a+b))
