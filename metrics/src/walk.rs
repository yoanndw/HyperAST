use hyper_ast::nodes::CompressedNode;
use hyper_ast::store::{
    defaults::LabelIdentifier, nodes::DefaultNodeIdentifier as NodeIdentifier, SimpleStores,
};
use hyper_ast::types::{IterableChildren, WithChildren};

pub type WalkStackElement = NodeIdentifier;

pub struct HyperAstWalkIter<'a> {
    stack: Vec<WalkStackElement>,
    stores: &'a SimpleStores,
}

impl<'a> HyperAstWalkIter<'a> {
    pub fn new(stores: &'a SimpleStores, root: &NodeIdentifier) -> Self {
        let mut stack = Vec::new();
        stack.push(root.clone());
        Self { stack, stores }
    }
}

impl<'a> Iterator for HyperAstWalkIter<'a> {
    type Item = CompressedNode<NodeIdentifier, LabelIdentifier>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(node) = self.stack.pop() else {
            return None;
        };

        let node_ref = self.stores.node_store.resolve(node);
        let compressed_node = node_ref.into_compressed_node().unwrap();

        if let Some(children) = node_ref.children() {
            for c in children.iter_children().rev() {
                self.stack.push(*c);
            }
        }

        Some(compressed_node)
    }
}
