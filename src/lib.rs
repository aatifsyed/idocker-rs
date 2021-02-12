use async_trait::async_trait;
use colored::Colorize;
use dialoguer::MultiSelect;
use shiplift::{self, rep::Container as ContainerRep, ContainerListOptions, Containers};

// Used for creating ListOptions
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

// Present the given item on the screen
pub trait Formattable {
    fn format(&self) -> String;
}

// The meat of this crate.
// `shiplift` has some common concepts
// - A representation of a docker object (container, volume...).
// - An iterable of that object
// - Options for fetching that iterable
// This trait builds an interactive menu to return a subset of those objects
#[async_trait]
pub trait Listable {
    type Singular: Formattable;
    type ListOptions: Sync;

    async fn plural(
        &self,
        opts: &Self::ListOptions,
    ) -> Result<Vec<Self::Singular>, shiplift::Error>;

    async fn interactively_select(
        &self,
        options: &Self::ListOptions,
    ) -> Result<Vec<Self::Singular>, shiplift::Error> {
        let collection = self.plural(options).await?;
        let menu_items = collection
            .iter()
            .map(|x| x.format())
            .collect::<Vec<String>>();
        let chosen_indices = MultiSelect::new()
            .paged(true)
            .items(&menu_items)
            .interact()?;
        let chosen_singulars = collection
            .into_iter()
            .enumerate() // Use the index
            .filter(|(index, _)| chosen_indices.contains(&index))
            .map(|(_, singular)| singular) // Done with the index
            .collect();
        Ok(chosen_singulars)
    }
}

impl Formattable for ContainerRep {
    fn format(&self) -> String {
        format!(
            "{} {} {}",
            self.names[0],
            format!(", image {}", self.image).dimmed(),
            format!(", status {}", self.status.cyan()).dimmed(),
        )
    }
}

#[async_trait]
impl Listable for Containers<'_> {
    type Singular = ContainerRep;
    type ListOptions = ContainerListOptions;
    async fn plural(
        &self,
        opts: &Self::ListOptions,
    ) -> Result<Vec<Self::Singular>, shiplift::Error> {
        self.list(opts).await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
