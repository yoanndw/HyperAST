use crate::walk::HyperAstWalkIter;
use hyper_ast::store::{nodes::DefaultNodeIdentifier as NodeIdentifier, SimpleStores};
use hyper_ast::types::{Type, Typed};

pub fn count_while_statements(hyper_ast: (&SimpleStores, NodeIdentifier)) -> usize {
    let iter = HyperAstWalkIter::new(hyper_ast.0, &hyper_ast.1);
    let mut count: usize = 0;
    for n in iter {
        if n.get_type() == Type::WhileStatement || n.get_type() == Type::DoStatement {
            count += 1;
        }
    }

    count
}

pub fn count_nodes(hyper_ast: (&SimpleStores, NodeIdentifier)) -> usize {
    let iter = HyperAstWalkIter::new(hyper_ast.0, &hyper_ast.1);
    iter.filter(|n| {
        let node_type = n.get_type();

        // println!("{:?}", node_type);

        node_type.is_expression() || node_type.is_identifier() || node_type.is_statement()
    })
    .count()
}

#[cfg(test)]
mod test {
    mod from_str {
        use crate::{
            count::{count_while_statements, count_nodes}, utils::hyper_ast_from_str,
        };

        macro_rules! make_test {
            ($test_name: ident, $java_code: tt, $function: ident, $expected: tt) => {
                #[test]
                fn $test_name() {
                    const CASE: &str = $java_code;

                    let ast = hyper_ast_from_str(CASE);
                    assert_eq!($function(ast), $expected);
                }
            };
        }

        make_test!(
            count_whiles_no_while,
            r#"class Main {
                public static void main(String[] args) {
                }
            }"#,
            count_while_statements,
            0
        );

        make_test!(
            count_whiles_one_while,
            r#"class Main {
                public static void main(String[] args) {
                    while (true) {}
                }
            }"#,
            count_while_statements,
            1
        );

        make_test!(
            count_whiles_nested_whiles,
            r#"class Main {
                public static void main(String[] args) {
                    while (true) {
                        while (false) {}
                    }
                }
            }"#,
            count_while_statements,
            2
        );

        make_test!(
            count_whiles_sequence_whiles,
            r#"class Main {
                public static void main(String[] args) {
                    while (1 == 1) {

                    }

                    while (2==3) {

                    }
                }
            }"#,
            count_while_statements,
            2
        );

        make_test!(
            count_whiles_sequence_and_nested_whiles,
            r#"class Main {
                public static void main(String[] args) {
                    while (1 == 1) {
                        while (true) {}
                        while (false) {}
                    }

                    while (2==3) {

                    }
                }
            }"#,
            count_while_statements,
            4
        );

        make_test!(
            count_whiles_do_while_counted,
            r#"class Main {
                public static void main(String[] args) {
                    do {} while (1);
                }
            }"#,
            count_while_statements,
            1
        );

        make_test!(
            count_whiles_nested_whiles_and_dos,
            r#"class Main {
                public static void main(String[] args) {
                    do {
                        while () {}
                    } while (1);
                }
            }"#,
            count_while_statements,
            2
        );

        make_test!(
            count_whiles_sequences_dos_and_whiles,
            r#"class Main {
                public static void main(String[] args) {
                    do {} while (1);
                    while (){}
                    do {} while (1);
                    while (){}
                }
            }"#,
            count_while_statements,
            4
        );

        make_test!(
            count_whiles_empty_cond,
            r#"while () {}"#,
            count_while_statements,
            1
        );

        make_test!(
            count_whiles_nested_just_whiles,
            r#"while () {
                while () {}
            }"#,
            count_while_statements,
            2
        );

        make_test!(
            count_whiles_sequence_just_whiles,
            r#"while () { }
            while () {}"#,
            count_while_statements,
            2
        );

        make_test!(
            count_whiles_sequence_and_nested_just_whiles,
            r#"while () { }
            while () {
                while (true) {}
            }"#,
            count_while_statements,
            3
        );

        make_test!(
            count_whiles_no_block_error,
            r#"while ()"#,
            count_while_statements,
            0
        );

        make_test!(
            count_whiles_do_whiles_error_no_cond,
            "do {} while",
            count_while_statements,
            0
        );

        make_test!(
            count_whiles_do_whiles_error_no_colon,
            "do {} while ()",
            count_while_statements,
            0
        );

        make_test!(
            count_whiles_do_whiles_one_instruction,
            "do print(x); while ();",
            count_while_statements,
            1
        );

        make_test!(
            count_whiles_just_keyword_error,
            r#"while"#,
            count_while_statements,
            0
        );

        make_test!(
            count_whiles_1_ok_1_error,
            r#"while () {
                while () // ERROR
            }"#,
            count_while_statements,
            1
        );

        make_test!(
            count_whiles_no_brackets,
            r#"while () p();"#,
            count_while_statements,
            1
        );

        make_test!(
            count_nodes_one_number_counts_0,
            r#"1"#,
            count_nodes,
            0
        );

        make_test!(
            count_nodes_binexp,
            r#"1 + 2"#,
            count_nodes,
            1
        );

        make_test!(
            count_nodes_one_var,
            r#"x"#,
            count_nodes,
            1
        );

        make_test!(
            count_nodes_binexp_two_vars,
            r#"x + y"#,
            count_nodes,
            3
        );

        make_test!(
            count_nodes_call_no_param,
            r#"call();"#,
            count_nodes,
            3
        );

        make_test!(
            count_nodes_call_one_param,
            r#"call(x);"#,
            count_nodes,
            4
        );

        make_test!(
            count_nodes_call_one_param_number,
            r#"call(1);"#,
            count_nodes,
            3
        );

        make_test!(
            count_nodes_call_binexp_numbers,
            r#"call(ab / cd)"#,
            count_nodes,
            6
        );

        make_test!(
            count_nodes_break,
            r#"break;"#,
            count_nodes,
            1
        );

        make_test!(
            count_nodes_return_void,
            r#"return;"#,
            count_nodes,
            1
        );

        make_test!(
            count_nodes_return_number,
            r#"return 1;"#,
            count_nodes,
            1
        );

        make_test!(
            count_nodes_return_var,
            r#"return x;"#,
            count_nodes,
            2
        );

        make_test!(
            count_nodes_return_binexp_numbers,
            r#"return 1 + 2;"#,
            count_nodes,
            2
        );

        make_test!(
            count_nodes_return_binexp_one_var,
            r#"return x + 1;"#,
            count_nodes,
            3
        );

        make_test!(
            count_nodes_return_binexp_vars,
            r#"return x + y;"#,
            count_nodes,
            4
        );

        make_test!(
            count_nodes_if_empty_cond_empty_block,
            r#"if () {}"#,
            count_nodes,
            4
        );

        make_test!(
            count_nodes_if_cond_empty_block,
            r#"if (true) {}"#,
            count_nodes,
            3
        );

        make_test!(
            count_nodes_if_empty_cond_block,
            r#"if () {p();}"#,
            count_nodes,
            7
        );

        make_test!(
            count_nodes_if_cond_block,
            r#"if (true) {p();}"#,
            count_nodes,
            6
        );

        // If else
        make_test!(
            count_nodes_if_empty_cond_empty_block_empty_else,
            r#"if () {} else {}"#,
            count_nodes,
            5
        );

        make_test!(
            count_nodes_if_empty_cond_empty_block_else,
            r#"if () {} else {p();}"#,
            count_nodes,
            8
        );

        make_test!(
            count_nodes_if_cond_empty_block_empty_else,
            r#"if (true) {} else {}"#,
            count_nodes,
            4
        );

        make_test!(
            count_nodes_if_cond_empty_block_else,
            r#"if (true) {} else {p();}"#,
            count_nodes,
            7
        );

        make_test!(
            count_nodes_if_empty_cond_block_empty_else,
            r#"if () {p();} else {}"#,
            count_nodes,
            8
        );

        make_test!(
            count_nodes_if_empty_cond_block_else,
            r#"if () {p();} else {p();}"#,
            count_nodes,
            11
        );

        make_test!(
            count_nodes_if_cond_block_empty_else,
            r#"if (true) {p();} else {}"#,
            count_nodes,
            7
        );

        make_test!(
            count_nodes_if_cond_block_else,
            r#"if (true) {p();} else {p();}"#,
            count_nodes,
            10
        );

        make_test!(
            count_nodes_if_elseif_empty_cond_empty_block,
            r#"if () {} else if () {}"#,
            count_nodes,
            8
        );

        make_test!(
            count_nodes_if_elseif_empty_cond_block,
            r#"if () {} else if () {p();}"#,
            count_nodes,
            11
        );

        make_test!(
            count_nodes_if_elseif_cond_empty_block,
            r#"if () {} else if (true) {}"#,
            count_nodes,
            7
        );

        make_test!(
            count_nodes_if_elseif_cond_block,
            r#"if () {} else if (true) {p();}"#,
            count_nodes,
            10
        );
    }
}
