// Copyright 2022-2024 Nomadic Workplace
// SPDX-License-Identifier: MIT OR Apache-2.0

extern crate kiss_docker;

#[tokio::main]
async fn main() {
    match kiss_docker::container::list_all(Some("alpine")).await {
        Ok(containers) => containers
            .iter()
            .for_each(|container| println!("{}", container.id)),
        Err(e) => {
            panic!("{}", e);
        }
    };
}
