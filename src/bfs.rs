use std::collections::{HashMap, HashSet, VecDeque};

// BFS search that tracks the direction D used to reach each node
// Returns a list of (node, directions) that was used to reach the goal
pub fn search<N, D, FN, FG, IT>(start: &N, neighbours: FN, goal: FG) -> Option<Vec<(N, D)>>
where
    // Nodes
    N: Clone + Eq + std::hash::Hash,
    // Direction of neighbour
    D: Copy,
    // Closure that returns neighbouring nodes and the direction to them
    FN: Fn(&N) -> IT,
    // Close that checks if the goal has been reached
    FG: Fn(&N) -> bool,
    // Iterator that FN is expected to return
    IT: IntoIterator<Item = (N, D)>,
{
    // search queue
    let mut queue: VecDeque<N> = VecDeque::new();
    queue.push_front(start.clone());

    // Map of visited nodes and the predecessor + direction that reached it
    let mut visited: HashMap<N, Option<(N, D)>> = HashMap::new();
    visited.insert(start.clone(), None);

    while let Some(node) = queue.pop_front() {
        if goal(&node) {
            let mut path = vec![];
            let mut pos = node;
            while let Some(Some(step)) = visited.get(&pos) {
                path.push((pos.clone(), step.1));
                pos = step.0.clone();
            }
            path.reverse();
            return Some(path);
        }

        for (neighbour, dir) in neighbours(&node) {
            if !visited.contains_key(&neighbour) {
                visited.insert(neighbour.clone(), Some((node.clone(), dir)));
                queue.push_back(neighbour);
            }
        }
    }

    None
}

pub fn traverse<N, D, FN, FC, IT>(start: &N, neighbours: FN, mut callback: FC)
where
    // Nodes
    N: Clone + Eq + std::hash::Hash,
    // Direction of neighbour
    D: Copy,
    // Closure that returns neighbouring nodes and the direction to them
    FN: Fn(&N) -> IT,
    // Closure that is called for each visited node, with depth
    FC: FnMut(&N, u32),
    // Iterator that FN is expected to return
    IT: IntoIterator<Item = (N, D)>,
{
    // search queue (Node, depth visited)
    let mut queue: VecDeque<(N, u32)> = VecDeque::new();
    queue.push_front((start.clone(), 0));

    // Set of visited nodes
    let mut visited: HashSet<N> = HashSet::new();
    callback(start, 0);
    visited.insert(start.clone());

    while let Some((node, depth)) = queue.pop_front() {
        for (neighbour, _dir) in neighbours(&node) {
            if !visited.contains(&neighbour) {
                callback(&neighbour, depth + 1);
                visited.insert(neighbour.clone());
                queue.push_back((neighbour, depth + 1));
            }
        }
    }
}

#[test]
fn test_search() {
    // Finds shortest route on a chessboard from (1,1) to (4,6) using knight moves
    let result: Vec<(_, char)> = search(
        &(1, 1),
        |&(x, y)| {
            [
                ((x + 1, y - 2), 'a'),
                ((x + 2, y - 1), 'b'),
                ((x + 2, y + 1), 'c'),
                ((x + 1, y + 2), 'd'),
                ((x - 1, y + 2), 'e'),
                ((x - 2, y + 1), 'f'),
                ((x - 2, y - 1), 'e'),
                ((x - 1, y - 2), 'f'),
            ]
            .into_iter()
            .filter(|((x, y), _d)| *x >= 1 && *x <= 8 && *y >= 1 && *y <= 8)
        },
        |&(x, y)| x == 4 && y == 6,
    )
    .unwrap();
    assert_eq!(4, result.len());
    assert_eq!(
        vec![((3, 2), 'c'), ((5, 3), 'c'), ((6, 5), 'd'), ((4, 6), 'f'),],
        result
    );
}

#[test]
fn test_traverse() {
    // Iterate over each square on a chessboar reachable by knight moves
    let mut maxdepth = 0;
    let mut count = 0;
    traverse(
        &(1, 1),
        |&(x, y)| {
            [
                ((x + 1, y - 2), 'a'),
                ((x + 2, y - 1), 'b'),
                ((x + 2, y + 1), 'c'),
                ((x + 1, y + 2), 'd'),
                ((x - 1, y + 2), 'e'),
                ((x - 2, y + 1), 'f'),
                ((x - 2, y - 1), 'e'),
                ((x - 1, y - 2), 'f'),
            ]
            .into_iter()
            .filter(|((x, y), _d)| *x >= 1 && *x <= 8 && *y >= 1 && *y <= 8)
        },
        |_p, depth| {
            count += 1;
            maxdepth = depth;
        },
    );
    assert_eq!(6, maxdepth); // see https://chess.stackexchange.com/a/34589
    assert_eq!(64, count);
}
