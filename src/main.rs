use idocker::{InteractivelyCreate, Listable};
use shiplift::{ContainerListOptions, Docker};
use tokio;

#[tokio::main]
async fn main() {
    let opts = ContainerListOptions::interactively_create();
    println!("{:#?}", opts);

    let docker = Docker::new();
    let containers = docker.containers();

    match containers.interactively_select(&opts).await {
        Ok(containers) => println!("{:#?}", containers),
        Err(err) => eprintln!("{:#?}", err),
    }
}
