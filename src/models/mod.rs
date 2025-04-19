use bollard::{Docker, container::ListContainersOptions, models::ContainerSummary};
use ollama_rs::Ollama;
use std::{collections::HashMap, default::Default};

struct Model {
    ollama: Ollama,
    container_names: Vec<String>,
}

pub struct Models {
    variants: HashMap<String, HashMap<String, Model>>,
    docker: Docker,
}

pub async fn make_models() -> Models {
    let docker: Docker = Docker::connect_with_socket_defaults().unwrap();

    Models::new(docker, []).await
}

impl Models {
    pub async fn new<const N: usize>(client: Docker, filters: [(&str, Vec<&str>); N]) -> Self {
        let options: Option<ListContainersOptions<&str>> = match filters.len() < 1 {
            true => None,
            false => Some(ListContainersOptions {
                all: true,
                filters: HashMap::from(filters),
                ..Default::default()
            }),
        };

        let containers: Vec<ContainerSummary> = client.list_containers(options).await.unwrap();

        for container in containers {
            println!("{:#?}", container);
        }

        Self {
            variants: HashMap::new(),
            docker: client,
        }
    }
}
