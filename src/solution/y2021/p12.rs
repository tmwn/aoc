use std::collections::HashMap;

use aocio::aocio;

use crate::solution::util::graph;

type Graph<'a> = graph::Graph<&'a str, &'a str>;

#[aocio]
pub fn small(a: Vec<String>) -> i32 {
    let g = graph(&a);
    dfs(&g, "start", &mut HashMap::new(), true)
}

#[aocio]
pub fn large(a: Vec<String>) -> i32 {
    let g = graph(&a);
    dfs(&g, "start", &mut HashMap::new(), false)
}

fn graph(a: &[String]) -> Graph<'_> {
    let mut graph = Graph::new();
    for x in a {
        let mut ss = x.split('-');
        graph.insert_both(ss.next().unwrap(), ss.next().unwrap());
    }
    graph
}

fn dfs<'arena>(
    graph: &'arena Graph,
    cur: &'arena str,
    visited: &mut HashMap<&'arena str, i32>,
    mut twice: bool,
) -> i32 {
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
    *visited.entry(cur).or_default() += 1;
    let mut res = 0;
    for nxt in graph.edges(&cur).unwrap() {
        res += dfs(graph, nxt, visited, twice);
    }
    *visited.entry(cur).or_default() -= 1;
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
