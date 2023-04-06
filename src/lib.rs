use std::path::Path;

#[mockall::automock]
#[async_trait::async_trait]
pub trait NotWorking: Send + Sync {
    async fn some_func(&self, path: &Path);
}

pub struct NotWorkingImpl {}

#[async_trait::async_trait]
impl NotWorking for NotWorkingImpl {
    async fn some_func(&self, path: &Path) {
        println!("{}", path.display());
    }
}
