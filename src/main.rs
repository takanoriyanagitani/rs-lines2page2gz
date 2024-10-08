use std::io;

use std::process::ExitCode;

use rs_lines2page2gz::stdin2gz2stdout_env;

fn sub() -> Result<(), io::Error> {
    stdin2gz2stdout_env()?;
    Ok(())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
