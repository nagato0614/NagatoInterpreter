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
        let mut node_name: String;
        match leaf {
            Leaf::Node(node) => {
                node_name = format!("{}: {:?}", self.node_index, node.borrow().val().unwrap());
            }
            Leaf::FunctionCall(func) => {
                node_name = format!("{}: Function Call [{:?}]", self.node_index, func.name());
                

            }
            Leaf::FunctionDefinition(func) => {
                node_name = format!("{}: Function Definition [{:?}]", self.node_index, func.name());
            }
            _ => {
                node_name = format!("{}: {:?}", self.node_index, leaf);
            }
        }

        let graph_node = self.graph.add_node(node_name);
        self.node_index_list.push(graph_node);
        self.node_index += 1;
        
        // 関数の場合は body のノードを追加
        if let Leaf::FunctionDefinition(func) = leaf {
            let body = func.body();
            for (i, b) in body.iter().enumerate() {
                let node_index = self.add_node(b);
                if let Some(node_index) = node_index {
                    self.graph.add_edge(graph_node, node_index, String::from(""));
                }
            }
        }
        
        // 関数呼び出しの場合は引数のノードを追加
        if let Leaf::FunctionCall(func) = leaf {
            let args = func.arguments();
            for (i, arg) in args.iter().enumerate() {
                let node_index = self.add_node(arg);
                if let Some(node_index) = node_index {
                    self.graph.add_edge(graph_node, node_index, String::from(""));
                }
            }
        }

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