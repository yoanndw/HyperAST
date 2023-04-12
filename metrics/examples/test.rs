use hyper_ast_gen_ts_java::legion_with_refs::JavaTreeGen;

fn main() {
    let case = r#"call(1);"#;

    let tree = JavaTreeGen::tree_sitter_parse(case.as_bytes()).unwrap_or_else(|t| t);

    println!("{}", tree.root_node().to_sexp());
}