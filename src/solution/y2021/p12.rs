use std::collections::HashMap;

use crate::solution::util::graph;

type Graph = graph::Graph<String, String>;
pub fn small(a: Vec<String>) -> i32 {
    let g = graph(a);
    dfs(&g, "start", &mut HashMap::new(), true)
}

pub fn large(a: Vec<String>) -> i32 {
    let g = graph(a);
    dfs(&g, "start", &mut HashMap::new(), false)
}

fn graph(a: Vec<String>) -> Graph {
    let mut graph = Graph::new();
    for x in a {
        let mut ss = x.split('-').map(ToOwned::to_owned);
        graph.insert_both(ss.next().unwrap(), ss.next().unwrap());
    }
    graph
}

fn dfs(graph: &Graph, cur: &str, visited: &mut HashMap<String, i32>, mut twice: bool) -> i32 {
    if cur == "end" {
        return 1;
    }
    let b = cur.as_bytes()[0];
    if (b'a'..=b'z').contains(&b) && *visited.get(cur).unwrap_or(&0) > 0 {
        if cur == "start" || twice {
            return 0;
        }
        twice = true;
    }
    *visited.entry(cur.to_string()).or_default() += 1;
    let mut res = 0;
    for nxt in graph.edges(cur).unwrap() {
        res += dfs(graph, nxt, visited, twice);
    }
    *visited.entry(cur.to_string()).or_default() -= 1;
    res
}

#[cfg(test)]
mod tests {
    use crate::solution::solve;

    const INPUT: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
    #[test]
    fn small() {
        assert_eq!(solve(INPUT, 2021, 12, false).unwrap(), "10");
    }
    #[test]
    fn large() {
        assert_eq!(solve(INPUT, 2021, 12, true).unwrap(), "36");
    }
}
