#!/bin/node

const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/)).filter(r => r)


function scenic_score(x, y, xdim, ydim) {
	if (x === 0 || x === xdim-1) return 0
	if (y === 0 || y === ydim-1) return 0
	let h = grid[y][x]
	let up = 0, down = 0, left = 0, right = 0
	let s
	for (let k=1; y-k >= 0; k++) {
		s = grid[y-k][x]
		if (s <= h) up++
		if (k>1 && s > h) up++
		if (s >= h) break
	}
	for (let k=1; y+k < ydim; k++) {
		s = grid[y+k][x]
		if (s <= h) down++
		if (k>1 && s > h) down++
		if (s >= h) break
	}
	for (let k=1; x-k >= 0; k++) {
		s = grid[y][x-k]
		if (s <= h) left++
		if (k>1 && s > h) left++
		if (s >= h) break
	}
	for (let k=1; x+k < xdim; k++) {
		s = grid[y][x+k]
		if (s <= h) right++
		if (k>1 && s > h) right++
		if (s >= h) break
	}
	return up * down * left * right
}

let grid = []
for (let r of arr)
	grid.push([...r].map(t => parseInt(t)))

let ydim = grid.length
let xdim = grid[0].length

let max_score = 0
let score
for (let y=0; y < ydim; y++) {
	for (let x=0; x < xdim; x++) {
		score = scenic_score(x, y, xdim, ydim)
		if (score > max_score) max_score = score
	}
}

console.log(max_score)

