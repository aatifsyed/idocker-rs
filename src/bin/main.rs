use idocker::Listable;
use shiplift::{
    self, Container, ContainerListOptions, Docker, ImageListOptions, NetworkListOptions,
};
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

fn main() {
    let subcommand = Opt::from_args();

    match subcommand {
        Opt::Container(action) => subcommand_container(action),
        Opt::Image(action) => subcommand_image(action),
        Opt::Network(action) => subcommand_network(action),
        Opt::Volume(action) => subcommand_volume(action),
    };
}

#[tokio::main]
async fn subcommand_container(action: ContainerOpt) -> Result<(), Box<dyn Error>> {
    let docker = Docker::new();
    let opts = ContainerListOptions::builder().all().build();
    let containers = docker.containers();
    let selected = containers.interactively_select(&opts).await?;
    let selected = selected
        .iter()
        .map(|rep| Container::new(&docker, rep.id.to_owned()));

    match action {
        ContainerOpt::Rm { force } => {
            if force {
                selected.map(|container| container.kill(None));
            };
            selected.map(|container| container.remove(Default::default()))
        }
        ContainerOpt::Inspect => (),
    };
    Ok(())
}

#[tokio::main]
async fn subcommand_image(action: ImageOpt) -> Result<(), Box<dyn Error>> {
    match action {
        ImageOpt::Rm => 1,
        ImageOpt::Inspect => 1,
    };
    Ok(())
}

#[tokio::main]
async fn subcommand_network(action: NetworkOpt) -> Result<(), Box<dyn Error>> {
    match action {
        NetworkOpt::Rm => 1,
        NetworkOpt::Inspect => 1,
    };
    Ok(())
}

#[tokio::main]
async fn subcommand_volume(action: VolumeOpt) -> Result<(), Box<dyn Error>> {
    match action {
        VolumeOpt::Rm => 1,
        VolumeOpt::Inspect => 1,
    };
    Ok(())
}
