use hyper_ast::{store::{nodes::{DefaultNodeIdentifier as NodeIdentifier, DefaultNodeStore as NodeStore}, SimpleStores, labels::LabelStore, TypeStore}};
use hyper_ast_cvs_git::{preprocessed::PreProcessedRepository};
use hyper_ast_gen_ts_java::legion_with_refs::JavaTreeGen;

pub fn hyper_ast_from_str(case: &str) -> (SimpleStores, NodeIdentifier) {
    let tree = JavaTreeGen::tree_sitter_parse(case.as_bytes()).unwrap_or_else(|t| t);

    let mut stores = SimpleStores {
        label_store: LabelStore::new(),
        type_store: TypeStore {},
        node_store: NodeStore::new(),
    };
    let md_cache = Box::new(Default::default());
    let mut java_tree_gen = JavaTreeGen::new(&mut stores, Box::leak(md_cache));

    let full_node = java_tree_gen.generate_file(b"", case.as_bytes(), tree.walk());
    let root = full_node.local.compressed_node as NodeIdentifier;

    (stores, root)
}

pub fn hyper_ast_from_git_repo<'a>(preprocessed: PreProcessedRepository, processing_ordered_commits: Vec<git2::Oid>, window_size: usize) -> (SimpleStores, NodeIdentifier) {
    // preprocessed.processor.purge_caches();

    let c_len = processing_ordered_commits.len();
    let c = (0..c_len)
        .map(|c| &processing_ordered_commits[c..(c + window_size).min(c_len)])
        .next();

    let oid_src = &c.unwrap()[0];
    //let oid_dst = &c.unwrap()[1];

    let commit_src = preprocessed.commits.get_key_value(&oid_src).unwrap();
    //let commit_dst = preprocessed.commits.get_key_value(&oid_dst).unwrap();

    let src_tr = commit_src.1.ast_root;
    //let dst_tr = commit_dst.1.ast_root;

    // let stores = &preprocessed.processor.main_stores;

    // let src_node_ref = stores.node_store.resolve(src_tr);
    // let walk_iter = HyperAstWalkIter::new(stores, &src_tr);

    (preprocessed.processor.main_stores, src_tr)
}