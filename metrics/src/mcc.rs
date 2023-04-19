use crate::walk::HyperAstWalkIter;
use hyper_ast::cyclomatic::{Mcc, MetaData};
use hyper_ast::store::{nodes::DefaultNodeIdentifier as NodeIdentifier, SimpleStores};
use hyper_ast::types::{IterableChildren, Labeled, Tree, Type, Typed, WithChildren};

pub fn cyclomatic_complexity(hyper_ast: (&SimpleStores, NodeIdentifier)) -> u32 {
    let node_ref = hyper_ast.0.node_store.resolve(hyper_ast.1);
    let mcc_res = Mcc::retrieve(&node_ref);
    mcc_res.unwrap_or(0)
}

#[cfg(test)]
mod test {
    mod from_str {
        use crate::{
            count::{count_instanceofs, count_numbers, count_nodes, count_while_statements},
            utils::hyper_ast_from_str, mcc::cyclomatic_complexity,
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
            empty_str,
            "",
            cyclomatic_complexity,
            1
        );

        make_test!(
            one_if,
            "if () {}",
            cyclomatic_complexity,
            2
        );

        make_test!(
            if_else,
            "if () {} else {}",
            cyclomatic_complexity,
            2
        );

        make_test!(
            if_elseif,
            "if () {} else if () {}",
            cyclomatic_complexity,
            3
        );

        make_test!(
            if_elseif_else,
            "if () {} else if () {} else {}",
            cyclomatic_complexity,
            3
        );

        make_test!(
            if_if,
            "if () {} if () {}", 
            cyclomatic_complexity,
            3
        );

        make_test!(
            if_if_nested,
            r#"if () {
                if () {}
            }"#, 
            cyclomatic_complexity,
            3
        );

        make_test!(
            one_while,
            r#"while () {} "#, 
            cyclomatic_complexity,
            2
        );

        make_test!(
            while_while,
            r#"while () {} while () {}"#, 
            cyclomatic_complexity,
            3
        );

        make_test!(
            while_while_nested,
            r#"while () {
                while () {}
            } "#, 
            cyclomatic_complexity,
            3
        );

        make_test!(
            one_dowhile,
            r#"do {} while (); "#, 
            cyclomatic_complexity,
            2
        );

        make_test!(
            dowhile_dowhile,
            r#"do {} while () ;
            do {} while();"#, 
            cyclomatic_complexity,
            3
        );

        make_test!(
            dowhile_dowhile_nested,
            r#"do {
                do {

                } while();
            } while ();"#, 
            cyclomatic_complexity,
            3
        );

        make_test!(
            switch_no_case,
            "switch (x) {}",
            cyclomatic_complexity,
            1
        );

        make_test!(
            switch_one_default,
            "switch (x) {default: break;}",
            cyclomatic_complexity,
            2
        );

        make_test!(
            switch_one_case,
            "switch (x) {case 1: break;}",
            cyclomatic_complexity,
            2
        );

        make_test!(
            switch_case_and_default,
            "switch (x) {
                case 1: break;
                default: break;
            }",
            cyclomatic_complexity,
            3
        );

        make_test!(
            switch_one_case_no_break_default,
            "switch (x) {
                case 1:
                default: break;
            }",
            cyclomatic_complexity,
            3
        );

        make_test!(
            switch_two_cases,
            "switch (x) {
                case 1: break;
                case 2: break;
            }",
            cyclomatic_complexity,
            3
        );

        make_test!(
            switch_two_cases_and_default,
            "switch (x) {
                case 1: break;
                case 2: break;
                default: break;
            }",
            cyclomatic_complexity,
            4
        );

        make_test!(
            switch_two_cases_break_at_end,
            "switch (x) {
                case 1: 
                case 2: break;
            }",
            cyclomatic_complexity,
            3
        );

        make_test!(
            switch_two_cases_break_at_end_and_default,
            "switch (x) {
                case 1: 
                case 2: break;
                default: break;
            }",
            cyclomatic_complexity,
            4
        );

        make_test!(
            switch_two_cases_same_value,
            "switch (x) {
                case 1: break;
                case 1: break;
            }",
            cyclomatic_complexity,
            3
        );

        make_test!(
            empty_main,
            r#"class M {
                public static void main(String[] args) {
                }
            }"#,
            cyclomatic_complexity,
            1
        );

        make_test!(
            if_in_main,
            r#"class M {
                public static void main(String[] args) {
                    if (true) {}
                }
            }"#,
            cyclomatic_complexity,
            2
        );

        make_test!(
            if_else_in_main,
            r#"class M {
                public static void main(String[] args) {
                    if (true) {}
                    else {}
                }
            }"#,
            cyclomatic_complexity,
            2
        );

        make_test!(
            switch_in_main,
            r#"class M {
                public static void main(String[] args) {
                    switch(x) {
                        case 1: break;
                        case 2: break;
                        case 3: break;
                        default: System.out.println("default");
                    }
                }
            }"#,
            cyclomatic_complexity,
            5
        );

        make_test!(
            while_in_main,
            r#"class M {
                public static void main(String[] args) {
                    while () {

                    }
                }
            }"#,
            cyclomatic_complexity,
            2
        );

        make_test!(
            while_while_in_main,
            r#"class M {
                public static void main(String[] args) {
                    while () {

                    }
                    while() {}
                }
            }"#,
            cyclomatic_complexity,
            3
        );

        make_test!(
            while_while_nested_in_main,
            r#"class M {
                public static void main(String[] args) {
                    while () {
                        while() {}
                    }
                }
            }"#,
            cyclomatic_complexity,
            3
        );

        make_test!(
            dowhile_dowhile_in_main,
            r#"class M {
                public static void main(String[] args) {
                    do {

                    }while () ;

                    do {} while () ;
                }
            }"#,
            cyclomatic_complexity,
            3
        );

        make_test!(
            dowhile_dowhile_nested_in_main,
            r#"class M {
                public static void main(String[] args) {
                    do {
                        do {} while() ;
                    }while () ;
                }
            }"#,
            cyclomatic_complexity,
            3
        );

        make_test!(
            throw_in_main,
            r#"class M {
                public static void main(String[] args) {
                    throw new Exception("e");
                }
            }"#,
            cyclomatic_complexity,
            1
        );

        make_test!(
            catch_in_main,
            r#"class M {
                public static void main(String[] args) {
                    try {}
                    catch (Exception e) {}
                }
            }"#,
            cyclomatic_complexity,
            3
        );

        make_test!(
            two_catches_in_main,
            r#"class M {
                public static void main(String[] args) {
                    try {}
                    catch (IOException e) {}
                    catch (Exception e) {}
                }
            }"#,
            cyclomatic_complexity,
            4
        );

        make_test!(
            normal_case_two_methods,
            r#"class M {
                public static void main(String[] args) {
                    try {}
                    catch (IOException e) {}
                }
                
                static void m() {
                    if (true) {}
                    else {}
                }
            }"#,
            cyclomatic_complexity,
            4
        );

        make_test!(
            normal_case_two_methods_with_call,
            r#"class M {
                public static void main(String[] args) {
                    try {
                        m();
                    }
                    catch (IOException e) {}
                }
                
                static void m() {
                    if (true) {}
                    else {}
                }
            }"#,
            cyclomatic_complexity,
            4
        );

        make_test!(
            method_call,
            r#"class M {
                public static void main(String[] args) {
                    p();
                }
            }"#,
            cyclomatic_complexity,
            1
        );
    }
}