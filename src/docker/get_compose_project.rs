use crate::docker::{Docker, DockerContainerId};
use core::str;
use std::process::Command;
use log::info;

use super::DockerError;

impl Docker {
    pub fn get_compose_project(host: DockerContainerId) -> Result<String, DockerError> {

        info!("Getting compose project name for host: {}", &host);

        let command = Command::new("docker")
        .args([
            "inspect",
            "--format",
            "\"{{index .Config.Labels \"com.docker.compose.project\"}}\"",
            &host
        ])
        .output();

        let output_bytes = match command {
            Ok(v) => v,
            Err(_) => return Err(DockerError::Failed),
        };

        let mut output_str = String::from_utf8(output_bytes.stdout).unwrap();

        let mut t = output_str.chars();
        t.next();
        t.next_back();
        t.next_back();

        output_str = String::from(t.as_str());

        Ok(String::from(output_str))
    }
}