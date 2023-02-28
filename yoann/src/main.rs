use hyper_ast::{
    types::{Type, Typed, WithChildren, Tree}, tree_gen::parser::{Node, TreeCursor},
};

use hyper_ast_gen_ts_java::legion_with_refs::{
    print_tree_ids, print_tree_syntax, print_tree_syntax_with_ids, JavaTreeGen, TTreeCursor
};


static CASE_YOANN1: &'static str = r#"if (1 < 2) {
    f();
} else {
    g();
}"#;

static CASE_YOANN2: &'static str = r#"(1 < 2)"#;

static CASE_YOANN3: &'static str = r#"while (true) {
    f();
    while (variab) {
        g();
    }
}"#;

fn main() {
    let tree = JavaTreeGen::tree_sitter_parse(CASE_YOANN3.as_bytes()).unwrap_or_else(|t| t);

    println!("{}", tree.root_node().to_sexp());

    println!("===========");

    let mut cursor = tree.walk();
    walk(&mut cursor);
}

fn walk(cursor: &mut tree_sitter::TreeCursor) {
    println!("Node: {}", cursor.node().kind());

    if cursor.goto_first_child() {
        walk(cursor);
    } 

    while cursor.goto_next_sibling() {
        walk(cursor);
    } 
    cursor.goto_parent();
    
}
