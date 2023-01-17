use std::process::Command;

pub fn docker_exec(command: &str) -> String {
    println!("Executing command docker {}", command);
    let output = Command::new("docker")
        .arg(command)
        .output()
        .expect("failed to execute process");

    String::from_utf8_lossy(&output.stdout).to_string()
}