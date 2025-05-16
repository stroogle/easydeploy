use crate::docker::{Docker, DockerError, DockerContainerId};

use std::process:: Command;
use log::{debug, info};

impl Docker {

    pub fn get_container_image(container_id: DockerContainerId) -> Result<String, DockerError> {
        info!("Getting image for container: {}", &container_id);
        let command = Command::new("docker")
        .args([
            "inspect",
            &container_id,
            "--format",
            "\"{{ .Config.Image }}\""
        ])
        .output();

        let output_bytes = match command {
            Ok(v) => {
                debug!("Getting container image, raw bytes from output are: {:?}", &v.stdout);
                v.stdout
            },
            Err(_) => return Err(DockerError::Failed),
        };

        let mut output_str = String::from_utf8(output_bytes).unwrap();

        let mut t = output_str.chars();
        t.next();
        t.next_back();
        t.next_back();

        output_str = String::from(t.as_str());

        Ok(output_str)
    }

}