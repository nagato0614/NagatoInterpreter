use std::cell::Ref;
use petgraph::dot::{Dot, Config};
use petgraph::graph::{Graph, NodeIndex};
use std::fs::File;
use std::io::{self, Write};
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::parser::{Leaf, Node};

pub struct TreeViewer {
    graph: Graph<String, String>,
    node_index: usize,
}

impl TreeViewer {
    pub fn new() -> Self {
        TreeViewer {
            graph: Graph::<String, String>::new(),
            node_index: 0,
        }
    }

    pub fn make_tree(&mut self, root: &Rc<RefCell<Node>>) {
        self.node_index = 0;
        self.graph = Graph::<String, String>::new();

        self.add_node(root);
    }

    fn create_graph_node(&mut self, leaf: &Leaf) -> NodeIndex {
        let mut node_name: String;
        match leaf {
            Leaf::Node(node) => {
                node_name = format!("{}: {:?}", self.node_index, node.borrow().val().unwrap());
            }
            Leaf::FunctionCall(func) => {
                node_name = format!("{}: Function Call [{:?}]", self.node_index, func.name());
            }
            _ => {
                node_name = format!("{}: {:?}", self.node_index, leaf);
            }
        }

        let graph_node = self.graph.add_node(node_name);
        self.node_index += 1;

        graph_node
    }

    fn add_node(&mut self, node: &Rc<RefCell<Node>>) -> Option<NodeIndex> {
        if let Some(val) = node.borrow().val() {
            let graph_node = self.create_graph_node(val);

            if let Some(lhs) = node.borrow().lhs() {
                let lhs_graph_node = self.add_node(lhs);
                if let Some(lhs_graph_node) = lhs_graph_node {
                    self.graph.add_edge(graph_node, lhs_graph_node, String::from(""));
                }
            }

            if let Some(rhs) = node.borrow().rhs() {
                let rhs_graph_node = self.add_node(rhs);
                if let Some(rhs_graph_node) = rhs_graph_node {
                    self.graph.add_edge(graph_node,rhs_graph_node,  String::from(""));
                }
            }

            return Some(graph_node);
        }

        None
    }

    pub fn output_dot(&self) {
        let dot_output = format!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]));

        // ファイルに書き込み
        let mut file = File::create("output.dot").unwrap();
        file.write_all(dot_output.as_bytes()).unwrap();
    }
}