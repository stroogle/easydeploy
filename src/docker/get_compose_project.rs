use crate::docker::{Docker, DockerContainerId};
use core::str;
use std::process::Command;

use super::DockerError;

impl Docker {
    pub fn get_compose_project(host: DockerContainerId) -> Result<String, DockerError> {

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

        let output_str = str::from_utf8(&output_bytes.stdout).unwrap();

        Ok(String::from(output_str))
    }
}