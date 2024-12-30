//export type CircuitType = Map<string, Wire>;

export class Wire {
    _value: boolean|undefined;
    constructor() {
        this._value = undefined;
    }

    get value() {
        return this._value;
    }

    set value(val) {
        this._value = val;
    }
}

export class Gate {
    input: [string, string];
    output: string;
    op: string;
    isSet: boolean;
    
    constructor(input: [string, string], op: string, output: string) {
        this.input = input;
        this.op = op;
        this.output = output;
        this.isSet = false;
    }

    apply(circuit: Map<string, Wire>) {
        let input1 = circuit.get(this.input[0])?.value;
        let input2 = circuit.get(this.input[1])?.value;
        let outval = undefined;
        if (!this.isSet && input1 !== undefined && input2 !== undefined) {
            switch (this.op) {
                case "AND":
                    outval = input1 && input2;
                    break;
                case "OR":
                    outval = input1 || input2;
                    break;
                case "XOR":
                    outval = input1 !== input2 ? true : false;
                    break;
                default:
                    throw "Unknown operation!";
            }
            let outwire = circuit.get(this.output);
            if (outwire !== undefined) {
                outwire.value = outval;
                this.isSet = true;
            }
        }
    }
}

export class Circuit {
    wires: Map<string, Wire>;
    gates: Gate[];

    constructor(wires: Map<string, Wire>, gates: Gate[]) {
        this.wires = wires;
        this.gates = gates;
    }

    static from(lines: string[]): Circuit {
        let wires = new Map<string, Wire>();
        let gates: Gate[] = [];
        let initWires= true;
        for (const line of lines) {
            if (line.trim() == "") {
            initWires = false;
            continue;
            }
            if (initWires) {
                const [key, val] = line.split(":");
                let wire = new Wire();
                wire.value = val.trim() === "1" ? true : false;
                wires.set(key.trim(), wire);
                continue; 
            } else {
                const [in1, op, in2, , out] = line.split(" ");
                if (!wires.has(in1)) wires.set(in1, new Wire());
                if (!wires.has(in2)) wires.set(in2, new Wire());
                if (!wires.has(out)) wires.set(out, new Wire());
                let gate = new Gate([in1.trim(), in2.trim()], op, out);
                gates.push(gate);
            }
        }
        return new Circuit(wires, gates);
    }

    isReady = () => 
        [...this.wires.entries()]
            .filter(([w,]) => w.startsWith("z"))
            .every(([, w]) => w.value !== undefined);
    
    output = () => 
        parseInt([...this.wires.entries()]
            .filter(([w, ]) => w.startsWith("z"))
            .sort()
            .map(([, w]) => w.value ? "1" : "0")
            .reverse()
            .join(""), 2);

    apply() {
        while (!this.isReady()) {
            for (const gate of this.gates) {
              gate.apply(this.wires);
            }
        }
    }

    dim(): [number, number] {
        let xDim = [...this.wires.entries()]
            .filter(([w,]) => w.startsWith("x"))
            .length;
        let yDim = [...this.wires.entries()]
            .filter(([w,]) => w.startsWith("y"))
            .length;
        return([xDim, yDim]);
    }

    reset() {
       for (const wire of this.wires.values()) {
        wire.value = undefined;
       }
       for (const gate of this.gates) {
        gate.isSet = false;
       }
    }
    setInput(xStr: string, yStr: string) {
        this.reset();
        let [xDim, yDim] = this.dim();
        if (xStr.length !== xDim || yStr.length !== yDim) throw "Input dimension mismatch!";
        for (let x = xDim-1; x >= 0; x--) {
            let xPad = "x" + (xDim - x - 1).toString().padStart(xDim.toString().length, "0");
            let wire = this.wires.get(xPad);
            if (wire === undefined) throw `Missing input wire ${xPad}!`;
            wire.value = xStr[x] === "1" ? true : false;
            this.wires.set(xPad, wire);
        } 
        
        for (let y = yDim-1; y >= 0; y--) {
            let yPad = "y" + (yDim - y - 1).toString().padStart(yDim.toString().length, "0");
            let wire = this.wires.get(yPad);
            if (wire === undefined) throw `Missing input wire ${yPad}!`;
            wire.value = yStr[y] === "1" ? true : false;
            this.wires.set(yPad, wire);
        }
    }
}