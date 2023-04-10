use crate::walk::HyperAstWalkIter;
use hyper_ast::types::{Type, Typed};

pub fn countWhileStatements(iter: HyperAstWalkIter) -> usize {
    let mut cntWhile: usize = 0;
    for n in iter {
        if n.get_type() == Type::WhileStatement || n.get_type() == Type::DoStatement {
            cntWhile += 1;
        }
    }

    cntWhile
}

pub fn count_nodes(iter: HyperAstWalkIter) -> usize {
    iter.filter(|n| {
        let node_type = n.get_type();
        node_type.is_expression() || node_type.is_identifier() || node_type.is_statement()
    })
    .count()
}

#[cfg(test)]
mod test {
    mod from_str {
        use crate::{
            count::{countWhileStatements, self}, utils::hyper_ast_from_str, walk::HyperAstWalkIter,
        };

        macro_rules! make_test {
            ($test_name: ident, $java_code: tt, $function: ident, $expected: tt) => {
                #[test]
                fn $test_name() {
                    const CASE: &str = $java_code;

                    let (s, n) = hyper_ast_from_str(CASE);
                    let iter = HyperAstWalkIter::new(s, &n);
                    assert_eq!($function(iter), $expected);
                }
            };
        }

        make_test!(
            count_whiles_no_while,
            r#"class Main {
                public static void main(String[] args) {
                }
            }"#,
            countWhileStatements,
            0
        );

        make_test!(
            count_whiles_one_while,
            r#"class Main {
                public static void main(String[] args) {
                    while (true) {}
                }
            }"#,
            countWhileStatements,
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
            countWhileStatements,
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
            countWhileStatements,
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
            countWhileStatements,
            4
        );

        make_test!(
            count_whiles_do_while_counted,
            r#"class Main {
                public static void main(String[] args) {
                    do {} while (1);
                }
            }"#,
            countWhileStatements,
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
            countWhileStatements,
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
            countWhileStatements,
            4
        );

        make_test!(
            count_whiles_empty_cond,
            r#"while () {}"#,
            countWhileStatements,
            1
        );

        make_test!(
            count_whiles_nested_just_whiles,
            r#"while () {
                while () {}
            }"#,
            countWhileStatements,
            2
        );

        make_test!(
            count_whiles_sequence_just_whiles,
            r#"while () { }
            while () {}"#,
            countWhileStatements,
            2
        );

        make_test!(
            count_whiles_sequence_and_nested_just_whiles,
            r#"while () { }
            while () {
                while (true) {}
            }"#,
            countWhileStatements,
            3
        );

        make_test!(
            count_whiles_no_block_error,
            r#"while ()"#,
            countWhileStatements,
            0
        );

        make_test!(
            count_whiles_do_whiles_error_no_cond,
            "do {} while",
            countWhileStatements,
            0
        );

        make_test!(
            count_whiles_do_whiles_error_no_colon,
            "do {} while ()",
            countWhileStatements,
            0
        );

        make_test!(
            count_whiles_do_whiles_one_instruction,
            "do print(x); while ();",
            countWhileStatements,
            1
        );

        make_test!(
            count_whiles_just_keyword_error,
            r#"while"#,
            countWhileStatements,
            0
        );

        make_test!(
            count_whiles_1_ok_1_error,
            r#"while () {
                while () // ERROR
            }"#,
            countWhileStatements,
            1
        );

        make_test!(
            count_whiles_no_brackets,
            r#"while () p();"#,
            countWhileStatements,
            1
        );
    }
}
