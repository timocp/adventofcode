use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// Find the shortest path of the shortest path from start node to goal
// Returns the cost
pub fn shortest_path<N, FN, FG, IT>(start: &N, neighbours: FN, goal: FG) -> Option<u32>
where
    // Nodes
    N: Clone + Eq + std::hash::Hash + std::cmp::Ord,
    // Closure that returns neighbouring nodes and the cost to go there
    FN: Fn(&N) -> IT,
    // Closure that checks if the goal has been reached
    FG: Fn(&N) -> bool,
    // Iterator that FN is expected to return
    IT: IntoIterator<Item = (N, u32)>,
{
    let mut dist: HashMap<N, u32> = HashMap::new();
    let mut heap: BinaryHeap<State<N>> = BinaryHeap::new();

    dist.insert(start.clone(), 0);
    heap.push(State {
        cost: 0,
        node: start.clone(),
    });

    while let Some(State { cost, node }) = heap.pop() {
        if goal(&node) {
            return Some(cost);
        }
        if cost > *dist.get(&node).unwrap_or(&u32::MAX) {
            continue;
        }
        for (next_node, next_cost) in neighbours(&node) {
            let next = State {
                cost: cost + next_cost,
                node: next_node.clone(),
            };

            if next.cost < *dist.get(&next_node).unwrap_or(&u32::MAX) {
                dist.insert(next_node, next.cost);
                heap.push(next);
            }
        }
    }
    None
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<N>
where
    N: Eq,
{
    cost: u32,
    node: N,
}

impl<N> Ord for State<N>
where
    N: Eq + Ord,
{
    // other <=> cost so that the heap is minimum cost first
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<N> PartialOrd for State<N>
where
    N: Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
