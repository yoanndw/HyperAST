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
    }
}