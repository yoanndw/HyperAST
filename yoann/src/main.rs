use std::{collections::VecDeque, marker::PhantomData};

use hyper_ast::{
    nodes::CompressedNode,
    position::{StructuralPosition, TreePath},
    store::labels::LabelStore,
    store::SimpleStores,
    store::TypeStore,
    store::{
        defaults::{LabelIdentifier, NodeIdentifier},
        nodes::{legion::HashedNodeRef, DefaultNodeStore as NodeStore},
    },
    tree_gen::parser::{Node, TreeCursor},
    types::{IterableChildren, Labeled, Tree, Type, Typed, WithChildren},
};

use hyper_ast_gen_ts_java::legion_with_refs::{
    print_tree_ids, print_tree_syntax, print_tree_syntax_with_ids, JavaTreeGen, TTreeCursor,
};

static CASE_YOANN1: &'static str = r#"if (1 < 2) {
    f();
    while (true) {}
} else {
    while (1 == 1)
        while (false)
            g();
}"#;

static CASE_YOANN2: &'static str = r#"(1 < 2)"#;

fn main() {
    let case = CASE_YOANN1;
    let tree = JavaTreeGen::tree_sitter_parse(case.as_bytes()).unwrap_or_else(|t| t);

    println!("{}", tree.root_node().to_sexp());

    println!("===========");

    let stores = Box::new(SimpleStores {
        label_store: LabelStore::new(),
        type_store: TypeStore {},
        node_store: NodeStore::new(),
    });
    let md_cache = Box::new(Default::default());
    let mut java_tree_gen = JavaTreeGen::new(Box::leak(stores), Box::leak(md_cache));

    let full_node = java_tree_gen.generate_file(b"", case.as_bytes(), tree.walk());
    let root = full_node.local.compressed_node as NodeIdentifier;

    //walk_hast(java_tree_gen.stores, &StructuralPosition::new(root), root);
    //walk_imp(java_tree_gen.stores, &StructuralPosition::new(root));

    let walk_iter = HyperAstWalkIter::new(java_tree_gen.stores, &StructuralPosition::new(root));
    // for cn in walk_iter {
    //     println!("Iter type: {:?}", cn.get_type());
    // }

    let num = walk_iter
        .filter(|n| n.get_type() == Type::WhileStatement)
        // .for_each(|nc| println!("{:?}", nc));
        .count();

    println!("Count while: {}", num);
}

type WalkStackElement = StructuralPosition;

struct HyperAstWalkIter<'a> {
    stack: VecDeque<WalkStackElement>,
    stores: &'a SimpleStores,
}

impl<'a> HyperAstWalkIter<'a> {
    pub fn new(stores: &'a SimpleStores, path: &StructuralPosition) -> Self {
        let mut stack = VecDeque::new();
        stack.push_back(path.clone());
        Self { stack, stores }
    }
}

// TODO: pourquoi avec HashedNodeRef il y avait un problème de lifetime
impl<'a> Iterator for HyperAstWalkIter<'a> {
    type Item = CompressedNode<NodeIdentifier, LabelIdentifier>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            None
        } else {
            let top = self.stack.pop_back().unwrap();

            let node = top.node().unwrap();
            let node_ref = self.stores.node_store.resolve(*node);
            let compressed_node = node_ref.into_compressed_node().unwrap();

            if node_ref.has_children() {
                for c in node_ref.children().unwrap().iter_children().rev() {
                    self.stack.push_back(StructuralPosition::new(*c));
                }
            }

            Some(compressed_node)
        }
    }
}
