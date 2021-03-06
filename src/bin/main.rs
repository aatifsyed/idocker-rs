use idocker::Listable;
use shiplift::{
    self, Container, ContainerListOptions, Docker, Image, ImageListOptions, Network,
    NetworkListOptions, Volume,
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
#[structopt(about)]
enum Opt {
    Container(ContainerOpt),
    Image(ImageOpt),
    Network(NetworkOpt),
    Volume(VolumeOpt),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subcommand = Opt::from_args();
    let docker = Docker::new();

    match subcommand {
        Opt::Container(action) => subcommand_container(docker, action).await,
        Opt::Image(action) => subcommand_image(docker, action).await,
        Opt::Network(action) => subcommand_network(docker, action).await,
        Opt::Volume(action) => subcommand_volume(docker, action).await,
    }
}

async fn subcommand_container(docker: Docker, action: ContainerOpt) -> Result<(), Box<dyn Error>> {
    let opts = ContainerListOptions::builder().all().build();
    let containers = docker.containers();
    let selected = containers.interactively_select(&opts).await?;
    let selected: Vec<_> = selected
        .iter()
        .map(|rep| Container::new(&docker, rep.id.to_owned()))
        .collect();

    match action {
        ContainerOpt::Rm { force } => {
            if force {
                for container in &selected {
                    match container.kill(None).await {
                        Err(err) => {
                            eprintln!(
                                "Couldn't kill container {}: {:?}",
                                container.inspect().await?.name,
                                err
                            );
                        }
                        _ => (),
                    };
                }
            };
            for container in &selected {
                match container.delete().await {
                    Err(err) => eprintln!("Couldn't delete container {}: {}", container.id(), err),
                    _ => (),
                }
            }
            Ok(())
        }
        ContainerOpt::Inspect => unimplemented!(),
    }
}

async fn subcommand_image(docker: Docker, action: ImageOpt) -> Result<(), Box<dyn Error>> {
    let opts = ImageListOptions::builder().all().build();
    let images = docker.images();
    let selected = images.interactively_select(&opts).await?;
    let selected: Vec<_> = selected
        .iter()
        .map(|rep| Image::new(&docker, rep.id.to_owned()))
        .collect();

    match action {
        ImageOpt::Rm => {
            for image in &selected {
                match image.delete().await {
                    Err(err) => {
                        eprintln!(
                            "Couldn't delete image {}: {}",
                            match image.inspect().await {
                                Ok(details) => details.id,
                                Err(_) => "<Couldn't get ID>".to_owned(),
                            },
                            err
                        )
                    }
                    _ => (),
                }
            }
        }
        ImageOpt::Inspect => unimplemented!(),
    };
    Ok(())
}

async fn subcommand_network(docker: Docker, action: NetworkOpt) -> Result<(), Box<dyn Error>> {
    let opts = NetworkListOptions::default();
    let networks = docker.networks();
    let selected = networks.interactively_select(&opts).await?;
    let selected: Vec<_> = selected
        .iter()
        .map(|rep| Network::new(&docker, rep.id.to_owned()))
        .collect();
    match action {
        NetworkOpt::Rm => {
            for network in &selected {
                match network.delete().await {
                    Err(err) => eprintln!("Couldn't delete network {}: {}", network.id(), err),
                    _ => (),
                }
            }
        }
        NetworkOpt::Inspect => unimplemented!(),
    };
    Ok(())
}

async fn subcommand_volume(docker: Docker, action: VolumeOpt) -> Result<(), Box<dyn Error>> {
    let opts = ();
    let volumes = docker.volumes();
    let selected = volumes.interactively_select(&opts).await?;
    let selected: Vec<_> = selected
        .iter()
        .map(|rep| Volume::new(&docker, rep.name.to_owned()))
        .collect();
    match action {
        VolumeOpt::Rm => {
            for volume in &selected {
                match volume.delete().await {
                    Err(err) => eprintln!("Couldn't delete a volume: {}", err),
                    _ => (),
                }
            }
        }
        VolumeOpt::Inspect => unimplemented!(),
    };
    Ok(())
}
