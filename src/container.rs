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
}

pub trait ContainerImpl {
    fn start(&self) -> String;
    fn start_blocking(&self) -> String;
    fn stop(&self, id: String) -> String;
    fn list_running(&self) -> String;
    fn get_image(&self) -> String;
    fn get_env(&self) -> Vec<String>;
    fn get_volumes(&self) -> String;
    fn get_cmd(&self) -> String;
}

impl ContainerImpl for Container {
    fn start(&self) -> String {
        let mut cmd = vec!["run"];
        let img = self.get_image();
        let ops = vec!["-d", img.as_str()];
        let e = self.get_env();
        let env: Vec<&str> = e.iter().map(|s| s.as_str()).collect();

        // Order matters to docker
        cmd.extend(env);
        cmd.extend(ops);

        command::docker_exec(cmd).unwrap()
    }

    fn start_blocking(&self) -> String {
        command::docker_exec(vec!["run", "-a", self.get_image().as_str()]).unwrap()
    }

    fn stop(&self, id: String) -> String {
        command::docker_exec(vec!["stop", id.as_str()]).unwrap()
    }

    fn list_running(&self) -> String {
        command::docker_exec(vec!["ps", "-a", "-f", "status=running"]).unwrap()
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

    fn get_volumes(&self) -> String {
        "".to_string()
    }

    fn get_cmd(&self) -> String {
        "".to_string()
    }
}
