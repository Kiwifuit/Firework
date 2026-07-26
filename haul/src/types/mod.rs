pub mod forge;
pub mod modrinth;

pub use forge::ForgeModpack;
pub use modrinth::ModrinthModpack;

#[derive(PartialEq)]
pub enum ModpackProvider {
    Forge,
    Modrinth,
    None,
}

pub trait ModpackProviderMetadata {
    type Out;

    fn overrides_dir(&self) -> Self::Out;
    fn modpack_name(&self) -> Self::Out;
}
