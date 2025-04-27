use std::{
    collections::HashSet,
    process::Command
};
use core::str;
use crate::docker::{Docker, DockerError};

use super::DockerContainerId;

impl Docker {

    pub fn get_service_containers(project_name: String, service_name: String) -> Result<HashSet<DockerContainerId>, DockerError> {
        
        let mut containers = HashSet::new();    
    
        let project_filter = format!(
            "\"label=com.docker.compose.project={}\"",
            project_name
        );

        let service_filter = format!(
            "\"label=com.docker.compose.service={}\"",
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

        let listed_containers = output_str.split("\n");

        for container in listed_containers {
            containers.insert(String::from(container));
        } 

        Ok(containers)
    }

}