mod docker;

use docker::Docker;
use std::env;
use log::info;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    info!("Starting easyploy.");

    let host: String = String::from(env::var("HOSTNAME").unwrap());

    let project = Docker::get_compose_project(host).unwrap();

    info!("Found project: {}", &project);

    let services = Docker::get_enabled_service_names(project.clone()).unwrap();

    info!("Found services: {:?}", &services);

    for service in services.iter() {
        let _: Result<String, docker::DockerError> = Docker::upgrade_service(
            project.clone(),
            service.clone()
        );
    }
}
