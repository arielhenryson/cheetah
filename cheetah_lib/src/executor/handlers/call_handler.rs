use std::ops::Deref;

use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::expression::{ Expression, ExpressionDef };
use crate::types::expression::DebugInfo;
use crate::types::values::Value;
use crate::types::function::Function;

impl Executor {
    pub fn call_handler(&mut self, name: &String, args: &Vec<Expression> ,debug_info: &DebugInfo) -> ExecutorResults {
        let res = match self.env.get_scope(None).records.clone().get(name) {
            Some(value) => {
                self.ensure_func(*value.clone(), name.clone(), debug_info);

                let (params, block) = self.get_params_and_block(*value.clone());

                self.env.crate_scope(name.clone());

                let _v = self.set_args(params, args);

                let val = self.run(block)?;

                Ok(val)
            },
            None => {
                let (loc, _line, _column) = self.error_handler.error_loc(debug_info.start);
                eprintln!("Error: no function with the name {} {}", name, loc);
                std::process::exit(1);
            }
        };

        self.is_return = false;

        res
    }

    // Make sure that what we call is actually a function
    fn ensure_func(&mut self, value: Value, name: String, debug_info: &DebugInfo) {
        match value {
            Value::Function(ref _func) => {}
            _ => {
                let (loc, _line, _column) = self.error_handler.error_loc(debug_info.start);
                eprintln!("Error: `{}` is not a function {}", name, loc);
                std::process::exit(1);
            }
        }
    }

    // Push the arguments to the corresponding parameters of the function
    fn set_args(&mut self, params: Vec<Expression>, args: &Vec<Expression>) -> Result<(), Value> {
        let mut index = 0;
        
        for parm in params {
            match parm.def {
                ExpressionDef::Identifier(ref name, _) => {
                    let op_expr = args.get(index);
                    match op_expr {
                        // Set the argument to the corresponding parameter
                        Some(expr) => {
                            let val = self.run(expr.clone())?;
                            self.env.push(name, Box::new(val))
                        }

                        // If there is no value in this index
                        // there is no more arguments so we can break
                        _ => break
                    }
                }
                _ => {}
            }
            
            index += 1;
        }

        Ok(())
    }

    // helper function to extract the parameters and
    // the code block from the function expression
    fn get_params_and_block(&mut self, value: Value) -> (Vec<Expression>, Expression) {
        match value {
            Value::Function(ref inner_func) => {
                match *inner_func.deref() {
                    Function::RegularFunc(ref data) => (data.args.clone(), data.expression.clone())
                }
            },
            _ => std::process::exit(1)
        }
    }
}