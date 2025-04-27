mod docker;

use docker::Docker;

fn main() {
    let host: String = String::from("some_host");

    let project = Docker::get_compose_project(host).unwrap();

    let services = Docker::get_enabled_service_names(project.clone()).unwrap();

    for service in services.iter() {
        let _: Result<String, docker::DockerError> = Docker::upgrade_service(
            project.clone(),
            service.clone()
        );
    }
}
