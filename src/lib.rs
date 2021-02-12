use colored::Colorize;
use dialoguer::MultiSelect;
use shiplift::{self, rep::Container as ContainerRep, ContainerListOptions, Containers};

pub trait InteractivelyCreate {
    fn interactively_create() -> Self;
}

impl InteractivelyCreate for ContainerListOptions {
    fn interactively_create() -> Self {
        let options = vec!["all"];

        let chosen = MultiSelect::new()
            .items(&options)
            .interact()
            .expect("Couldn't select");

        let mut builder = ContainerListOptions::builder();

        if chosen.contains(&0) {
            builder.all();
        }

        builder.build()
    }
}

pub trait Listable {
    type Singular;
    type ListOptions;
}

pub trait InteractivelySelect: Listable {
    fn interactively_select(
        &self,
        options: &Self::ListOptions,
    ) -> Result<Vec<Self::Singular>, shiplift::Error>;
}

impl Listable for Containers<'_> {
    type Singular = ContainerRep;
    type ListOptions = ContainerListOptions;
}

impl InteractivelySelect for Containers<'_> {
    #[tokio::main]
    async fn interactively_select(
        &self,
        options: &ContainerListOptions,
    ) -> Result<Vec<ContainerRep>, shiplift::Error> {
        let containers = self.list(options).await?;
        let menu_items = containers
            .iter()
            .map(|x| {
                format!(
                    "{} {} {}",
                    x.names[0],
                    format!(", image {}", x.image).dimmed(),
                    format!(", status {}", x.status.cyan()).dimmed(),
                )
            })
            .collect::<Vec<String>>();
        let chosen_indices = MultiSelect::new()
            .paged(true)
            .items(&menu_items)
            .interact()?;
        let chosen_containers = containers
            .into_iter()
            .enumerate() // Use the index
            .filter(|(index, _)| chosen_indices.contains(&index))
            .map(|(_, container)| container)
            .collect();
        Ok(chosen_containers)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
