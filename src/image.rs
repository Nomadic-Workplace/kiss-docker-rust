use crate::command::docker_exec;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct ImageMetadata {
    name: String,
    tag: String,
}

pub fn list_images() -> String {
    docker_exec(vec!["images"])
}

pub fn find_by_tag(component: &str, tag: &str) -> String {
    let output = docker_exec(vec!["images", "--format", "'{{json .}}'"]);
    let images: Vec<ImageMetadata> = serde_json::from_str(&output).expect("Failed to parse JSON");

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
