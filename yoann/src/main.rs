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

fn main() {
    let tree = JavaTreeGen::tree_sitter_parse(CASE_YOANN1.as_bytes()).unwrap_or_else(|t| t);

    println!("{}", tree.root_node().to_sexp());

    println!("===========");

    let mut cursor = tree.walk();
    walk(&mut cursor);
}

fn walk(cursor: &mut tree_sitter::TreeCursor) {
    println!("{}", cursor.node().kind());
    
    
    if cursor.goto_first_child() {
        walk(cursor);
    } else if cursor.goto_next_sibling() {
        loop {
            walk(cursor);

            if !cursor.goto_next_sibling() {
                break;
            }
        }
    } else {
        cursor.goto_parent();
    }
}
