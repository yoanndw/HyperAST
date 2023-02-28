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

    do {} while(x);
}"#;

// NOTE: do {} while() == do_statement sur tree sitter

fn main() {
    let tree = JavaTreeGen::tree_sitter_parse(CASE_YOANN3.as_bytes()).unwrap_or_else(|t| t);

    println!("{}", tree.root_node().to_sexp());

    println!("===========");

    let mut cursor = tree.walk();
    let count = find_while(&mut cursor, 0);
    println!("{}", count);
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

fn find_while(cursor: &mut tree_sitter::TreeCursor, mut while_count: i32) -> i32 {
    if cursor.node().kind() == "while_statement" {
        println!("Node: {}", cursor.node().kind());
        while_count = 1;
    } else {
        while_count = 0;
    }


    if cursor.goto_first_child() {
        while_count += find_while(cursor, while_count);
    } 

    while cursor.goto_next_sibling() {
        while_count += find_while(cursor, while_count);
    } 
    cursor.goto_parent();
    
    while_count
}
