use crate::command::docker_exec;
use crate::error;
use crate::models::ImageSummary;

pub async fn list_images() -> error::Result<Vec<ImageSummary>> {
    let output = docker_exec(vec!["images", "--format", "{{json .}}"])
        .await
        .unwrap();
    let images: Vec<ImageSummary> = output
        .split('\n')
        .filter(|part| !part.is_empty())
        .map(serde_json::from_str)
        .collect::<Result<Vec<ImageSummary>, serde_json::Error>>()?;

    Ok(images)
}

pub async fn find_by_tag(component: &str, tag: &str) -> error::Result<Option<ImageSummary>> {
    let images = list_images().await?;

    for image in images {
        if image.tag == tag && image.repository.contains(component) {
            return Ok(Some(image));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use crate::image::{find_by_tag, list_images};

    #[tokio::test]
    async fn test_list_images() {
        list_images().await.unwrap();
    }

    #[tokio::test]
    async fn test_find_alpine() {
        let r = find_by_tag("alpine", "latest").await.unwrap();
        assert!(r.is_some());
    }

    #[tokio::test]
    async fn test_find_nonexistant() {
        let r = find_by_tag("non-existant", "local").await.unwrap();
        assert!(r.is_none());
    }
}
