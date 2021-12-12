use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;

pub struct Graph<V: Hash + Eq + Clone, E> {
    edges: HashMap<V, Vec<E>>,
}

impl<V: Hash + Eq + Clone, E> Graph<V, E> {
    pub fn new() -> Graph<V, E> {
        Graph {
            edges: HashMap::new(),
        }
    }

    pub fn edges<Q: ?Sized>(&self, v: &Q) -> Option<impl Iterator<Item = &E>>
    where
        V: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.edges.get(v).map(|es| es.iter())
    }

    pub fn insert(&mut self, v: V, e: E) {
        self.edges.entry(v).or_insert_with(Vec::new).push(e);
    }
}

impl<V: Hash + Eq + Clone> Graph<V, V> {
    pub fn insert_both(&mut self, v: V, u: V) {
        self.insert(v.clone(), u.clone());
        self.insert(u, v);
    }
}
