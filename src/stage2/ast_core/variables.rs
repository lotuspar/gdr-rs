use crate::stage2::ast_core::{Expr, Val};

pub struct VariableDefinition {
    pub identifier: Val,
    pub type_hint: Val,
    pub value: Expr,
    pub constant: Val,
}

pub struct VariableRef {
    pub identifier: Val,
}