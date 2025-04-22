pub struct Docker {}

pub enum DockerError {
    Failed(&str)
}

pub type DockerContainerId = String;