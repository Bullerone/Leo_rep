// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{Assigner, SymbolTable, TypeTable};

use leo_ast::{
    AccessExpression,
    ArrayAccess,
    ArrayExpression,
    ArrayType,
    BinaryExpression,
    BinaryOperation,
    Block,
    Expression,
    ExpressionReconstructor,
    Identifier,
    IntegerType,
    Literal,
    Member,
    MemberAccess,
    Node,
    NodeBuilder,
    NonzeroNumber,
    ReturnStatement,
    Statement,
    Struct,
    StructExpression,
    StructVariableInitializer,
    TernaryExpression,
    TupleAccess,
    TupleExpression,
    TupleType,
    Type,
};
use leo_span::Symbol;

use indexmap::IndexMap;

pub struct Destructurer<'a> {
    /// The symbol table associated with the program.
    pub(crate) symbol_table: &'a SymbolTable,
    /// A mapping between node IDs and their types.
    pub(crate) type_table: &'a TypeTable,
    /// A counter used to generate unique node IDs.
    pub(crate) node_builder: &'a NodeBuilder,
    /// A struct used to construct (unique) assignment statements.
    pub(crate) assigner: &'a Assigner,
    /// A mapping between variables and flattened tuple expressions.
    pub(crate) tuples: IndexMap<Symbol, TupleExpression>,
}

impl<'a> Destructurer<'a> {
    pub(crate) fn new(
        symbol_table: &'a SymbolTable,
        type_table: &'a TypeTable,
        node_builder: &'a NodeBuilder,
        assigner: &'a Assigner,
    ) -> Self {
        Self { symbol_table, type_table, node_builder, assigner, tuples: IndexMap::new() }
    }

    /// A wrapper around `assigner.unique_simple_assign_statement` that updates `self.structs`.
    pub(crate) fn unique_simple_assign_statement(&mut self, expr: Expression) -> (Identifier, Statement) {
        // Create a new variable for the expression.
        let name = self.assigner.unique_symbol("$var", "$");
        // Construct the lhs of the assignment.
        let place = Identifier { name, span: Default::default(), id: self.node_builder.next_id() };
        // Construct the assignment statement.
        let statement = self.simple_assign_statement(place, expr);

        (place, statement)
    }

    /// A wrapper around `assigner.simple_assign_statement` that tracks the type of the lhs.
    pub(crate) fn simple_assign_statement(&mut self, lhs: Identifier, rhs: Expression) -> Statement {
        // Update the type table.
        let type_ = match self.type_table.get(&rhs.id()) {
            Some(type_) => type_,
            None => unreachable!("Type checking guarantees that all expressions have a type."),
        };
        self.type_table.insert(lhs.id(), type_);
        // Construct the statement.
        self.assigner.simple_assign_statement(lhs, rhs, self.node_builder.next_id())
    }
}