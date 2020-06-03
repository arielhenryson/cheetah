mod cmd;

fn main() {
    std::process::exit(
        match cmd::main::execute() {
            Ok(_) => 0,
            Err(_err) => 1
        }
    );
}
