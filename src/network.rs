// Copyright 2022-2024 Nomadic Workplace
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::command::docker_exec;
use crate::models::NetworkSummary;
use crate::{command, error};
use std::collections::HashMap;

pub async fn list_networks() -> error::Result<Vec<NetworkSummary>> {
    let output = docker_exec(vec!["network", "ls", "--format", "{{json .}}"])
        .await
        .unwrap();
    let images: Vec<NetworkSummary> = output
        .split('\n')
        .filter(|part| !part.is_empty())
        .map(serde_json::from_str)
        .collect::<Result<Vec<NetworkSummary>, serde_json::Error>>()?;

    Ok(images)
}

#[derive(Debug, Clone, Default)]
pub struct Network<'a> {
    pub name: &'a str,

    /// docker runtime options/flags, the caller is responsible for providing the - or -- prefix
    pub flags: HashMap<String, String>,
}

pub async fn rm_network(id: &str) -> error::Result<()> {
    docker_exec(vec!["network", "rm", id]).await?;
    Ok(())
}

pub async fn network_connect_container(network_id: &str, container_id: &str) -> error::Result<()> {
    docker_exec(vec!["network", "connect", network_id, container_id]).await?;
    Ok(())
}

pub async fn network_disconnect_container(
    network_id: &str,
    container_id: &str,
) -> error::Result<()> {
    docker_exec(vec!["network", "disconnect", network_id, container_id]).await?;
    Ok(())
}

impl Network<'_> {
    pub async fn create(&self) -> error::Result<String> {
        let mut cmd = vec!["network", "create"];

        let r_o = self.get_runtime_flags();
        let flags: Vec<&str> = r_o.iter().map(|s| s.as_str()).collect();

        cmd.extend(flags);

        cmd.push(self.name);

        let result = command::docker_exec(cmd).await?;

        Ok(String::from(result.trim()))
    }

    pub fn get_runtime_flags(&self) -> Vec<String> {
        let mut env: Vec<String> = vec![];
        for (key, value) in &self.flags {
            env.push(key.clone());
            env.push(value.clone());
        }
        env
    }
}

#[cfg(test)]
mod tests {
    use crate::network::{list_networks, rm_network, Network};

    #[tokio::test]
    async fn test_list_images() {
        list_networks().await.unwrap();
    }

    #[test]
    fn test_make_default() {
        let _ctn = Network {
            ..Default::default()
        };
    }

    #[tokio::test]
    async fn test_create_and_destroy() {
        let network_id = Network {
            name: "test",
            ..Default::default()
        }
        .create()
        .await
        .unwrap();

        rm_network(&network_id).await.unwrap();
    }
}
