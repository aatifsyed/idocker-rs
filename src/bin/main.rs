use idocker::Listable;
use shiplift::Docker;
use structopt::StructOpt;
use tokio;

#[derive(Debug, StructOpt)]
enum Container {
    Rm {
        /// Kill the container first
        #[structopt(long, short)]
        force: bool,
    },
    Inspect,
}
#[derive(Debug, StructOpt)]
enum Image {
    Rm,
    Inspect,
}
#[derive(Debug, StructOpt)]
enum Network {
    Rm,
    Inspect,
}
#[derive(Debug, StructOpt)]
enum Volume {
    Rm,
    Inspect,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Interactively select docker objects on which to perform operations")]
enum Opt {
    Container(Container),
    Image(Image),
    Network(Network),
    Volume(Volume),
}

#[tokio::main]
async fn main() {
    let subcommand = Opt::from_args();
    let docker = Docker::new();
    let collection: Box<dyn Listable<_, _>>;

    match subcommand {
        Opt::Container(action) => match action {
            Container::Rm { force } => Some(force),
            Container::Inspect => None,
        },
        Opt::Image(action) => match action {
            Image::Rm => None,
            Image::Inspect => None,
        },
        Opt::Network(action) => match action {
            Network::Rm => None,
            Network::Inspect => None,
        },
        Opt::Volume(action) => match action {
            Volume::Rm => None,
            Volume::Inspect => None,
        },
    };

    let containers = docker.containers();

    match containers.interactively_select(&Default::default()).await {
        Ok(containers) => println!("{:#?}", containers),
        Err(err) => eprintln!("{:#?}", err),
    }
}
