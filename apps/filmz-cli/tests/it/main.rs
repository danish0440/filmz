mod cli;
mod workflows;

fn filmz_command() -> std::process::Command {
    let executable = std::env::var_os("NEXTEST_BIN_EXE_filmz_cli").unwrap_or_else(|| env!("CARGO_BIN_EXE_filmz-cli").into());
    std::process::Command::new(executable)
}

#[test]
fn cli_executable_is_resolved() {
    assert!(std::path::Path::new(filmz_command().get_program()).is_file());
}
