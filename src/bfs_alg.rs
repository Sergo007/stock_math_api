use std::collections::{HashSet, VecDeque};
use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
pub struct SearchPath<N>(N, Option<Rc<SearchPath<N>>>);

impl<N> SearchPath<N> {
    fn to_list(&self) -> Vec<N>
    where
        N: Copy + Eq + Hash,
    {
        let mut list = Vec::new();
        let mut path = &self.1;
        list.push(self.0);
        while !path.is_none() {
            list.push(path.clone().unwrap().0);
            path = &path.as_ref().unwrap().1
        }
        return list.iter().rev().cloned().collect();
    }
}

impl<N: Debug> Debug for SearchPath<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.1 {
            Some(v) => write!(f, "{:?} -> {:?}\n", v, &self.0),
            None => write!(f, "{:?}\n", &self.0),
        }
    }
}

fn _bfs<N, F, R>(start: N, end: N, expand: F) -> Option<SearchPath<N>>
where
    N: Copy + Eq + Hash,
    F: Fn(N) -> R,
    R: IntoIterator<Item = N>,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(SearchPath(start, None));
    visited.insert(start);

    while let Some(SearchPath(node, path)) = queue.pop_front() {
        if node == end {
            return Some(SearchPath(node, path));
        }

        let path = Rc::new(SearchPath(node, path.clone()));

        for edge in expand(node) {
            if !visited.contains(&edge) {
                visited.insert(edge);
                queue.push_back(SearchPath(edge, Some(path.clone())));
            }
        }
    }

    None
}

pub fn bfs<T, F>(
    data: &Vec<Vec<T>>,
    start: (usize, usize),
    end: (usize, usize),
    is_barricade: F,
) -> Option<Vec<(usize, usize)>>
where
    T: Copy + Eq + Hash,
    F: Fn(T) -> bool,
{
    let path = _bfs(start, end, |(y, x)| {
        let mut edges = Vec::new();

        // Iterate over adjacent offsets
        for (dy, dx) in &[
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            // (1, 1),
            // (1, -1),
            // (-1, -1),
            // (-1, 1),
        ] {
            let edge_y = (y as isize + dy) as usize;
            let edge_x = (x as isize + dx) as usize;

            // Check if offset is within bounds
            if edge_y >= data.len() || edge_x >= data[edge_y].len() {
                continue;
            }

            // Check if offset points to valid location
            if is_barricade(data[edge_y][edge_x]) {
                continue;
            }

            edges.push((edge_y, edge_x));
        }
        edges
    });
    match path {
        Some(v) => Some(v.to_list()),
        None => None,
    }
}
