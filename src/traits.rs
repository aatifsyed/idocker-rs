use async_trait::async_trait;
use dialoguer::MultiSelect;

// Used for creating ListOptions
pub trait InteractivelyCreate {
    fn interactively_create() -> Self;
}

// Present the given item on the screen, for interactive menus
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
