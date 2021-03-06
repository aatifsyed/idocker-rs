use crate::traits::{Formattable, Listable};

use async_trait::async_trait;
use colored::Colorize;
use shiplift::{
    self,
    rep::{Container as ContainerRep, Image as ImageRep, NetworkDetails, Volume as VolumeRep},
    ContainerListOptions, Containers, ImageListOptions, Images, NetworkListOptions, Networks,
    Volumes,
};
use std::error::Error;

impl Formattable for ContainerRep {
    fn format(&self) -> String {
        format!(
            "{}{}{}",
            self.names[0],
            format!(", {}", self.image).dimmed(),
            format!(", {}", self.status.cyan()).dimmed(),
        )
    }
}
#[async_trait]
impl Listable for Containers<'_> {
    type Singular = ContainerRep;
    type ListOptions = ContainerListOptions;
    async fn list(&self, opts: &Self::ListOptions) -> Result<Vec<Self::Singular>, Box<dyn Error>> {
        Ok(Containers::list(self, opts).await?)
    }
}

impl Formattable for ImageRep {
    fn format(&self) -> String {
        let name = match &self.labels {
            Some(map) => format!("{:?}", map),
            None => self.id.to_owned(),
        };
        format!(
            "{} {} {}",
            name,
            format!(", created {}", self.created).dimmed(),
            format!(", size {}", self.virtual_size).cyan().dimmed()
        )
    }
}
#[async_trait]
impl Listable for Images<'_> {
    type Singular = ImageRep;
    type ListOptions = ImageListOptions;
    async fn list(&self, opts: &Self::ListOptions) -> Result<Vec<Self::Singular>, Box<dyn Error>> {
        Ok(Images::list(self, opts).await?)
    }
}

impl Formattable for NetworkDetails {
    fn format(&self) -> String {
        self.name.to_owned()
    }
}
#[async_trait]
impl Listable for Networks<'_> {
    type Singular = NetworkDetails;
    type ListOptions = NetworkListOptions;
    async fn list(&self, opts: &Self::ListOptions) -> Result<Vec<Self::Singular>, Box<dyn Error>> {
        Ok(Networks::list(self, opts).await?)
    }
}

impl Formattable for VolumeRep {
    fn format(&self) -> String {
        format!(
            "{}{}",
            self.name,
            format!(", created {}", self.created_at).dimmed()
        )
    }
}
#[async_trait]
impl Listable for Volumes<'_> {
    type Singular = VolumeRep;
    type ListOptions = ();
    async fn list(&self, _: &Self::ListOptions) -> Result<Vec<Self::Singular>, Box<dyn Error>> {
        Ok(Volumes::list(self).await?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
