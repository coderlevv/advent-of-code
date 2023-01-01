#!/bin/node

const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/)).filter(r => r)

class Voxel {
	constructor(x, y, z) {
		this.x = x
		this.y = y
		this.z = z
	}
}

// parse voxel data from input
let voxels = []
for (let r of arr) {
	let [x, y, z] = r.split(/,/)
	voxels.push(new Voxel(parseInt(x), parseInt(y), parseInt(z)))
}

let adj = []
let v1, v2, row, dist
for (let i = 0; i < voxels.length; i++) {
	row = []
	for (let j = 0; j < voxels.length; j++) {
		v1 = voxels[i]
		v2 = voxels[j]
		dist = Math.abs(v1.x - v2.x) + Math.abs(v1.y - v2.y) + Math.abs(v1.z - v2.z)
		if (dist === 1)
			row.push(1)
		else
			row.push(0)
	}
	adj.push(row)
}

let free = voxels.length * 6 - adj.flat().reduce((a, b) => a + b)
console.log(free)
	
