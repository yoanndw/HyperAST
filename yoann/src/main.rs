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

// static CASE_YOANN1: &'static str = r#"if (1 < 2) {
//     f();
//     while (true) {}
// } else {
//     while (1 == 1)
//         while (false)
//             g();
// }"#;

// static CASE_YOANN2: &'static str = r#"(1 < 2)"#;

// fn main() {
//     let case = CASE_YOANN1;
//     let tree = JavaTreeGen::tree_sitter_parse(case.as_bytes()).unwrap_or_else(|t| t);

//     println!("{}", tree.root_node().to_sexp());

//     println!("===========");

//     let stores = Box::new(SimpleStores {
//         label_store: LabelStore::new(),
//         type_store: TypeStore {},
//         node_store: NodeStore::new(),
//     });
//     let md_cache = Box::new(Default::default());
//     let mut java_tree_gen = JavaTreeGen::new(Box::leak(stores), Box::leak(md_cache));

//     let full_node = java_tree_gen.generate_file(b"", case.as_bytes(), tree.walk());
//     let root = full_node.local.compressed_node as NodeIdentifier;

//     //walk_hast(java_tree_gen.stores, &StructuralPosition::new(root), root);
//     //walk_imp(java_tree_gen.stores, &StructuralPosition::new(root));

//     let walk_iter = HyperAstWalkIter::new(java_tree_gen.stores, &StructuralPosition::new(root));
//     // for cn in walk_iter {
//     //     println!("Iter type: {:?}", cn.get_type());
//     // }

//     let num = walk_iter
//         .filter(|n| n.get_type() == Type::WhileStatement)
//         // .for_each(|nc| println!("{:?}", nc));
//         .count();

//     println!("Count while: {}", num);
// }

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


// use hyper_ast::{store::nodes::legion::HashedNodeRef, types::WithChildren};
use hyper_ast_cvs_git::{preprocessed::PreProcessedRepository, git::fetch_github_repository};
use hyper_diff::{
    decompressed_tree_store::CompletePostOrder,
    matchers::{
        heuristic::gt::{greedy_subtree_matcher::{GreedySubtreeMatcher, SubtreeMatcher}},
        mapping_store::{DefaultMultiMappingStore, VecStore},
    },
};

use hyper_ast_benchmark_diffs::postprocess::print_mappings;

// use hyper_ast_benchmark_diffs::print_mappings;

fn main() {
    // INRIA/spoon 7c7f094bb22a350fa64289a94880cc3e7231468f 78d88752a9f4b5bc490f5e6fb0e31dc9c2cf4bcd "spoon-pom" "" 2
    let preprocessed = PreProcessedRepository::new("yoanndw/TPMaven");
    let window_size = 2;
    let mut preprocessed = preprocessed;
    let (before, after) = (
        "9acb418d7c750fce0924b2d71185f798d00c2bb0",
        "9acb418d7c750fce0924b2d71185f798d00c2bb0"
    );
    let (before, after) = (
        "HEAD",
        "HEAD"
    );
    assert!(window_size > 1);

    let processing_ordered_commits = preprocessed.pre_process_with_limit(
        &mut fetch_github_repository(&preprocessed.name),
        before,
        after,
        "/",
        1000,
    );
    preprocessed.processor.purge_caches();
    let c_len = processing_ordered_commits.len();
    println!("Vec commits: {:?}", processing_ordered_commits);
    let c = (0..c_len)
        .map(|c| &processing_ordered_commits[c..(c + window_size).min(c_len)])
        .next()
        .unwrap();
    let oid_src = &c[0];
//  let oid_dst = &c[1];
    let oid_dst = &c[0];

    let commit_src = preprocessed.commits.get_key_value(&oid_src).unwrap();
    let src_tr = commit_src.1.ast_root;
    // let src_tr = preprocessed.child_by_name(src_tr, "hadoop-common-project").unwrap();

    let commit_dst = preprocessed.commits.get_key_value(&oid_dst).unwrap();
    let dst_tr = commit_dst.1.ast_root;
    // let dst_tr = preprocessed.child_by_name(dst_tr, "hadoop-common-project").unwrap();
    let stores = &preprocessed.processor.main_stores;

    println!("========================");
    // println!("Store: {:?}", stores);
    let walk_iter = HyperAstWalkIter::new(stores, &StructuralPosition::new(src_tr));
    for cn in walk_iter {
        println!("Iter node: {:?}", cn);
        println!("Iter type: {:?}", cn.get_type());
    }
}