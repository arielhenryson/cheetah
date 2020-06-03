use std::mem;

// clap is Command Line Argument we use to pass data to the engine
use super::arguments;

// use for read the source code to string
use super::reader;

// the engine lexer and the parser
use cheetah_lib::{ lexer, parser };

// the engine AST Executor
use cheetah_lib::executor::executor::Executor;

// the engine JIT
use cheetah_lib::jit::jit::JIT;
use cheetah_lib::error_handler::source_code::SourceCode;
use cheetah_lib::error_handler::error_handler::ErrorHandler;

// the main pipeline of the compiler
pub fn execute() -> Result<(), String> {
    // create the cli tool
    let matches =  arguments::app_arguments();

    let file_path = matches.value_of("FILE").unwrap();

    let source_code =  SourceCode::new(
        // the file path that we want to work on
        String::from(file_path),

        // read to source code to a string
        reader::read_source_code(file_path)
    );

    // create the program tokens
    let tokens = lexer::lex::lex_code(
        &source_code.content.clone()
    )?;

    let error_handler = ErrorHandler::new(
        source_code
    );

    // println!("{:#?}", tokens);

    // create the program AST
    let ast = parser::parser::Parser::new(
        tokens,
        error_handler.clone()
    ).start()?;

    println!("{:#?}", ast);

    // Execute the code
    let executor_value = Executor::new(
        error_handler
    ).start(
        ast.clone()
    );

    // run the JIT
    let _jit = JIT::new().compile(ast).unwrap();

    unsafe { mem::transmute::<_, fn(isize, isize) -> isize>(_jit) };

    // println!("the JIT value is: {}", foo(1, 0));

    println!("{:#?}", executor_value);

    Ok(())
}
