mod upgrade_service;
mod get_compose_project;
mod get_enabled_service_names;

pub struct Docker {}

pub enum DockerError {
    Failed
}

pub type DockerContainerId = String;