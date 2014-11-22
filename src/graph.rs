use code;
use std::fmt::Show;
use std::hash::Hash;
use graphviz as dot;
use graphviz::maybe_owned_vec::IntoMaybeOwnedVector;

type Node<'a, T> = &'a code::Node<T>;
type Edge<'a, T> = (Node<'a, T>, Node<'a, T>);
struct Graph<'a, T: 'a + Ord + Copy> {
    pub name: &'a str,
    pub root: &'a code::Node<T>,
}

impl<'a, T: Ord + Copy + Hash + Show> dot::Labeller<'a, Node<'a, T>, Edge<'a, T>> for Graph<'a, T> {
    fn graph_id(&'a self) -> dot::Id<'a> {
        dot::Id::new(self.name.as_slice()).unwrap()
    }

    fn node_id(&self, node: &Node<'a, T>) -> dot::Id<'a> {
        let h = ::std::hash::hash(&**node);
        dot::Id::new(format!("node_{}", h)).unwrap()
    }

    fn node_label(&self, node: &Node<'a, T>) -> dot::LabelText<'a> {
        let label = match *node {
            &code::Node::Leaf(value, weight) => format!("{} ({})", value, weight),
            ref b @ &code::Node::Branch(..) => format!("{}", b.weight()),
        };
        dot::LabelText::LabelStr(label.into_maybe_owned())
    }
}

impl<'a, T: Ord + Copy> dot::GraphWalk<'a, Node<'a, T>, Edge<'a, T>> for Graph<'a, T> {
    fn nodes(&'a self) -> dot::Edges<'a, Node<'a, T>> {
        let mut nodes = Vec::new();

        fn walk<'a, T: Ord + Copy>(nodes: &mut Vec<Node<'a, T>>, node: &'a code::Node<T>) {
            nodes.push(node);

            match *node {
                code::Node::Leaf(..) => {}
                code::Node::Branch(ref left, ref right) => {
                    walk(nodes, &**left);
                    walk(nodes, &**right);
                }
            }
        }
        walk(&mut nodes, self.root);

        nodes.into_maybe_owned()
    }

    fn edges(&'a self) -> dot::Edges<'a, Edge<'a, T>> {
        let mut edges = Vec::new();

        fn walk<'a, T: Ord + Copy>(edges: &mut Vec<Edge<'a, T>>, node: &'a code::Node<T>) {
            match *node {
                code::Node::Leaf(..) => {}
                code::Node::Branch(ref left, ref right) => {
                    edges.push((node, &**left));
                    edges.push((node, &**right));

                    walk(edges, &**left);
                    walk(edges, &**right);
                }
            }
        }
        walk(&mut edges, self.root);

        edges.into_maybe_owned()
    }

    fn source(&self, edge: &Edge<'a, T>) -> Node<'a, T> {
        edge.val0()
    }

    fn target(&self, edge: &Edge<'a, T>) -> Node<'a, T> {
        edge.val1()
    }
}

pub fn render<W: Writer, T: Ord + Copy + Hash + Show>(w: &mut W, name: &str, root: &code::Node<T>) -> ::std::io::IoResult<()> {
    let graph = Graph {
        name: name,
        root: root,
    };
    dot::render(&graph, w)
}
