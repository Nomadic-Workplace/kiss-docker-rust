use crate::command;
use std::collections::HashMap;

use crate::models::RunningContainer;
use serde_json;

#[derive(Debug, Clone, Default)]
pub struct Container {
    pub repo: String,
    pub tag: String,
    pub volumes: Vec<String>,
    pub env: HashMap<String, String>,
    pub port_expose: usize,
    pub port_internal: usize,
    pub blocking: bool,

    /// command [arg...]
    pub ops: Vec<String>,
}

pub async fn stop_container(id: &str) -> String {
    command::docker_exec(vec!["stop", id]).await.unwrap()
}
pub async fn list_running(filter: Option<&str>) -> Vec<RunningContainer> {
    let lines = command::docker_exec(vec![
        "ps",
        "-a",
        "-f",
        "status=running",
        "--format",
        "{{json .}}",
    ])
    .await
    .unwrap();

    lines
        .split('\n')
        .filter(|s| !s.is_empty())
        .filter(|s| {
            if let Some(f) = &filter {
                s.contains(f)
            } else {
                true
            }
        })
        .map(|raw| serde_json::from_str(raw).unwrap())
        .collect()
}

impl Container {
    pub async fn start(&self) -> String {
        let mut cmd = vec!["run"];
        let img = self.get_image();
        let e = self.get_env();
        let env: Vec<&str> = e.iter().map(|s| s.as_str()).collect();

        if self.blocking {
            cmd.extend(vec!["--rm"]);
        } else {
            cmd.extend(vec!["-d"]);
        }

        let port_expose = self.port_expose.to_string();
        let port_internal = self.port_internal.to_string();
        let ports = format!("{}:{}", port_expose, port_internal);
        let ports_str = ports.as_str();

        if self.port_internal != 0 && self.port_expose != 0 {
            cmd.extend(vec!["-p", ports_str]);
        }

        if !self.volumes.is_empty() {
            for vol in &self.volumes {
                cmd.extend(vec!["-v", vol.as_str()]);
            }
        }

        if !self.env.is_empty() {
            cmd.extend(env);
        }

        cmd.extend(vec![img.as_str()]);

        cmd.extend(self.ops.iter().map(|s| s.as_str()));

        String::from(command::docker_exec(cmd).await.unwrap().trim())
    }

    pub fn get_image(&self) -> String {
        let mut cmd = self.repo.clone();
        if !self.tag.is_empty() {
            cmd.push_str(format!(":{}", self.tag).as_str());
        }
        cmd
    }

    pub fn get_env(&self) -> Vec<String> {
        let mut env: Vec<String> = vec![];
        for (key, value) in &self.env {
            env.push("-e".to_string());
            env.push(format!("{}={}", key, value));
        }
        env
    }
}

#[cfg(test)]
mod tests {
    use crate::container::{list_running, stop_container, Container};

    #[tokio::test]
    async fn test_list_running() {
        let r = list_running(None).await;
        println!("{:?}", r);
    }

    #[test]
    fn test_make_default() {
        let _ctn = Container {
            ..Default::default()
        };
    }

    #[tokio::test]
    async fn test_run_and_kill() {
        let ctn_id = Container {
            repo: String::from("alpine"),
            ..Default::default()
        }
        .start()
        .await;

        stop_container(&ctn_id).await;
    }

    #[tokio::test]
    async fn test_run_blocking() {
        let text = "test_text_print";

        let output = Container {
            repo: String::from("alpine"),
            ops: vec![String::from("echo"), String::from(text)],
            blocking: true,
            ..Default::default()
        }
        .start()
        .await;

        assert_eq!(text, output)
    }
}
