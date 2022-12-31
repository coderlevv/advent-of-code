#!/bin/node

const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/)).filter(r => r)


function is_visible(x, y, xdim, ydim) {
	if (x === 0 || y === 0) return true
	if (x === xdim-1 || y === ydim-1) return true
	let h = grid[y][x]
	let k = 1
	let up=true, down=true, left=true, right=true
	for (let k=1; y-k >= 0; k++)
		if (grid[y-k][x] >= h) up = false
	for (let k=1; y+k < ydim; k++)
		if (grid[y+k][x] >= h) down = false
	for (let k=1; x-k >= 0; k++)
		if (grid[y][x-k] >= h) left = false
	for (let k=1; x+k < xdim; k++)
		if (grid[y][x+k] >= h) right = false
	return up || down || left || right
}


let grid = []
for (let r of arr)
	grid.push([...r].map(t => parseInt(t)))

let ydim = grid.length
let xdim = grid[0].length

let count = 0
for (let y=0; y < ydim; y++) {
	for (let x=0; x < xdim; x++) {
		if (is_visible(x,y,xdim,ydim)) count++
	}
}

console.log(count)

