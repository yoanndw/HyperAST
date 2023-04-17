use hyper_ast::types::Typed;
use hyper_ast_gen_ts_java::legion_with_refs::JavaTreeGen;
use hyper_ast_metrics::{utils::hyper_ast_from_str, walk::HyperAstWalkIter};

fn main() {
    let case = r#"while(true) {p();} /* */ do {} while(false);"#;

    let tree = JavaTreeGen::tree_sitter_parse(case.as_bytes()).unwrap_or_else(|t| t);

    println!("{}", tree.root_node().to_sexp());
    let ast = hyper_ast_from_str(case);
    for n in HyperAstWalkIter::new(ast.0, &ast.1) {
        let node_type = n.get_type();
        println!("{:?} -> {}", n.get_type(), node_type.is_expression() || node_type.is_identifier() || node_type.is_statement());
    }
}