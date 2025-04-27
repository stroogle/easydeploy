use std::{
    collections::HashSet,
    process::Command
};
use core::str;
use crate::docker::{Docker, DockerError};
use log::{info, debug};

use super::DockerContainerId;

impl Docker {

    pub fn get_service_containers(project_name: String, service_name: String) -> Result<HashSet<DockerContainerId>, DockerError> {
        info!("Getting service containers for project: {}, service: {}", &project_name, &service_name);

        let mut containers = HashSet::new();    
    
        let project_filter = format!(
            "label=com.docker.compose.project={}",
            project_name
        );

        debug!("Set project filter to {}", &project_filter);

        let service_filter = format!(
            "label=com.docker.compose.service={}",
            service_name
        );

        let command = Command::new("docker")
        .args([
            "ps",
            "-aq",
            "--filter",
            &project_filter,
            "--filter",
            &service_filter
        ])
        .output();

        let output_bytes = match command {
            Ok(v) => v.stdout,
            Err(_) => return Err(DockerError::Failed),
        };

        let output_str: String = str::from_utf8(&output_bytes).unwrap().to_owned();

        for container in output_str.lines() {
            let t = container.chars();
            containers.insert(String::from(t.as_str()));
        }

        info!("Found service containers: {:?}", &containers);

        Ok(containers)
    }

}