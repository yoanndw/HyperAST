use crate::walk::HyperAstWalkIter;
use hyper_ast::store::{nodes::DefaultNodeIdentifier as NodeIdentifier, SimpleStores};
use hyper_ast::types::{Type, Typed};

use std::collections::HashSet;


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

        node_type.is_expression()
            || node_type.is_identifier()
            || node_type.is_statement()
            || node_type.is_type_declaration()
            || node_type.is_executable_member()
            || node_type.is_value_member()
    })
    .count()
}

pub fn evaluate_plagiarism(repositories: Vec<(&SimpleStores, NodeIdentifier)>) -> Vec<Vec<f64>> {
    
    let mut hashsets: Vec<_> = Vec::new();

    //transform all repos into hashmaps
    let mut repo_nb = 0;
    for n in repositories {

        let iter = HyperAstWalkIter::new(n.0, &n.1);
        let mut map = HashSet::new();

        let mut i = 0;
        for n in iter {
            map.insert(n.get_type());
            i += 1;
        }

        hashsets.push(map);

        repo_nb += 1;
    }
    
    //two dimension list to store result
    // value at index (a,b) is plagiarism level between repo at index a and repo at index b in original list
    let mut results = Vec::new();

    //calculate Jaccard coefficient between every repo
    let number_of_sets = hashsets.len();
    for i in 0..number_of_sets {

        let mut result = Vec::new();

        for j in 0..number_of_sets {

            //Jaccard coefficient calculation
            let intersection = hashsets[i].intersection(&hashsets[j]);
            let union = hashsets[i].union(&hashsets[j]);

            let intersection_size = intersection.size_hint().1.expect("");
            let union_size = union.size_hint().0;
            let mut ratio = intersection_size as f64 / union_size as f64;
            // rounding ratio as percentage : 0.3333333333 -> 33.33
            ratio = ratio * 10000.0;
            ratio = ratio.round();
            ratio = ratio / 100.0;

            /*
            println!("Comparing repos: {:?} - {:?}", i, j);
            println!("  repo{:?} : {:?}", i, hashsets[i]);
            println!("  repo{:?} : {:?}", j, hashsets[j]);
            println!("  intersection : {:?}", intersection);
            println!("  union : {:?}", union);
            println!("  intersection size : {:?}", intersection_size);
            println!("  union size: {:?}", union_size);
            println!("  ratio: {:?}%", ratio);
            println!("");
            */

            result.push(ratio);
        }

        results.push(result);
    }
    
    results

}

#[cfg(test)]
mod testplagiarism {
    mod from_str {
        use crate::{
            count::{evaluate_plagiarism},
            utils::hyper_ast_from_str,
        };

        macro_rules! make_test {
            ($test_name: ident, $java_code_1: tt, $java_code_2: tt, $function: ident, $expected: tt) => {
                #[test]
                fn $test_name() {
                    const CASE1: &str = $java_code_1;
                    const CASE2: &str = $java_code_2;

                    let ast1 = hyper_ast_from_str(CASE1);
                    let ast2 = hyper_ast_from_str(CASE2);
                    let param = vec![ast1, ast2];
                    assert_eq!($function(param), $expected);
                }
            };
        }

        make_test!(
            evaluate_plagiarism_identical_repo,
            r#"1"#,
            r#"1"#,
            evaluate_plagiarism,
            [[100.0, 100.0], [100.0, 100.0]]
        );

        make_test!(
            evaluate_plagiarism_empty_repo,
            r#""#,
            r#"1"#,
            evaluate_plagiarism,
            [[100.0, 33.33], [33.33, 100.0]]
        );

        make_test!(
            evaluate_plagiarism_different_repo,
            r#""#,
            r#"class Main {
                
                }
            }"#,
            evaluate_plagiarism,
            [[100.0, 11.11], [11.11, 100.0]]
        );
        
    }
}


#[cfg(test)]
mod test {
    mod from_str {
        use crate::{
            count::{count_nodes, count_while_statements},
            utils::hyper_ast_from_str,
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

        make_test!(count_whiles_empty_cond, r#"while () {}"#, count_while_statements, 1);

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

        make_test!(count_whiles_no_block_error, r#"while ()"#, count_while_statements, 0);

        make_test!(count_whiles_do_whiles_error_no_cond, "do {} while", count_while_statements, 0);

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

        make_test!(count_whiles_just_keyword_error, r#"while"#, count_while_statements, 0);

        make_test!(
            count_whiles_1_ok_1_error,
            r#"while () {
                while () // ERROR
            }"#,
            count_while_statements,
            1
        );

        make_test!(count_whiles_no_brackets, r#"while () p();"#, count_while_statements, 1);

        make_test!(count_nodes_one_number_counts_0, r#"1"#, count_nodes, 0);

        make_test!(count_nodes_binexp, r#"1 + 2"#, count_nodes, 1);

        make_test!(count_nodes_one_var, r#"x"#, count_nodes, 1);

        make_test!(count_nodes_binexp_two_vars, r#"x + y"#, count_nodes, 3);

        make_test!(count_nodes_call_no_param, r#"call();"#, count_nodes, 3);

        make_test!(count_nodes_call_one_param, r#"call(x);"#, count_nodes, 4);

        make_test!(count_nodes_call_one_param_number, r#"call(1);"#, count_nodes, 3);

        make_test!(count_nodes_call_binexp_numbers, r#"call(ab / cd)"#, count_nodes, 6);

        make_test!(count_nodes_break, r#"break;"#, count_nodes, 1);

        make_test!(count_nodes_return_void, r#"return;"#, count_nodes, 1);

        make_test!(count_nodes_return_number, r#"return 1;"#, count_nodes, 1);

        make_test!(count_nodes_return_var, r#"return x;"#, count_nodes, 2);

        make_test!(count_nodes_return_binexp_numbers, r#"return 1 + 2;"#, count_nodes, 2);

        make_test!(count_nodes_return_binexp_one_var, r#"return x + 1;"#, count_nodes, 3);

        make_test!(count_nodes_return_binexp_vars, r#"return x + y;"#, count_nodes, 4);

        make_test!(count_nodes_if_empty_cond_empty_block, r#"if () {}"#, count_nodes, 4);

        make_test!(count_nodes_if_cond_empty_block, r#"if (true) {}"#, count_nodes, 3);

        make_test!(count_nodes_if_empty_cond_block, r#"if () {p();}"#, count_nodes, 7);

        make_test!(count_nodes_if_cond_block, r#"if (true) {p();}"#, count_nodes, 6);

        // Lambda
        make_test!(
            count_nodes_lambda_no_arg_empty_block,
            "var l = () -> {};",
            count_nodes,
            5
        );

        make_test!(
            count_nodes_lambda_no_arg_number,
            "var l = () -> 5;",
            count_nodes,
            4
        );

        make_test!(
            count_nodes_lambda_one_arg_empty_block,
            "var l = (a) -> {};",
            count_nodes,
            6
        );

        make_test!(
            count_nodes_lambda_one_arg_number,
            "var l = (a) -> 5;",
            count_nodes,
            5
        );

        make_test!(
            count_nodes_lambda_one_arg_variable,
            "var l = (a) -> a;",
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

        // If, else if, else
        make_test!(
            count_nodes_if_elseif_cond_empty_else,
            r#"if () {} else if (true) {} else {}"#,
            count_nodes,
            8
        );

        make_test!(
            count_nodes_if_elseif_cond_else,
            r#"if () {} else if (true) {} else {f();}"#,
            count_nodes,
            11
        );

        make_test!(
            count_nodes_if_elseif_cond_block_empty_else,
            r#"if () {} else if (true) {p();} else {}"#,
            count_nodes,
            11
        );

        make_test!(
            count_nodes_if_elseif_cond_block_else,
            r#"if () {} else if (true) {p();} else {f();}"#,
            count_nodes,
            14
        );

        // While
        make_test!(count_nodes_while_empty_cond_empty_block, "while () {}", count_nodes, 4);

        make_test!(count_nodes_while_cond_empty_block, "while (true) {}", count_nodes, 3);

        make_test!(count_nodes_while_empty_cond_block, "while () {p();}", count_nodes, 7);

        make_test!(count_nodes_while_cond_block, "while (true) {p();}", count_nodes, 6);

        // Do while
        make_test!(count_nodes_dowhile_empty_cond_empty_block, "do {} while ();", count_nodes, 4);

        make_test!(count_nodes_dowhile_cond_empty_block, "do {} while (true);", count_nodes, 3);

        make_test!(count_nodes_dowhile_empty_cond_block, "do {p();} while ();", count_nodes, 7);

        make_test!(count_nodes_dowhile_cond_block, "do {p();} while (true);", count_nodes, 6);
        
        // Variables
        make_test!(
            count_nodes_int_a,
            "int a;",
            count_nodes,
            2
        );

        make_test!(
            count_nodes_int_a_5,
            "int a = 5;",
            count_nodes,
            2
        );

        make_test!(
            count_nodes_int_a_5_plus_2,
            "int a = 5 + 2;",
            count_nodes,
            3
        );

        make_test!(
            count_nodes_int_a_5_plus_x,
            "int a = 5 + x;",
            count_nodes,
            4
        );

        make_test!(
            count_nodes_int_a_x_plus_y,
            "int a = x + y;",
            count_nodes,
            5
        );
        
        // Methods
        make_test!(
            count_nodes_void_f_no_arg_empty_block,
            r#"void f() {}"#,
            count_nodes,
            3
        );
        
        make_test!(
            count_nodes_void_f_no_arg_block,
            r#"void f() {p();}"#,
            count_nodes,
            6
        );
        
        make_test!(
            count_nodes_void_f_one_arg_empty_block,
            r#"void f(int a) {}"#,
            count_nodes,
            4
        );
        
        make_test!(
            count_nodes_void_f_one_arg_block,
            r#"void f(int a) {p();}"#,
            count_nodes,
            7
        );
        
        make_test!(
            count_nodes_public_void_f_no_arg_empty_block,
            r#"public void f() {}"#,
            count_nodes,
            3
        );
        
        make_test!(
            count_nodes_private_void_f_no_arg_empty_block,
            r#"private void f() {}"#,
            count_nodes,
            3
        );
        
        make_test!(
            count_nodes_protected_void_f_no_arg_empty_block,
            r#"protected void f() {}"#,
            count_nodes,
            3
        );
        
        make_test!(
            count_nodes_static_void_f_no_arg_empty_block,
            r#"static void f() {}"#,
            count_nodes,
            3
        );

        // Class
        make_test!(
            count_nodes_empty_class,
            r#"class C {}"#,
            count_nodes,
            2
        );

        make_test!(
            count_nodes_class_internal_attr,
            r#"class C {
                int a;
            }"#,
            count_nodes,
            4
        );

        make_test!(
            count_nodes_class_static_internal_attr,
            r#"class C {
                static int a;
            }"#,
            count_nodes,
            4
        );

        make_test!(
            count_nodes_class_public_attr,
            r#"class C {
                public int b;
            }"#,
            count_nodes,
            4
        );

        make_test!(
            count_nodes_class_internal_method_no_arg_empty_block,
            r#"class C {
                void f() {}
            }"#,
            count_nodes,
            5
        );

        make_test!(
            count_nodes_class_private_method_no_arg_empty_block,
            r#"class C {
                private void g() {}
            }"#,
            count_nodes,
            5
        );

        make_test!(
            count_nodes_class_static_method_no_arg_empty_block,
            r#"class C {
                static void g() {}
            }"#,
            count_nodes,
            5
        );

        make_test!(
            count_nodes_class_internal_method_no_arg_block,
            r#"class C {
                void f() {p();}
            }"#,
            count_nodes,
            8
        );

        make_test!(
            count_nodes_class_private_method_no_arg_block,
            r#"class C {
                private void g() {p();}
            }"#,
            count_nodes,
            8
        );

        make_test!(
            count_nodes_class_static_method_no_arg_block,
            r#"class C {
                static void g() {p();}
            }"#,
            count_nodes,
            8
        );

        make_test!(
            count_nodes_class_internal_method_one_arg_empty_block,
            r#"class C {
                void f(int a) {}
            }"#,
            count_nodes,
            6
        );

        make_test!(
            count_nodes_class_private_method_one_arg_empty_block,
            r#"class C {
                private void g(int b) {}
            }"#,
            count_nodes,
            6
        );

        make_test!(
            count_nodes_class_static_method_one_arg_empty_block,
            r#"class C {
                static void g(int b) {}
            }"#,
            count_nodes,
            6
        );

        make_test!(
            count_nodes_class_internal_method_one_arg_block,
            r#"class C {
                void f(int a) {p();}
            }"#,
            count_nodes,
            9
        );

        make_test!(
            count_nodes_class_private_method_one_arg_block,
            r#"class C {
                private void g(int b) {p();}
            }"#,
            count_nodes,
            9
        );

        make_test!(
            count_nodes_class_static_method_one_arg_block,
            r#"class C {
                static void g(int b) {p();}
            }"#,
            count_nodes,
            9
        );

        make_test!(
            count_nodes_class_static_inner_class,
            r#"class C {
                static class D {

                }
            }"#,
            count_nodes,
            4
        );

        // Class instance
        make_test!(
            count_nodes_decl_class_instance,
            "B b;",
            count_nodes,
            3
        );

        make_test!(
            count_nodes_decl_type_inference,
            "var b;",
            count_nodes,
            3
        );
        
    }
}
