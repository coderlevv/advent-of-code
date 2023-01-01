import numpy as np
import re

with open("input", "r") as f:
    content = f.read()

class Monkey:
    def __init__(self, op1="", op2="", func="", number=""):
        self.op1 = op1
        self.op2 = op2
        self.func = func
        self.number = number

    def __repr__(self):
        return ",".join([self.op1, self.op2, self.func, self.number])

monkeys = dict()
for c in content.split("\n"):
    if c.strip() != "":
        spl = c.split(":")
        name = spl[0].strip()
        func = re.findall(f"[+|\-|*|/]", spl[1])
        if func:
            func = func[0]
            tmp = re.split(f"[+|\-|*|/]", spl[1])
            op1 = tmp[0].strip()
            op2 = tmp[1].strip()
            monkeys[name] = Monkey(op1, op2, func)
        else:
            monkeys[name] = Monkey(number=spl[1].strip())

def compute(name):
    if monkeys[name].number != "":
        return int(monkeys[name].number)
    op1 = compute(monkeys[name].op1)
    op2 = compute(monkeys[name].op2)
    func = monkeys[name].func
    if func == "+":
        return op1 + op2
    elif func == "-":
        return op1 - op2
    elif func == "*":
        return op1 * op2
    else:
        return op1 / op2

print(compute("root"))
