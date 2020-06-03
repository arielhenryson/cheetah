use super::executor::Executor;
use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::constant::Const;
use crate::executor::executor::ExecutorResults;

impl Executor {
    pub fn run(&mut self, expression: Expression) -> ExecutorResults {
        return match expression.def {
            ExpressionDef::Return(ref node)
              => self.return_handler(node),

            ExpressionDef::Block(nodes)
              => self.block_handler(nodes),

            ExpressionDef::Const(
                Const::Int(num)
            )
              => self.number_handler(num),

            ExpressionDef::Const(
                Const::Undefined
            )
              => self.undefined_handler(),

            ExpressionDef::Assign(ref ref_expr, ref val_expr)
              => self.assign_handler(ref_expr, val_expr),

            ExpressionDef::Identifier(ref name, ref debug_info)
              => self.identifier_handler(name, debug_info),

            ExpressionDef::BinOp(ref op, ref a, ref b)
              => self.binary_expression_handler(a, op, b),

            ExpressionDef::FunctionDecl(ref name, ref args, ref expr)
              => self.function_handler(name, args, expr),

            ExpressionDef::Call(ref name, ref args, ref debug_info)
              => self.call_handler(name, args, debug_info),

            ExpressionDef::VariableDecl(ref name, ref expr)
              => self.variable_handler(name, expr),

            ExpressionDef::ArrayDecl(ref items)
              => self.array_handler(items),

            ExpressionDef::If(ref condition, ref expression, ref else_block)
              => self.if_handler(condition, expression, else_block),

            ExpressionDef::Member(ref _value, ref _member)
            => self.undefined_handler()
        }
    }
}