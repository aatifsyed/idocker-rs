use idocker::Listable;
use shiplift::Docker;
use tokio;

#[tokio::main]
async fn main() {
    let docker = Docker::new();
    let containers = docker.containers();

    match containers.interactively_select(&Default::default()).await {
        Ok(containers) => println!("{:#?}", containers),
        Err(err) => eprintln!("{:#?}", err),
    }
}
