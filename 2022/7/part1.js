#!/bin/node

const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let arr = Array.from(dat.split(/\n/))


class Entry {
	constructor(name, size=0, dir=true, parent=null) {
		this.parent = parent
		this.children = []
		this.name = name
		this.size = size
		this.dir = dir
	}

	has(name) {
		for (let c of this.children) 
			if (c.name === name) return true
		return false
	}

	get(name) {
		for (let c of this.children) 
			if (c.name === name) return c
		return null
	}
}


// build a tree of entries from the input
let root = new Entry("/")
let current_dir = root
for (let r of arr) {
	tok = r.split(/\s/)
	if (tok[0] === "$" && tok[1] === "cd") {
		if (current_dir.name === tok[2])
			continue
		if (tok[2] !== "..") {
			if (!current_dir.has(tok[2])) {
				let new_entry = new Entry(tok[2], 0, true, current_dir)
				current_dir.children.push(new_entry)
			}
			current_dir = current_dir.get(tok[2])
		} else {
			if (current_dir.parent)
				current_dir = current_dir.parent
		}
	}
	if (tok[0] === "dir") {
		let dir = new Entry(tok[1], 0, true, current_dir);	
		current_dir.children.push(dir)
	}
	if (tok[0].match(/^\d+$/)) {
		let file = new Entry(tok[1], parseInt(tok[0]), false, current_dir);	
		current_dir.children.push(file)
		current_dir.size += parseInt(tok[0])
	}
}



// do a recursive search over the tree
function search(d) {
	let size = 0
	for (let e of d.children)
		// follow dirs recursively
		if (e.dir) {
			size += search(e)
		}
		else {
			// file sizes can just be added
			size += e.size
		}
	// condition of part 1
	if (size <= 100000) total += size
	return size
}

let total = 0
search(root)
console.log(total)

