use crate::docker::{Docker, DockerError};
use std::process::Command;

impl Docker {
    pub fn upgrade_service(project_name: String, service_name: String) -> Result<String, DockerError> {

        let containers = match Docker::get_service_containers(
            project_name.clone(),
            service_name.clone()
        ) {
            Ok(c) => c,
            Err(e) => return Err(e)
        };

        let container_id: String = match containers.iter().peekable().peek() {
            Some(id) => id.to_string(),
            None => return Err(DockerError::Failed),
        };

        let image_name = match Docker::get_container_image(
            container_id.clone()
        ) {
            Ok(image) => image,
            Err(e) => return Err(e),
        };

        let image_status = match Docker::pull_latest_image(image_name) {
            Ok(status) => status,
            Err(e) => return Err(e),
        };

        match image_status {
            crate::docker::pull_latest_image::ImageStatus::PulledNew => {},
            crate::docker::pull_latest_image::ImageStatus::AlreadyUpToDate => return Err(DockerError::NoUpgradeNeeded),
        }

        for container in containers.iter() {

            let _ = Command::new("docker")
            .args([
                "compose",
                "-p",
                &project_name,
                "-f",
                "/app/docker-compose.yml",
                "up",
                "-d",
                "--no-deps",
                "--scale",
                &format!("{}={}", &service_name, containers.len() + 1),
                "--no-recreate",
                &service_name
            ])
            .output();

            let _ = Command::new("docker")
            .args([
                "stop",
                container
            ])
            .output();

            let _ = Command::new("docker")
            .args([
                "rm",
                container
            ])
            .output();

            let _ = Command::new("docker")
            .args([
                "compose",
                "-p",
                &project_name,
                "-f",
                "/app/docker-compose.yml",
                "up",
                "-d",
                "--no-deps",
                "--scale",
                &format!("{}={}", &service_name, containers.len()),
                "--no-recreate",
                &service_name
            ])
            .output();
        }

        Ok("OK".to_owned())
    }
}