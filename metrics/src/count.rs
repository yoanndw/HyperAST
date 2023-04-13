use crate::walk::HyperAstWalkIter;
use hyper_ast::types::{Type, Typed};

pub fn countWhileStatements(iter: HyperAstWalkIter) -> usize {
    let mut cntWhile : usize = 0;
    for n in iter {
        dbg!(n.get_type());
        if n.get_type() == Type::WhileStatement {
            cntWhile += 1;
        }
    }

    cntWhile
}

pub fn count_nodes(iter: HyperAstWalkIter) -> usize {
    iter
        .filter(|n| {
            let node_type = n.get_type();
            node_type.is_expression() || node_type.is_identifier() || node_type.is_statement() 
        })
        .count()
}
