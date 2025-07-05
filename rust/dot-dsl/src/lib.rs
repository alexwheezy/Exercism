pub mod graph {
    use graph_items::{edge::Edge, node::Node};
    use std::collections::HashMap;

    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Self {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Self {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            let attrs: HashMap<String, String> = attrs
                .iter()
                .map(|tuple| (tuple.0.to_owned(), tuple.1.to_owned()))
                .collect();
            Self { attrs, ..self }
        }

        pub fn node(&self, name: &str) -> Option<Node> {
            self.nodes.iter().find(|&n| n.name() == name).cloned()
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                start_node: String,
                end_node: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(start_node: &str, end_node: &str) -> Self {
                    Self {
                        start_node: start_node.to_owned(),
                        end_node: end_node.to_owned(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    let attrs: HashMap<String, String> = attrs
                        .iter()
                        .map(|tuple| (tuple.0.to_owned(), tuple.1.to_owned()))
                        .collect();
                    Self { attrs, ..self }
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| s.as_ref())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct Node {
                name: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(node: &str) -> Self {
                    Self {
                        name: node.to_owned(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn name(&self) -> &str {
                    &self.name
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    let attrs: HashMap<String, String> = attrs
                        .iter()
                        .map(|tuple| (tuple.0.to_owned(), tuple.1.to_owned()))
                        .collect();
                    Self { attrs, ..self }
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| s.as_ref())
                }
            }
        }
    }
}
