extern crate serde;
extern crate serde_json;

use crate::command;
use std::collections::HashMap;

/*
API features:
- Specify volumes (hashmap)
- Specify environment variables (hashmap) IN PROGRESS
- Specify custom command

TODO: Vegi requirements:
And collect stdout
Plus some basic error handling
Basically docker run --rm

I need a blocking action that returns all the logs when the container terminates

 */

pub struct Container {
    pub repo: String,
    pub tag: String,
    pub volumes: HashMap<String, String>,
    pub env: HashMap<String, String>,
    pub cmd: String,
    pub port: usize,
    pub blocking: bool
}

pub trait ContainerImpl {
    fn start(&self) -> String;
    fn get_image(&self) -> String;
    fn get_env(&self) -> Vec<String>;
    fn get_port(&self) -> usize;
    fn get_volumes(&self) -> String;
    fn get_cmd(&self) -> String;
}

pub fn stop_container(id: String) -> String {
    command::docker_exec(vec!["stop", id.as_str()]).unwrap()
}

pub fn list_running() -> String {
    command::docker_exec(vec!["ps", "-a", "-f", "status=running"]).unwrap()
}

impl ContainerImpl for Container {

    fn start(&self) -> String {
        let mut cmd = vec!["run"];
        let img = self.get_image();
        let e = self.get_env();
        let env: Vec<&str> = e.iter().map(|s| s.as_str()).collect();
        if self.blocking {
            cmd.extend(vec!["-a", "-rm"]);
        } else {
            cmd.extend(vec!["-d"]);
        }
        cmd.extend(env);
        cmd.extend(vec![img.as_str()]);

        command::docker_exec(cmd).unwrap()
    }

    fn get_image(&self) -> String {
        let mut cmd = self.repo.clone();
        if !self.tag.is_empty() {
            cmd.push_str(format!(":{}", self.tag).as_str());
        }
        cmd
    }

    fn get_env(&self) -> Vec<String> {
        let mut env: Vec<String> = vec![];
        for (key, value) in &self.env {
            env.push("-e".to_string());
            env.push(format!("{}={}", key, value));
        }
        env
    }

    fn get_port(&self) -> usize {
        self.port
    }

    fn get_volumes(&self) -> String {
        "".to_string()
    }

    fn get_cmd(&self) -> String {
        "".to_string()
    }
}
