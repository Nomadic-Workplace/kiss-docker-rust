extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use crate::command;

/*
API features:
- Specify volumes (hashmap)
- Specify environment variables (hashmap) IN PROGRESS
- Specify custom command

Starting a container should allow specifying repo + tag instead of just image


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
    pub cmd: String
}

pub trait ContainerImpl {
    fn start(&self) -> String;
    fn start_blocking(&self) -> String;
    fn stop(&self, id: String) -> String;
    fn list_running(&self) -> String;
    fn get_image(&self) -> String;
    fn get_env(&self) -> String;
    fn get_volumes(&self) -> String;
    fn get_cmd(&self) -> String;
}

impl ContainerImpl for Container {
    fn start(&self) -> String {
        command::docker_exec(vec!["run", "-d", self.get_env().as_str(), self.get_image().as_str()])
    }

    fn start_blocking(&self) -> String {
        command::docker_exec(vec!["run", "-a", self.get_image().as_str()])
    }

    fn stop(&self, id: String) -> String {
        command::docker_exec(vec!["stop", id.as_str()])
    }

    fn list_running(&self) -> String {
        command::docker_exec(vec!["ps", "-a", "-f", "status=running"])
    }

    fn get_image(&self) -> String {
        let mut cmd = self.repo.clone();
        if !self.tag.is_empty() {
            cmd.push_str(format!(":{}", self.tag).as_str());
        }
        cmd
    }

    fn get_env(&self) -> String {
        // Take the hashmap and convert it to a series of -e $KEY=$VALUE
        let mut env = "".to_string();
        for(key, value) in &self.env {
            env.push_str(format!(" -e {}={}", key, value).as_str());
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
