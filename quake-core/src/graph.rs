use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use indexmap::IndexMap;
use thiserror::Error;

// TODO introduce marker types to differentiate multigraphs from simple graphs

type NodeId = usize;

#[derive(Clone)]
pub struct Graph<K, N, E = ()>
where
    K: Eq + Hash + Display,
{
    nodes: IndexMap<K, N>,
    edges: HashMap<NodeId, Vec<(NodeId, E)>>,
}

impl<K, N, E> Graph<K, N, E>
where
    K: Eq + Hash + Display,
{
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Graph {
            nodes: IndexMap::with_capacity(nodes),
            edges: HashMap::with_capacity(edges),
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    #[inline]
    pub fn insert_node(&mut self, key: K, node: N) -> Option<N> {
        self.nodes.insert(key, node)
    }

    #[inline]
    pub fn get_node<'a>(&'a self, key: &K) -> Option<&'a N> {
        self.nodes.get(key)
    }

    #[inline]
    pub fn get_node_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut N> {
        self.nodes.get_mut(key)
    }

    #[inline]
    pub fn contains_node(&self, key: &K) -> bool {
        self.nodes.contains_key(key)
    }

    #[inline]
    pub fn nodes<'a>(&'a self) -> impl Iterator<Item = &'a N> {
        self.nodes.values()
    }

    pub fn find_node<'a, P>(&'a self, mut predicate: P) -> Option<(&'a K, &'a N)>
    where
        P: FnMut(&N) -> bool,
    {
        self.nodes
            .iter()
            .find(|(_k, n)| predicate(&n))
            .map(|(k, n)| (k, n))
    }

    pub fn insert_edge(&mut self, from: &K, to: &K, edge: E) -> Option<()> {
        // NOTE: parallel edges permitted
        let (from, to) = (self.id(from)?, self.id(to)?);
        self.edges.entry(from).or_default().push((to, edge));
        Some(())
    }

    pub fn get_edges<'a>(&'a self, from: &K, to: &K) -> Option<impl Iterator<Item = &'a E>> {
        let (from, to) = self.edge(from, to)?;
        self.edges.get(&from).map(move |edges| {
            edges
                .iter()
                .filter_map(move |(other, edge)| (to == *other).then(|| edge))
        })
    }

    pub fn contains_edge(&self, from: &K, to: &K) -> bool {
        // TODO implement less hacky version later
        self.get_edges(from, to)
            .map(|mut edges| edges.next().is_some())
            .unwrap_or(false)
    }

    pub fn num_edges(&self) -> usize {
        self.edges.values().map(|es| es.len()).sum()
    }

    pub fn edges<'a>(&'a self) -> impl Iterator<Item = ((&'a K, &'a K), &'a E)> {
        self.edges.iter().flat_map(|(from, edges)| {
            edges
                .iter()
                .map(|(to, edge)| ((self.key(*from), self.key(*to)), edge))
        })
    }

    pub fn edges_from<'a>(&'a self, key: &K) -> Option<impl Iterator<Item = (&'a K, &'a E)>> {
        let id = self.id(key)?;
        self.edges.get(&id).map(|edges| {
            edges
                .iter()
                .map(|(id, edge)| (self.nodes.get_index(*id).unwrap().0, edge))
        })
    }

    // TODO edges_to function
    // (which might invite a redesign of the internals for efficiency)

    #[inline(always)]
    fn id(&self, key: &K) -> Option<usize> {
        self.nodes.get_index_of(key)
    }

    #[inline(always)]
    fn edge(&self, from: &K, to: &K) -> Option<(usize, usize)> {
        Some((self.nodes.get_index_of(from)?, self.nodes.get_index_of(to)?))
    }

    #[inline(always)]
    fn key<'a>(&'a self, index: usize) -> &'a K {
        self.nodes
            .get_index(index)
            .map(|(key, _)| key)
            .expect("invalid internal key for graph")
    }
}

impl<K, N, E> Default for Graph<K, N, E>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self {
            nodes: IndexMap::new(),
            edges: HashMap::new(),
        }
    }
}

#[derive(Debug, Error)]
pub enum GraphError<K: Display> {
    #[error("invalid key: {0}")]
    InvalidKey(K),
}
