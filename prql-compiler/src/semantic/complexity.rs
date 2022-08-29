use anyhow::Result;

use super::*;
use crate::ast::ast_fold::{fold_transform, AstFold};
use crate::ast::*;

/// sets `node.is_complex` of some nodes in AST
#[allow(dead_code)]
pub fn determine_complexity(nodes: Vec<Expr>, context: &Context) -> Vec<Expr> {
    let mut d = DetermineComplex {
        is_complex: false,
        context,
    };

    d.fold_exprs(nodes).unwrap()
}

struct DetermineComplex<'a> {
    is_complex: bool,
    context: &'a Context,
}

impl<'a> AstFold for DetermineComplex<'a> {
    fn fold_expr(&mut self, mut node: Expr) -> Result<Expr> {
        node.kind = self.fold_expr_kind(node.kind)?;
        node.is_complex = self.is_complex;

        if !node.is_complex {
            if let Some(declared_at) = node.declared_at {
                let decl = self.context.declarations.decls[declared_at].0.clone();
                if let Declaration::Expression(expr) = decl {
                    let expr = self.fold_expr(*expr).unwrap();
                    node.is_complex = expr.is_complex;
                }
            }
        }

        Ok(node)
    }

    fn fold_exprs(&mut self, nodes: Vec<Expr>) -> Result<Vec<Expr>> {
        let res: Vec<_> = nodes
            .into_iter()
            .map(|node| {
                self.is_complex = false;
                self.fold_expr(node).unwrap()
            })
            .collect();

        self.is_complex = res.iter().any(|n| n.is_complex);
        Ok(res)
    }

    fn fold_transform(&mut self, transform: Transform) -> Result<Transform> {
        // fold only filter transforms (other don't need is_complex)
        match transform.kind {
            TransformKind::Filter(_) => fold_transform(self, transform),
            _ => Ok(transform),
        }
    }

    fn fold_type(&mut self, t: Ty) -> Result<Ty> {
        Ok(t)
    }

    fn fold_windowed(&mut self, windowed: Windowed) -> Result<Windowed> {
        self.is_complex = true;
        Ok(windowed)
    }
}
