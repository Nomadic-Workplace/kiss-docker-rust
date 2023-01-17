use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageSummary {
    // #[serde(rename = "Containers")]
    // /// Number of containers using this image. Includes both stopped and running
    // /// containers.
    // ///
    // /// This size is not calculated by default, and depends on which API endpoint
    // /// is used. `-1` indicates that the value has not been set / calculated.
    // pub containers: isize,
    // #[serde(rename = "Created")]
    // /// Date and time at which the image was created as a Unix timestamp
    // /// (number of seconds sinds EPOCH).
    // pub created: isize,
    #[serde(rename = "ID")]
    /// ID is the content-addressable ID of an image.
    ///
    /// This identifier is a content-addressable digest calculated from the
    /// image's configuration (which includes the digests of layers used by
    /// the image).
    ///
    /// Note that this digest differs from the `RepoDigests` below, which
    /// holds digests of image manifests that reference the image.
    pub id: String,
    #[serde(rename = "Tag")]
    #[serde(default)]
    /// Image name/tas in the local image cache that reference this
    /// image.
    ///
    /// Multiple image tags can refer to the same image and this item may be
    /// empty if no tags reference the image, in which case the image is
    /// "untagged", in which case it can still be referenced by its ID.
    pub tag: String,

    #[serde(rename = "Repository")]
    #[serde(default)]
    pub repository: String,
}
