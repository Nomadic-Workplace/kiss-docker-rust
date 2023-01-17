extern crate serde;
extern crate serde_json;

use std::process::Command;

use serde::Deserialize;

/*
API features:
- Specify volumes
- Specify environment variables
- Specify custom command

Starting a container should allow specifying repo + tag instead of just image


TODO: Vegi requirements:
And collect stdout
Plus some basic error handling
Basically docker run --rm

I need a blocking action that returns all the logs when the container terminates

 */

#[derive(Deserialize, Debug)]
struct Image {
    name: String,
    tag: String,
}

pub struct DockerManager {
    pub baseline_sha: String,
    pub new_sha: String,
}

pub trait DockerImpl {
    fn start_container(&self, container_name: &str) -> String;
    fn stop_container(&self, container_name: &str) -> String;
    fn list_containers(&self) -> String;
    fn list_images(&self) -> String;
    fn find_tag(&self, component: &str, tag: &str) -> String;
    fn docker_exec(&self, command: &str) -> String {
        let output = Command::new("docker")
            .arg(command)
            .output()
            .expect("failed to execute process");

        String::from_utf8_lossy(&output.stdout).to_string()
    }
}

impl DockerImpl for DockerManager {
    fn start_container(&self, container_name: &str) -> String {
        self.docker_exec(&*format!("start -a {}", container_name))
    }

    fn stop_container(&self, container_name: &str) -> String {
        self.docker_exec(&*format!("stop -t 0 {}", container_name))
    }

    fn list_containers(&self) -> String {
        self.docker_exec("ps -a -f status=running")
    }

    fn list_images(&self) -> String {
        self.docker_exec("images")
    }

    fn find_tag(&self, component: &str, tag: &str) -> String {
        let output = self.docker_exec("images --format '{{json .}}'");
        let images: Vec<Image> = serde_json::from_str(&output).expect("Failed to parse JSON");

        for image in images {
            if image.tag == tag && image.name.contains(component) {
                return image.name;
            }
        }
        format!(
            "Tag {} not found for component {}, please build manually and try again",
            tag, component
        )
    }
}
