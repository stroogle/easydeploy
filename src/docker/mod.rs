mod upgrade_service;
mod get_compose_project;
mod get_enabled_service_names;
mod get_service_containers;
mod pull_latest_image;
mod get_container_image;

pub struct Docker {}

#[derive(Debug)]
pub enum DockerError {
    Failed,
    NoUpgradeNeeded
}

pub type DockerContainerId = String;