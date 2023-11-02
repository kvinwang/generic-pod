use anyhow::Result;

use bollard::container::{Config, CreateContainerOptions, UpdateContainerOptions};
use bollard::service::{RestartPolicy, RestartPolicyNameEnum};
use bollard::Docker;

pub(crate) async fn start_container(name: &str, image: &str, cmd: Option<&str>) -> Result<String> {
    let docker = Docker::connect_with_local_defaults()?;
    let result = docker.inspect_container(name, None).await;
    let container_id = if let Ok(container) = result {
        container.id
    } else {
        None
    };
    let container_id = match container_id {
        Some(id) => id,
        None => {
            let container = docker
                .create_container(
                    Some(CreateContainerOptions {
                        name,
                        ..Default::default()
                    }),
                    Config {
                        image: Some(image),
                        cmd: cmd.map(|cmd| vec![cmd]),
                        ..Default::default()
                    },
                )
                .await?;
            docker
                .update_container::<&str>(
                    &container.id,
                    UpdateContainerOptions {
                        restart_policy: Some(RestartPolicy {
                            name: Some(RestartPolicyNameEnum::ALWAYS),
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                )
                .await?;
            container.id
        }
    };
    docker.start_container::<&str>(&container_id, None).await?;
    Ok(container_id)
}

pub(crate) async fn stop_container(name: &str) -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;
    docker.stop_container(name, None).await?;
    Ok(())
}
