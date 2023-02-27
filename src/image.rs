use crate::command::docker_exec;
use crate::error;
use crate::models::ImageSummary;

pub async fn list_images() -> error::Result<Vec<ImageSummary>> {
    let output = docker_exec(vec!["images", "--digests", "--format", "{{json .}}"])
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

pub async fn pull(name: &str, tag: Option<&str>) -> error::Result<()> {
    let image = format!("{}:{}", name, tag.unwrap_or("latest"));
    docker_exec(vec!["pull", &image]).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::image::{find_by_tag, list_images, pull};

    #[tokio::test]
    async fn test_list_images() {
        list_images().await.unwrap();
    }

    #[tokio::test]
    async fn pull_alpine() {
        pull("alpine", Some("latest")).await.unwrap();
        pull("alpine", None).await.unwrap();
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

    #[tokio::test]
    async fn test_digest() {
        let r = find_by_tag("alpine", "latest").await.unwrap();
        assert!(r.is_some());
        assert!(r.unwrap().digest.starts_with("sha256:"));
    }
}
