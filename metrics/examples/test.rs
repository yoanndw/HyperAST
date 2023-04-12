use hyper_ast::store::{SimpleStores, TypeStore, nodes::DefaultNodeIdentifier as NodeIdentifier, nodes::DefaultNodeStore as NodeStore, labels::LabelStore};
use hyper_ast_gen_ts_java::legion_with_refs::JavaTreeGen;
use hyper_ast_metrics::{
    count::count_while_statements, utils::hyper_ast_from_str, walk::HyperAstWalkIter,
};

fn main() {
    let case = r#"while () {while ()}"#;
    let case = r#"while () {
                while () // ERROR
            }"#;
    let case = r#"do {} while ();"#;

    let tree = JavaTreeGen::tree_sitter_parse(case.as_bytes()).unwrap_or_else(|t| t);

    println!("{}", tree.root_node().to_sexp());
}