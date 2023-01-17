use std::process::Command;

pub fn docker_exec(command: Vec<&str>) -> String {
    println!("Executing command docker {:?}", command);
    let output = Command::new("docker")
        .args(command)
        .output()
        .expect("failed to execute process");

    let rc = output.status.code().unwrap();
    if rc != 0 {
        println!("Command stderr: {}", String::from_utf8_lossy(&output.stderr).to_string());
    }

    String::from_utf8_lossy(&output.stdout).to_string()
}