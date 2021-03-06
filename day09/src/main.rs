use std::mem;

/// A circular linked list implemented by https://github.com/EdeMeijer/
struct CircularList<T> {
    nodes: Vec<Node<T>>,
    cursor: Option<usize>,
}

/// A node in the circular linked list
struct Node<T> {
    value: Option<T>,
    prev: usize,
    next: usize,
}

impl<T> CircularList<T> {
    fn new() -> CircularList<T> {
        CircularList { nodes: Vec::new(), cursor: None }
    }

    fn with_capacity(capacity: usize) -> CircularList<T> {
        CircularList {
            nodes: Vec::with_capacity(capacity),
            cursor: None,
        }
    }

    /// Insert a value after the current value. This value becomes the new current value.
    fn insert(&mut self, value: T) {
        let i = self.nodes.len();

        let (prev, next) = match self.cursor {
            Some(cur) => {
                // We will insert our value after the current value. To do this, we must let the
                // current node point to the new one as next. Also, the node that we will be
                // replacing as next one has to point to the new one as previous.
                let next_i = self.nodes[cur].next;
                self.nodes[next_i].prev = i;
                self.nodes[cur].next = i;
                (cur, next_i)
            }
            None => {
                // There is no current value because the list is empty. This value will become
                // the current one, and point to itself in both directions.
                (i, i)
            }
        };

        self.nodes.push(Node { value: Some(value), prev, next });
        self.cursor = Some(i);
    }

    /// Remove the current value, and make the next value the current one
    /// Returns the removed value
    /// If no current value is present, returns None
    fn remove(&mut self) -> Option<T> {
        self.cursor.map(|cur| {
            let mut removed = Node { value: None, prev: 0, next: 0 };
            mem::swap(&mut removed, &mut self.nodes[cur]);

            // If this node points to itself, which means that it's the only one and after this the
            // list will be empty, we just remove the cursor
            self.cursor = if removed.next == cur {
                None
            } else {
                // Let the previous and next nodes point to one another
                self.nodes[removed.next].prev = removed.prev;
                self.nodes[removed.prev].next = removed.next;
                Some(removed.next)
            };

            removed.value.unwrap()
        })
    }

    /// Traverse the list clockwise (positive value) or counter-clockwise (negative value) for a
    /// number of steps
    fn seek(&mut self, offset: isize) -> &mut Self {
        for _ in 0..offset {
            self.next();
        }
        for _ in offset..0 {
            self.prev();
        }
        self
    }

    /// Make the next value the current one
    fn next(&mut self) -> &mut Self {
        self.cursor = self.cursor.map(|i| self.nodes[i].next);
        self
    }

    /// Make the previous value the current one
    fn prev(&mut self) -> &mut Self {
        self.cursor = self.cursor.map(|i| self.nodes[i].prev);
        self
    }

    /// Get the current value, if any is present, otherwise None
    fn get(&self) -> Option<&T> {
        match self.cursor {
            Some(cur) => {
                match &self.nodes[cur].value {
                    Some(v) => Some(v),
                    None => None
                }
            }
            None => None
        }
    }
}

fn solve(players: usize, max_marble: usize) -> usize {
    let mut scores = vec![0usize; players];

    let mut circle = CircularList::with_capacity(max_marble + 1);
    circle.insert(0usize);

    for marble in 1..=max_marble {
        if marble % 23 == 0 {
            scores[marble % players] += circle.seek(-7).remove().unwrap() + marble;
        } else {
            circle.next().insert(marble);
        }
    }

    scores.into_iter().max().unwrap()
}

fn solve_part_1() -> usize {
    solve(405,71700)
}

fn solve_part_2() -> usize {
    solve(405,7170000)
}

fn main() {
    println!("{}", solve_part_1());
    println!("{}", solve_part_2());
}
