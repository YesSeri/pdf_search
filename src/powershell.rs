use std::process::Command;

pub fn run_powershell_command(command: &str) -> String {
    let mut powershell = Command::new("powershell.exe");
    let command = powershell.args(&["-Command", command]);
    let output = command.output().unwrap();
    let result = output.stdout;
    let string = String::from_utf8_lossy(&result).to_string();
    string.trim().to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_powershell_command() {
        let result = run_powershell_command("echo 'aaa\nbbb\n'");
        assert_eq!("aaa\nbbb", result);
    }
}