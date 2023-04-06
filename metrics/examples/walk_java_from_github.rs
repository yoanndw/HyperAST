use algorithms::walk::HyperAstWalkIter;
use hyper_ast_cvs_git::{git::fetch_github_repository, preprocessed::PreProcessedRepository};
use hyper_ast::types::Typed;

fn main() {
    let preprocessed = PreProcessedRepository::new("yoanndw/TPMaven");
    let window_size = 2;

    let mut preprocessed = preprocessed;

    let (before, after) = (

        "ff85007418d66190ae86de6eb741f3f04051aa0c",
        "4675bf2c22f34c99cff72d9de8ef9f7dc57ec929",
    );

    let processing_ordered_commits = preprocessed.pre_process_with_limit(&mut fetch_github_repository(&preprocessed.name), before, after, "", 1000);
    preprocessed.processor.purge_caches();

    let c_len = processing_ordered_commits.len();
    let c = (0..c_len - 1)
        .map(|c| &processing_ordered_commits[c..(c + window_size).min(c_len)])
        .next()
        .unwrap();

    let oid_src = &c[0];
    let oid_dst = &c[1];

    let commit_src = preprocessed.commits.get_key_value(&oid_src).unwrap();
    let commit_dst = preprocessed.commits.get_key_value(&oid_dst).unwrap();

    let src_tr = commit_src.1.ast_root;
    let dst_tr = commit_dst.1.ast_root;

    let stores = &preprocessed.processor.main_stores;

    let src_node_ref = stores.node_store.resolve(src_tr);
    let walk_iter = HyperAstWalkIter::new(stores, &src_tr);
    for n in walk_iter {
        println!("Node type: {:?}", n.get_type());
    }
}