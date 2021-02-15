use idocker::Listable;
use shiplift::{self, Container, ContainerListOptions, Docker};
use std::error::Error;
use structopt::StructOpt;
use tokio;

#[derive(Debug, StructOpt)]
enum ContainerOpt {
    Rm {
        /// Kill the container first
        #[structopt(long, short)]
        force: bool,
    },
    Inspect,
}
#[derive(Debug, StructOpt)]
enum ImageOpt {
    Rm,
    Inspect,
}
#[derive(Debug, StructOpt)]
enum NetworkOpt {
    Rm,
    Inspect,
}
#[derive(Debug, StructOpt)]
enum VolumeOpt {
    Rm,
    Inspect,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Interactively select docker objects on which to perform operations")]
enum Opt {
    Container(ContainerOpt),
    Image(ImageOpt),
    Network(NetworkOpt),
    Volume(VolumeOpt),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subcommand = Opt::from_args();

    match subcommand {
        Opt::Container(action) => subcommand_container(action).await,
        Opt::Image(action) => subcommand_image(action).await,
        Opt::Network(action) => subcommand_network(action).await,
        Opt::Volume(action) => subcommand_volume(action).await,
    }
}

async fn subcommand_container(action: ContainerOpt) -> Result<(), Box<dyn Error>> {
    let docker = Docker::new();
    let opts = ContainerListOptions::builder().all().build();
    let containers = docker.containers();
    let selected = containers.interactively_select(&opts).await?;
    let selected: Vec<_> = selected
        .iter()
        .map(|rep| Container::new(&docker, rep.id.to_owned()))
        .collect();

    match action {
        ContainerOpt::Rm { force } => remove_containers(force, selected),
        ContainerOpt::Inspect => unimplemented!(),
    }
    .await?;
    Ok(())
}

async fn remove_containers(
    force: bool,
    containers: Vec<Container<'_>>,
) -> Result<(), Box<dyn Error>> {
    if force {
        for container in &containers {
            match container.kill(None).await {
                Err(err) => eprintln!("Couldn't kill container {}: {}", container.id(), err),
                _ => (),
            };
        }
    };
    for container in &containers {
        match container.delete().await {
            Err(err) => eprintln!("Couldn't delete container {}: {}", container.id(), err),
            _ => (),
        }
    }
    Ok(())
}

async fn subcommand_image(action: ImageOpt) -> Result<(), Box<dyn Error>> {
    match action {
        ImageOpt::Rm => unimplemented!(),
        ImageOpt::Inspect => unimplemented!(),
    };
    Ok(())
}

async fn subcommand_network(action: NetworkOpt) -> Result<(), Box<dyn Error>> {
    match action {
        NetworkOpt::Rm => unimplemented!(),
        NetworkOpt::Inspect => unimplemented!(),
    };
    Ok(())
}

async fn subcommand_volume(action: VolumeOpt) -> Result<(), Box<dyn Error>> {
    match action {
        VolumeOpt::Rm => unimplemented!(),
        VolumeOpt::Inspect => unimplemented!(),
    };
    Ok(())
}
