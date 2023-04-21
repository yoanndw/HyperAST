use std::fs;
use std::path::Path;

use hyper_ast::types::{Type, Typed, HyperAST};

use hyper_ast::store::{nodes::DefaultNodeIdentifier as NodeIdentifier, SimpleStores};
use hyper_ast::types::WithChildren;
use hyper_ast_cvs_git::preprocessed::PreProcessedRepository;

use hyper_ast_metrics::count::{
    count_instanceofs, count_nodes, count_numbers, count_while_statements, evaluate_plagiarism,
};
use hyper_ast_metrics::mcc::cyclomatic_complexity;
use hyper_ast_metrics::utils::{clone_with_access_token, hyper_ast_from_git_repo};
use hyper_ast_metrics::walk::HyperAstWalkIter;

fn main() {
    let access_token = "glpat-Wu4BVYUiFGqHFyag7zZe";
    let mut subpath = "/tmp/clone/".to_owned();

    // dbg!("Directory test");

    //on vérifie si le dossier n'existe pas deja pour coriger le bug
    if Path::new(&subpath).is_dir() {
        // dbg!("supression fichier");
        fs::remove_dir_all(&subpath).expect("Error removing directory");
        // println!("Directory removed successfully!");
    }

    // let mut preprocesseds_list = vec![];
    // let mut processeds_commits_list = vec![];

    let url = "https://gitlab.istic.univ-rennes1.fr/rboure/test-maven";

    let username = "ydewilde";

    let path = subpath.to_owned() + username;

    let mut repo = clone_with_access_token(&url, access_token, &path);
    let mut preprocessed = PreProcessedRepository::new("");
    let window_size = 2;

    let processing_ordered_commits =
        preprocessed.pre_process_with_limit(&mut repo, "HEAD", "", "", 1000);

    // preprocesseds_list.push(preprocessed);
    // processeds_commits_list.push(processing_ordered_commits);

    // let preprocessed = preprocesseds_list.get(i);
    // let processing_ordered_commits = processeds_commits_list.get(i).unwrap();

    let ast = hyper_ast_from_git_repo(preprocessed, processing_ordered_commits, window_size);
    println!("Stores: {:?}", ast.0.label_store());

    let iter = HyperAstWalkIter::new(&ast.0, &ast.1);
    println!("--------");
    for n in iter {
        // let mcc = cyclomatic_complexity();
        // println!("MCC = {}", mcc);
    }
    println!("--------");

    // let whiles = count_while_statements(&ast);
    // println!("whiles = {}", whiles);
    let mcc = cyclomatic_complexity(&ast);
    println!("MCC = {}", mcc);
}
