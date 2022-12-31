const fs = require('fs')
const dat = fs.readFileSync('input', 'utf-8')
let signal = dat.trim()

for (let i=4; i<signal.length; i++) {
	let win = signal.slice(i-4, i)
	if (new Set([...win]).size === 4) {
		console.log(i)
		break
	}
}

for (let i=14; i<signal.length; i++) {
	let win = signal.slice(i-14, i)
	if (new Set([...win]).size === 14) {
		console.log(i)
		break
	}
}
