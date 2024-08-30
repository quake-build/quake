use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{BuildHasher, Hash, RandomState};
use std::marker::PhantomData;
use std::ops::Deref;

use indexmap::map::{Entry as IndexMapEntry, IndexMap};
use thiserror::Error;

// TODO introduce marker types to differentiate multigraphs from simple graphs

type NodeId = usize;

#[derive(Clone, PartialEq, Eq)]
pub struct Graph<K, N, E = ()>
where
    K: Eq + Hash,
{
    nodes: IndexMap<K, N>,
    edges: HashMap<NodeId, Vec<(NodeId, E)>>,
}

impl<K, N, E> Graph<K, N, E>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Graph {
            nodes: IndexMap::with_capacity(nodes),
            edges: HashMap::with_capacity(edges),
        }
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn insert_node(&mut self, key: K, node: N) -> Option<N> {
        self.nodes.insert(key, node)
    }

    pub fn get_node<'a>(&'a self, key: &K) -> Option<&'a N> {
        self.nodes.get(key)
    }

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

    fn id(&self, key: &K) -> Option<usize> {
        self.nodes.get_index_of(key)
    }

    fn edge(&self, from: &K, to: &K) -> Option<(usize, usize)> {
        Some((self.nodes.get_index_of(from)?, self.nodes.get_index_of(to)?))
    }

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
    #[inline]
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

pub struct Entry<'a, K, N, E>
where
    K: Eq + Hash,
{
    entry: IndexMapEntry<'a, K, N>,
    graph: &'a mut Graph<K, N, E>,
}

impl<'a, K, N, E> Entry<'a, K, N, E>
where
    K: Eq + Hash,
{
    #[inline(always)]
    fn new(key: &'a K, graph: &'a mut Graph<K, N, E>) -> Self {
        Self { entry: key, graph }
    }

    pub fn or_insert(self, node: N) -> &'a mut N {}

    pub fn or_default(self) -> &'a mut N
    where
        N: Default,
    {
        self.entry.or_default()
    }
}

pub struct Node<'a, K, N, E>
where
    K: Eq + Hash,
{
    todo: PhantomData<&'a (K, N, E)>,
}

impl<'a, K, N, E> Deref for Node<'a, K, N, E> {
    type Target = &'a N;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}
