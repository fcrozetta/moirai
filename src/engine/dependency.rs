use std::collections::{HashMap, HashSet};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use petgraph::visit::EdgeRef;
use crate::models::Job;
use crate::system_plugin;

// Represents the dependency graph of a workflow
pub struct DependencyGraph<'a> {
    graph: DiGraph<String,String>,
    node_indices: HashMap<String, NodeIndex>,
    job: &'a Job,
}

impl<'a> DependencyGraph<'a> {
    pub fn new(job: &'a Job) -> Self {
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();

        
        for node in &job.nodes {
            let idx = graph.add_node(node.id.clone());
            node_indices.insert(node.id.clone(),idx);
        }

        for edge in &job.edges {
            if let (Some(from_idx), Some(to_idx)) = (
                node_indices.get(&edge.from),
                node_indices.get(&edge.to),
            ) {
                graph.add_edge(*from_idx, *to_idx, edge.from_port.clone());
            }
        }

        Self {
            graph,
            node_indices,
            job: job,
        }

    }

    pub fn get_start_nodes(&self) -> HashSet<String> {
        self.job.nodes.iter()
            .filter( |n|  system_plugin::is_start_node(&n.plugin,&n.action))
            .map(|n| n.id.clone())
            .collect()
    }

    pub fn get_end_nodes(&self) -> HashSet<String> {
        self.job.nodes.iter()
            .filter(|n| system_plugin::is_end_node(&n.plugin, &n.action))
            .map(|n| n.id.clone())
            .collect()
    }

    //? I really believe the AI is trying to run everything that has no dependencies first. 
    //? It will cause issues
    pub fn get_nodes_with_no_dependencies(&self) -> HashSet<String> {
        let mut no_deps = HashSet::new();

        for (node_id, idx) in &self.node_indices {
            let incoming = self.graph.edges_directed(*idx, Direction::Incoming).count();
            if incoming == 0 {
                no_deps.insert(node_id.clone());
            }

        }
        no_deps
    }

    pub fn get_dependencies(&self, node_id: &str) -> HashSet<String> {
        let mut deps = HashSet::new();

        if let Some(idx) = self.node_indices.get(node_id) {
            for edge in self.graph.edges_directed(*idx, Direction::Incoming) {
                let src_idx = edge.source();
                for (id, i) in &self.node_indices {
                    if *i == src_idx {
                        deps.insert(id.clone());
                    }
                }
            }
        }
        deps
    }

    pub fn get_dependents(&self, node_id: &str) -> HashSet<String> {
        let mut deps = HashSet::new();

        if let Some(idx) = self.node_indices.get(node_id) {
            for edge in self.graph.edges_directed(*idx, Direction::Outgoing) {
                let dest_idx = edge.target();
                for (id, i) in &self.node_indices {
                    if *i == dest_idx {
                        deps.insert(id.clone());
                    }
                }
            }
        }
        deps
    }



}