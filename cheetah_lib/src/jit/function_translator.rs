use cranelift::prelude::*;
use cranelift_simplejit::{ SimpleJITBackend };
use cranelift_module::{ Module };

use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::constant::Const;

pub struct FunctionTranslator<'a> {
    pub int: types::Type,
    pub builder: FunctionBuilder<'a>,
    pub module: &'a mut Module<SimpleJITBackend>,
    pub is_return: bool
}

impl<'a>FunctionTranslator<'a> {
    pub fn translate_expr(&mut self, expression: Expression) -> Value {
        match expression.def {
            ExpressionDef::Return(ref node)
            => self.return_gen(node),

            ExpressionDef::Block(nodes)
            => self.block_gen(nodes),

            ExpressionDef::Const(
                Const::Int(num)
            ) => self.number_gen(num),

            // Placeholder for machine code
            ExpressionDef::Const(
                Const::Undefined
            ) => self.number_gen(0),

            // Placeholder for machine code
            ExpressionDef::Assign(ref _expr, ref _expr2)
            => self.number_gen(0),

            // Placeholder for machine code
            ExpressionDef::Identifier(ref _name, ref _debug_info)
              => self.number_gen(0),

            ExpressionDef::BinOp(ref op, ref a, ref b)
            => self.binary_expression_gen(a, op, b),

            // Placeholder for machine code
            ExpressionDef::FunctionDecl(ref _name, ref _args, ref _expr)
            => self.number_gen(0),

            // Placeholder for machine code
            ExpressionDef::Call(ref _name, ref _args, ref _debug_info)
            => self.number_gen(0),

            // Placeholder for machine code
            ExpressionDef::VariableDecl(ref _name, ref _expr)
            => self.number_gen(0),

            // Placeholder for machine code
            ExpressionDef::ArrayDecl(ref _items)
            => self.number_gen(0),

            // Placeholder for machine code
            ExpressionDef::If(ref _condition, ref _expression, ref _else)
            => self.number_gen(0),

            // Placeholder for machine code
            ExpressionDef::Member(ref _value, ref _member)
            => self.number_gen(0),
        }
    }
}