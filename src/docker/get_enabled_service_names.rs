use crate::docker::{Docker, DockerError};
use std::{collections::HashSet, process::Command};
use core::str;

impl Docker {
    pub fn get_enabled_service_names(project_name: String) -> Result<HashSet<String>, DockerError> {

        let mut services: HashSet<String> = HashSet::new();

        let command = Command::new("docker")
        .args([
            "ps",
            "-a",
            "--filter",
            &format!("label=com.docker.compose.project={}", project_name),
            "--filter",
            "label=com.easydeploy.enabled=true",
            "--format",
            "\"{{ .Label \"com.docker.compose.service\" }}\""
        ])
        .output();

        let output_bytes = match command {
            Ok(v) => v.stdout,
            Err(_) => return Err(DockerError::Failed),
        };

        let output_str: String = str::from_utf8(&output_bytes).unwrap().to_owned();

        let listed_services = output_str.split("\n");

        for service in listed_services {
            services.insert(String::from(service));
        } 
        
        Ok(services)
    }
}