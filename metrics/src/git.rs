use hyper_ast::store::{nodes::DefaultNodeIdentifier as NodeIdentifier, SimpleStores};
use hyper_ast_cvs_git::{preprocessed::PreProcessedRepository};

pub fn hyper_ast_from_git_repo<'a>(preprocessed: &'a mut PreProcessedRepository, processing_ordered_commits: &'a Vec<git2::Oid>, window_size: usize) -> (&'a SimpleStores, NodeIdentifier) {
    preprocessed.processor.purge_caches();

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

    let stores = &preprocessed.processor.main_stores;

    // let src_node_ref = stores.node_store.resolve(src_tr);
    // let walk_iter = HyperAstWalkIter::new(stores, &src_tr);

    (stores, src_tr)
}