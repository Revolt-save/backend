mod dummy;

#[async_trait]
pub trait AbstractMigrations: Sync + Send {
    async fn migrate_database(&self) -> Result<(), ()>;
}
