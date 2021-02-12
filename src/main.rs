use idocker::{InteractivelyCreate, InteractivelySelect};
use shiplift::{ContainerListOptions, Docker};

fn main() {
    let opts = ContainerListOptions::interactively_create();
    println!("{:#?}", opts);

    let docker = Docker::new();
    let containers = docker.containers();

    match containers.interactively_select(&opts) {
        Ok(selected) => println!("{:#?}", selected),
        Err(err) => eprintln!("{:#?}", err),
    }
}
