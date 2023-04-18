use crate::walk::HyperAstWalkIter;


pub fn evaluate_plagiarism(repositories: Vec<(&SimpleStores, NodeIdentifier)>) -> usize {
    
    let mut hashmaps = Vec::new();

    for n in repositories {
        let map = turnRepositoryIntoHashMap(n);
        hashmaps.push(map);
    }
    

}

pub fn turnRepositoryIntoHashSet(hyper_ast: (&SimpleStores, NodeIdentifier)) -> HashSet<CompressedNode<Entity, SymbolU32>> {
    let iter = HyperAstWalkIter::new(hyper_ast.0, &hyper_ast.1);
    let mut map = HashMap::new();

    let mut i = 0;
    for n in iter {
        map.insert(i, n);
        i += 1;
    }

    println!("{:?}", map);

    map
}

pub fn CalculateJaccardCoefficient(map1: HashMap, map2: HashMap) {

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
            turnRepositoryIntoHashSet_one_number,
            r#"1"#,
            turnRepositoryIntoHashSet,
            1
        );
    }
}