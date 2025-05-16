use crate::docker::{Docker, DockerError};
use std::{collections::HashSet, process::Command};
use core::str;
use log::debug;

impl Docker {
    pub fn get_enabled_service_names(project_name: String) -> Result<HashSet<String>, DockerError> {

        let mut services: HashSet<String> = HashSet::new();

        debug!("Looking for easydeploy enabled services...");

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
            Ok(v) => {
                debug!("Command output bytes: {:?}", &v.stdout);
                v.stdout
            },
            Err(_) => return Err(DockerError::Failed),
        };

        let output_str: String = String::from_utf8(output_bytes).unwrap();

        debug!("Looking for enabled services, found: {}", output_str);

        for service in output_str.lines() {
            let mut t = service.chars();
            t.next();
            t.next_back();
            services.insert(String::from(t.as_str()));
        } 
        
        Ok(services)
    }
}