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
    types::{IterableChildren, Labeled, Tree, Type, Typed, WithChildren, WithStats},
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

type WalkStackElement = NodeIdentifier;

struct HyperAstWalkIter<'a> {
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

// TODO: pourquoi avec HashedNodeRef il y avait un problème de lifetime
impl<'a> Iterator for HyperAstWalkIter<'a> {
    type Item = CompressedNode<NodeIdentifier, LabelIdentifier>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(node) = self.stack.pop() else {
            return None;
        } ;

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

// use hyper_ast::{store::nodes::legion::HashedNodeRef, types::WithChildren};
use hyper_ast_cvs_git::{git::fetch_github_repository, preprocessed::PreProcessedRepository};
use hyper_diff::{
    decompressed_tree_store::CompletePostOrder,
    matchers::{
        heuristic::gt::greedy_subtree_matcher::{GreedySubtreeMatcher, SubtreeMatcher},
        mapping_store::{DefaultMultiMappingStore, VecStore},
    },
};

use hyper_ast_benchmark_diffs::postprocess::print_mappings;

// use hyper_ast_benchmark_diffs::print_mappings;
use std::io::Write;
fn main() {
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
    //     .format(|buf, record| {
    //         if record.level().to_level_filter() > log::LevelFilter::Debug {
    //             writeln!(buf, "{}", record.args())
    //         } else {
    //             writeln!(
    //                 buf,
    //                 "[{} {}] {}",
    //                 buf.timestamp_millis(),
    //                 record.level(),
    //                 record.args()
    //             )
    //         }
    //     })
    //     .init();
    // INRIA/spoon 7c7f094bb22a350fa64289a94880cc3e7231468f 78d88752a9f4b5bc490f5e6fb0e31dc9c2cf4bcd "spoon-pom" "" 2
    // INRIA/spoon 7c7f094bb22a350fa64289a94880cc3e7231468f 78d88752a9f4b5bc490f5e6fb0e31dc9c2cf4bcd "spoon-pom" "" 2
    let preprocessed = PreProcessedRepository::new("yoanndw/TPMaven");
    let window_size = 2;
    let mut preprocessed = preprocessed;
    // let (before, after) = (
    //     "7c7f094bb22a350fa64289a94880cc3e7231468f",
    //     "78d88752a9f4b5bc490f5e6fb0e31dc9c2cf4bcd",
    // );
    let (before, after) = (
        "ff85007418d66190ae86de6eb741f3f04051aa0c",
        "4675bf2c22f34c99cff72d9de8ef9f7dc57ec929",
    );
    assert!(window_size > 1);

    let processing_ordered_commits = preprocessed.pre_process_with_limit(
        &mut fetch_github_repository(&preprocessed.name),
        before,
        after,
        "",
        1000,
    );
    preprocessed.processor.purge_caches();
    let c_len = processing_ordered_commits.len();
    let c = (0..c_len - 1)
        .map(|c| &processing_ordered_commits[c..(c + window_size).min(c_len)])
        .next()
        .unwrap();
    let oid_src = &c[0];
    let oid_dst = &c[1];

    let commit_src = preprocessed.commits.get_key_value(&oid_src).unwrap();
    let src_tr = commit_src.1.ast_root;
    // let src_tr = preprocessed.child_by_name(src_tr, "hadoop-common-project").unwrap();

    let commit_dst = preprocessed.commits.get_key_value(&oid_dst).unwrap();
    let dst_tr = commit_dst.1.ast_root;
    // let dst_tr = preprocessed.child_by_name(dst_tr, "hadoop-common-project").unwrap();
    let stores = &preprocessed.processor.main_stores;
    let src = &src_tr;
    let dst = &dst_tr;
    // let mappings = VecStore::default();
    // type DS<'a> = CompletePostOrder<HashedNodeRef<'a>, u32>;
    // let mapper = GreedySubtreeMatcher::<DS, DS, _, _, _>::matchh::<DefaultMultiMappingStore<_>>(
    //     &stores.node_store,
    //     &src,
    //     &dst,
    //     mappings,
    // );
    // let SubtreeMatcher {
    //     src_arena,
    //     dst_arena,
    //     mappings,
    //     ..
    // } = mapper.into();
    // print_mappings(
    //     &dst_arena,
    //     &src_arena,
    //     &stores.node_store,
    //     &stores.label_store,
    //     &mappings,
    // );

    let node = stores.node_store.resolve(src_tr);
    dbg!(node.child_count());

    let iter = HyperAstWalkIter::new(&stores, &src_tr);
    for n in iter {
        dbg!(n.get_type());
    }
}
