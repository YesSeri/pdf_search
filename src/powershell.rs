use std::process::{Command, Output};

// pub fn run_powershell_command(command: &str) -> String {
//     let mut powershell = Command::new("powershell.exe");
//     let command = powershell.args(["-Command", command]);
//     let output = command.output().unwrap();
//     let result = output.stdout;
//     let string = String::from_utf8_lossy(&result).to_string();
//     string.trim().to_string()
// }

pub fn run_powershell_command(cmd_string: &str) -> std::io::Result<Output> {
    let mut powershell = Command::new("powershell.exe");
    let command = powershell.args(["-Command", cmd_string]);
    command.output()
}

pub fn get_stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_powershell_command() {
        let output = run_powershell_command("echo 'aaa\nbbb'").expect("Could not run command.");
        let result_string = get_stdout(&output);
        assert_eq!("aaa\nbbb", result_string);
    }
}