use crate::docker::{Docker, DockerError, DockerContainerId};

use std::process:: Command;
use core::str;

impl Docker {

    pub fn get_container_image(container_id: DockerContainerId) -> Result<String, DockerError> {

        let command = Command::new("docker")
        .args([
            "inspect",
            &container_id,
            "--format",
            "\"{{ .Config.Image }}\""
        ])
        .output();

        let output_bytes = match command {
            Ok(v) => v.stdout,
            Err(_) => return Err(DockerError::Failed),
        };

        let output_str: String = str::from_utf8(&output_bytes).unwrap().to_owned();

        Ok(output_str)
    }

}