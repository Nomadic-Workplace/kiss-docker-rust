// Copyright 2022-2024 Nomadic Workplace
// SPDX-License-Identifier: MIT OR Apache-2.0

use thiserror::Error;

#[derive(Debug, Error)]
pub enum KissDockerError {
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Processing JSON has failed: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Docker command got terminated by a signal and did not complete")]
    CommandTerminatedUnexpectedly,
    #[error("Docker command failed with: {failure}")]
    DockerCommandFailed { failure: String },
}

pub type Result<T> = std::result::Result<T, KissDockerError>;
