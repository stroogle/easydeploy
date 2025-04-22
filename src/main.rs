use core::str;
use std::process::{Command, Output};
mod docker;

use docker::Docker;

fn main() {
    let host: String = String::from("some_host");

    let project = Docker::get_compose_project(host);

    let services = Docker::get_enabled_service_names(project.clone());

    for service in services.iter() {
        let upgrade: Result<String, docker::DockerError> = Docker::upgrade_service(
            project.clone(),
            service.clone()
        );

        match upgrade {
            Ok(v) => {},
            Err(e) => {}
        }
    }
}
