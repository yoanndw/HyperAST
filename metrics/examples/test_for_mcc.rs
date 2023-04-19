use hyper_ast::types::{Typed, HyperAST};
use hyper_ast_gen_ts_java::legion_with_refs::{JavaTreeGen, print_tree_ids, print_tree_structure, print_tree_syntax, print_tree_labels};
use hyper_ast_metrics::{utils::hyper_ast_from_str, walk::HyperAstWalkIter};

fn main() {
    let case = r#"if () {} if () { if () {} } else {}"#;

    let tree = JavaTreeGen::tree_sitter_parse(case.as_bytes()).unwrap_or_else(|t| t);

    println!("{}", tree.root_node().to_sexp());
    let ast = hyper_ast_from_str(case);

    println!("\n\nSTRUCTURE-------\n");
    print_tree_structure(ast.0.node_store(), &ast.1);
    println!("\n\nSYNTAX-------\n");
    print_tree_syntax(ast.0.node_store(), ast.0.label_store(), &ast.1);
    println!("\n\nLABELS-------\n");
    print_tree_labels(ast.0.node_store(), ast.0.label_store(), &ast.1);

    //for n in HyperAstWalkIter::new(ast.0, &ast.1) {
    //    let node_type = n.get_type();
    //    //println!("{:?} -> {}", n.get_type(), node_type.is_expression() || node_type.is_identifier() || node_type.is_statement());
    //    println!("{:?}", n.get_type());
    //}
}
