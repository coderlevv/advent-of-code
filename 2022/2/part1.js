#!/bin/node
const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/), r => r).filter(r => r.length > 0)

// Score for each shape
let shape = new Map()
shape.set('X', 1)
shape.set('Y', 2)
shape.set('Z', 3)
shape.set('A', 1)
shape.set('B', 2)
shape.set('C', 3)

// Scores for the outcome of a round
// Rock, Paper, Scissors
let scoring = [
	[ 3, 0, 6 ], // Rock
	[ 6, 3, 0 ], // Paper
	[ 0, 6, 3 ]  // Scissors
]

let total = 0
let score
for (let g of arr) {
	score = 0
	// get shapes played in a round
	// first letter is elf, second is me
	let [p1, p2] = g.split(/\s/)
	score += shape.get(p2)
	score += scoring[shape.get(p2)-1][shape.get(p1)-1]
	total += score
}	
console.log(total)
