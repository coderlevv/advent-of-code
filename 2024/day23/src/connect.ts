export function search(
  node: string,
  path: string[],
  connect: Map<string, Set<string>>,
  res: Set<string>,
) {
  const nextNodes = connect.get(node);
  if (path.length === 2 && nextNodes !== undefined) {
    let found = true;
    for (const p of path)
      if (!nextNodes.has(p)) {
        found = false;
        break;
      }
    if (found) res.add([node].concat(path).sort().join());
    return;
  }
  path.push(node);
  if (nextNodes !== undefined) {
    for (const next of nextNodes) {
      if (path.includes(next)) continue;
      search(next, path, connect, res);
    }
  }
  path.pop();
}

// Bron-Kerbosch Algorithm

// The graph is represented as an adjacency list,
// where each node maps to a set of connected nodes.
export type Graph = { [node: string]: Set<string> };

function bronKerbosch(
  graph: Graph,
  r: Set<string>, // Current clique
  p: Set<string>, // Potential nodes to expand the clique
  x: Set<string>, // Nodes already considered
  cliques: Set<string>[], // List of maximal cliques
): void {
  if (p.size === 0 && x.size === 0) {
    // Found a maximal clique
    cliques.push(new Set(r));
    return;
  }

  // Convert p to an array to iterate
  const pArray = Array.from(p);

  for (const node of pArray) {
    // Add node to the current clique
    const newR = new Set(r);
    newR.add(node);

    // Compute neighbors of the node
    const neighbors = graph[node] || new Set();

    // Update p and x for the next recursive step
    const newP = new Set(Array.from(p).filter((v) => neighbors.has(v)));
    const newX = new Set(Array.from(x).filter((v) => neighbors.has(v)));

    // Recursive call
    bronKerbosch(graph, newR, newP, newX, cliques);

    // Move node from p to x
    p.delete(node);
    x.add(node);
  }
}

// Wrapper function to find all maximal cliques
export function findMaximalCliques(graph: Graph): Set<string>[] {
  const cliques: Set<string>[] = [];
  const allNodes = new Set(Object.keys(graph));
  bronKerbosch(graph, new Set(), allNodes, new Set(), cliques);
  return cliques;
}
