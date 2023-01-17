extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use crate::command;

/*
API features:
- Specify volumes (hashmap)
- Specify environment variables (hashmap)
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
}

impl ContainerImpl for Container {
    fn start(&self) -> String {
        // TODO: This needs to return the container ID from docker start stdout
        let id = command::docker_exec(&*format!("start -d {}:{}", self.repo, self.tag));
        println!("Started container with id: {}", id);
        id
    }

    fn start_blocking(&self) -> String {
        command::docker_exec(&*format!("start -a {}:{}", self.repo, self.tag))
    }

    fn stop(&self, id: String) -> String {
        command::docker_exec(&*format!("stop -t 0 {}", id))
    }

    fn list_running(&self) -> String {
        command::docker_exec("ps -a -f status=running")
    }

}
