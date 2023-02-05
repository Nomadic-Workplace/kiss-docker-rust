use async_process::Command;

use crate::error;
use crate::error::KissDockerError::{CommandTerminatedUnexpectedly, DockerCommandFailed};

pub async fn docker_exec(command: Vec<&str>) -> error::Result<String> {
    let output = Command::new("docker").args(command).output().await?;

    let rc = output.status.code().ok_or(CommandTerminatedUnexpectedly)?;
    if rc != 0 {
        return Err(DockerCommandFailed {
            failure: String::from_utf8_lossy(&output.stderr).parse().unwrap(),
        });
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use crate::command::docker_exec;
    use crate::error::KissDockerError::DockerCommandFailed;

    #[tokio::test]
    async fn test_failed_command() {
        let r = docker_exec(vec!["this", "does", "not", "exist"]).await;
        assert!(matches!(r, Err(DockerCommandFailed { failure: _ })));
    }
}
