use async_trait::async_trait;
use godot::classes::Os;
use std::path::PathBuf;
use wharfkit_session::{SessionStorage, StorageError};

#[allow(dead_code)]
pub struct UserSessionStorage {
    base_dir: PathBuf,
}

#[allow(dead_code)]
impl UserSessionStorage {
    pub fn new() -> Self {
        let user_dir = Os::singleton().get_user_data_dir().to_string();
        let mut base_dir = PathBuf::from(user_dir);
        base_dir.push("wharfkit");
        Self { base_dir }
    }

    fn path_for(&self, key: &str) -> PathBuf {
        let safe: String = key
            .chars()
            .map(|c| if c == ':' || c == '/' { '_' } else { c })
            .collect();
        self.base_dir.join(format!("{safe}.bin"))
    }
}

impl Default for UserSessionStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SessionStorage for UserSessionStorage {
    async fn read(&self, key: &str) -> Option<Vec<u8>> {
        let path = self.path_for(key);
        tokio::task::spawn_blocking(move || std::fs::read(&path).ok())
            .await
            .ok()
            .flatten()
    }

    async fn write(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        let path = self.path_for(key);
        let value = value.to_vec();
        let base_dir = self.base_dir.clone();
        tokio::task::spawn_blocking(move || {
            std::fs::create_dir_all(&base_dir).map_err(|e| StorageError::Io(e.to_string()))?;
            std::fs::write(&path, value).map_err(|e| StorageError::Io(e.to_string()))
        })
        .await
        .map_err(|e| StorageError::Io(format!("join error: {e}")))?
    }

    async fn remove(&self, key: &str) -> Result<(), StorageError> {
        let path = self.path_for(key);
        tokio::task::spawn_blocking(move || match std::fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(StorageError::Io(e.to_string())),
        })
        .await
        .map_err(|e| StorageError::Io(format!("join error: {e}")))?
    }
}
