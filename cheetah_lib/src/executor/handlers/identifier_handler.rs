use crate::executor::executor::{ Executor, ExecutorResults };
use crate::types::expression::DebugInfo;

impl Executor {
    pub fn identifier_handler(&mut self, name: &String, debug_info: &DebugInfo) -> ExecutorResults {
        return match self.env.get(name) {
            Ok(val) => Ok(*val),
            Err(()) => {
                let (loc, _line, _column) = self.error_handler.error_loc(debug_info.start);
                eprintln!("Error: `{}` is not defined {}", name, loc);
                self.undefined_handler()
            }
        }
    }
}