use crate::traits::{Formatable, Listable};

use async_trait::async_trait;
use colored::Colorize;
use shiplift::{self, rep::Container as ContainerRep, ContainerListOptions, Containers};

impl Formatable for ContainerRep {
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
