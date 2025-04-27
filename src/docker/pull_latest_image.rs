use crate::docker::{
    Docker,
    DockerError
};
use std::process::Command;
use core::str;
use log::info;

pub enum ImageStatus {
    PulledNew,
    AlreadyUpToDate
}

impl Docker {

    pub fn pull_latest_image(image_name: String) -> Result<ImageStatus, DockerError> {
        info!("Attempting to pull image: {}", &image_name);

        let command = Command::new("docker")
        .args([
            "image",
            "pull",
            &image_name
        ])
        .output();

        let output_bytes = match command {
            Ok(v) => v.stdout,
            Err(_) => return Err(DockerError::Failed),
        };

        let output_str: String = str::from_utf8(&output_bytes).unwrap().to_owned();

        match output_str.contains("Image is up to date") {
            true => Ok(ImageStatus::AlreadyUpToDate),
            false => Ok(ImageStatus::PulledNew),
        }

    }

}