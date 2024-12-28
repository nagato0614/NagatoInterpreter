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
    node_index_list: Vec<NodeIndex>,
    node_index: usize,
}

impl TreeViewer {
    pub fn new() -> Self {
        TreeViewer {
            graph: Graph::<String, String>::new(),
            node_index_list: Vec::new(),
            node_index: 0,
        }
    }

    pub fn make_tree(&mut self, root: &Rc<RefCell<Node>>) {
        self.add_node(root);
    }

    fn create_graph_node(&mut self, leaf: &Leaf) -> NodeIndex {
        let node_name = match leaf {
            Leaf::Node(node) => format!("{}: {:?}", self.node_index, node.borrow().val().unwrap()),
            Leaf::FunctionCall(func) => format!("{}: Function Call [{:?}]", self.node_index, func.name()),
            Leaf::FunctionDefinition(func) => format!("{}: Function Definition [{:?}]", self.node_index, func.name()),
            Leaf::IfStatement(_) => format!("{}: If Statement", self.node_index),
            Leaf::BlockItem(_) => format!("{}: Block Item", self.node_index),
            Leaf::ForStatement(_) => format!("{}: For Statement", self.node_index),
            _ => format!("{}: {:?}", self.node_index, leaf),
        };

        let graph_node = self.graph.add_node(node_name);
        self.node_index_list.push(graph_node);
        self.node_index += 1;

        match leaf {
            Leaf::FunctionDefinition(func) => self.add_nodes_and_edges(graph_node, func.body()),
            Leaf::FunctionCall(func) => self.add_nodes_and_edges(graph_node, func.arguments()),
            Leaf::BlockItem(block) => self.add_nodes_and_edges(graph_node, block),
            Leaf::ForStatement(for_stmt) => {
                let initializer = for_stmt.initializer();
                let condition = for_stmt.condition();
                let increment = for_stmt.update();
                let body = for_stmt.statement();
                
                self.add_node_and_edge(graph_node, initializer);
                self.add_node_and_edge(graph_node, condition);
                self.add_node_and_edge(graph_node, increment);
                self.add_node_and_edge(graph_node, body);
                
            }
            _ => {}
        }

        graph_node
    }

    fn add_nodes_and_edges(&mut self, parent_node: NodeIndex, items: &[Rc<RefCell<Node>>]) {
        for item in items.iter() {
            if let Some(node_index) = self.add_node(item) {
                self.graph.add_edge(parent_node, node_index, String::from(""));
            }
        }
    }
    
    fn add_node_and_edge(&mut self, parent_node: NodeIndex, node: &Rc<RefCell<Node>>) {
        if let Some(node_index) = self.add_node(node) {
            self.graph.add_edge(parent_node, node_index, String::from(""));
        }
    }
    
    fn add_node(&mut self, node: &Rc<RefCell<Node>>) -> Option<NodeIndex> {
        if let Some(val) = node.borrow().val() {
            println!("val: {:?}", val);
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
                    self.graph.add_edge(graph_node, rhs_graph_node, String::from(""));
                }
            }

            return Some(graph_node);
        }

        None
    }

    pub fn output_dot(&self, file_name: &str) {
        let dot_output = format!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]));

        // ファイルに書き込み
        let mut file = File::create(file_name).unwrap();
        file.write_all(dot_output.as_bytes()).unwrap();
    }
}