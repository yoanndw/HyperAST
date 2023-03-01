use hyper_ast::{store::nodes::legion::HashedNodeRef, types::WithChildren};
use hyper_ast_cvs_git::{preprocessed::PreProcessedRepository, git::fetch_github_repository};
use hyper_diff::{
    decompressed_tree_store::CompletePostOrder,
    matchers::{
        heuristic::gt::{greedy_subtree_matcher::{GreedySubtreeMatcher, SubtreeMatcher}},
        mapping_store::{DefaultMultiMappingStore, VecStore},
    },
};

fn main() {
     // INRIA/spoon 7c7f094bb22a350fa64289a94880cc3e7231468f 78d88752a9f4b5bc490f5e6fb0e31dc9c2cf4bcd "spoon-pom" "" 2
     let preprocessed = PreProcessedRepository::new("INRIA/spoon");
     let window_size = 2;
     let mut preprocessed = preprocessed;
     let (before, after) = (
         "7c7f094bb22a350fa64289a94880cc3e7231468f",
         "78d88752a9f4b5bc490f5e6fb0e31dc9c2cf4bcd",
     );
     assert!(window_size > 1);

     let processing_ordered_commits = preprocessed.pre_process_with_limit(
         &mut fetch_github_repository(&preprocessed.name),
         before,
         after,
         "spoon-pom",
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
     let mappings = VecStore::default();
     type DS<'a> = CompletePostOrder<HashedNodeRef<'a>, u32>;
     let mapper = GreedySubtreeMatcher::<DS, DS, _, _, _>::matchh::<DefaultMultiMappingStore<_>>(
         &stores.node_store,
         &src,
         &dst,
         mappings,
     );
     let SubtreeMatcher {
         src_arena,
         dst_arena,
         mappings,
         ..
     } = mapper.into();
     print_mappings(
         &dst_arena,
         &src_arena,
         &stores.node_store,
         &stores.label_store,
         &mappings,
     );
}