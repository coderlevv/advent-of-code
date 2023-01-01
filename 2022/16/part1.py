import numpy as np
import re


class PriorityQueue:
    def __init__(self):
        self._items = []
        
    def queue(self, item, prio):
        self._items.append((prio, item))
        self._items = sorted(self._items, reverse=True)
        
    def dequeue(self):
        prio, item = self._items.pop()
        return item
    
    def is_empty(self):
        return len(self._items) == 0
    
    def clear(self):
        self._items = []


class Valve:
    def __init__(self, name, flow, leads_to):
        self.name = name
        self.flow = flow
        self.leads_to = leads_to
        self.open = False
        
    def __repr__(self):
        return f"{self.name}: open={self.open}, flow={self.flow} ->{self.leads_to}"


def find_path(start, end):
    """Finds shortest path between start and end.

    Parameters
    ----------
    start : string
        Two upper case letters representing a valve
    end : string
        Two upper case letters representing a valve
    
    Returns
    -------
    list of string
        list of valves representing the shortest path beween 
        start and end
    """

    frontier = PriorityQueue()
    frontier.queue(start, 0)
    path_chain = dict()
    steps = dict()
    path_chain[start] = None
    steps[start] = 0
    while not frontier.is_empty():
        curr_pos = frontier.dequeue()
        if curr_pos == end:
            break
        for nxt in valves[curr_pos].leads_to:
            new_steps = steps[curr_pos] + 1
            if nxt not in steps or new_steps < steps[nxt]:
                steps[nxt] = new_steps
                prio = new_steps
                frontier.queue(nxt, prio)
                path_chain[nxt] = curr_pos
    curr_pos = end
    path = []
    while curr_pos != start: 
        path.append(curr_pos)
        curr_pos = path_chain[curr_pos]
    return path[::-1]



def max_flow(t, name, cumul_flow, closed_with_flow):
    """Find order of visiting valves with max. pressure release."""

    if t >= 30 or len(closed_with_flow) == 0:
        return cumul_flow
    res = []
    # look at valves with flow which are still closed
    path_to = dict()
    for next_name in closed_with_flow:
        path_to[next_name] = find_path(name, next_name)
    for next_name in path_to.keys():
        path = path_to[next_name]
        next_valve = valves[next_name]
        #open_time = 1 if next_valve.flow > 0 else 0
        next_valve.open = True
        closed_with_flow.remove(next_name)
        time_inc = len(path) + 1 # steps + time to open
        flow_inc = max(0, (30 - (t + time_inc)) * next_valve.flow)
        res.append(max_flow(t+time_inc, next_name, cumul_flow+flow_inc, closed_with_flow))
        closed_with_flow.add(next_name)
        next_valve.open = False
    return max(res)


if __name__ == '__main__':
    with open("input" ,"r") as f:
        content = f.read()
    valves = dict()
    for c in content.split('\n'):
        if c.strip() != '':
            name = re.findall(r"Valve ([A-Z]{2})", c)[0]
            flow = int(re.findall(r"flow rate=(\d+)", c)[0])
            leads_to = [v.strip() for v in re.split(r"valves?", c)[1].split(",")]
            valves[name] = Valve(name, flow, leads_to)

    closed_with_flow = set()
    for k, v in valves.items():
        if not v.open and v.flow > 0:
            closed_with_flow.add(k)

    print(max_flow(0, "AA", 0, closed_with_flow))
