use std::collections::VecDeque;

use hyper_ast::{
    types::{Type, Typed, WithChildren, Tree, IterableChildren, Labeled}, tree_gen::parser::{Node, TreeCursor}, store::SimpleStores, store::TypeStore, store::{nodes::DefaultNodeStore as NodeStore, defaults::NodeIdentifier}, store::labels::LabelStore, position::{TreePath, StructuralPosition}
};

use hyper_ast_gen_ts_java::legion_with_refs::{
    print_tree_ids, print_tree_syntax, print_tree_syntax_with_ids, JavaTreeGen, TTreeCursor
};


static CASE_YOANN1: &'static str = r#"if (1 < 2) {
    f();
    while (true) {}
} else {
    while (1 == 1)
        g();
}"#;

static CASE_YOANN2: &'static str = r#"(1 < 2)"#;

fn main() {
    let case = CASE_YOANN1;
    let tree = JavaTreeGen::tree_sitter_parse(case.as_bytes()).unwrap_or_else(|t| t);

    println!("{}", tree.root_node().to_sexp());

    println!("===========");

    let stores = Box::new(SimpleStores {
        label_store: LabelStore::new(),
        type_store: TypeStore {},
        node_store: NodeStore::new(),
    });
    let md_cache = Box::new(Default::default());
    let mut java_tree_gen = JavaTreeGen::new(Box::leak(stores), Box::leak(md_cache));

    let full_node = java_tree_gen.generate_file(b"", case.as_bytes(), tree.walk());
    let root = full_node.local.compressed_node as NodeIdentifier;

    //walk_hast(java_tree_gen.stores, &StructuralPosition::new(root), root);
    walk_imp(java_tree_gen.stores, &StructuralPosition::new(root), root);
}

type StackElement = StructuralPosition;
fn walk_imp(stores: &SimpleStores, path: &StructuralPosition, root: NodeIdentifier) {
    let mut stack: VecDeque<StackElement> = VecDeque::new();
    stack.push_back(path.clone());

    while !stack.is_empty() {
        let top = stack.pop_back().unwrap();
        
        let node = top.node().unwrap();
        let node_ref = stores.node_store.resolve(*node);
        println!("Walk type: {:?}", node_ref.get_type());

        if node_ref.has_children() {
            for c in node_ref.children().unwrap().iter_children().rev() {
                stack.push_back(StructuralPosition::new(*c));
            }
        }
    }
}
