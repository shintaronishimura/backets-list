use std::fs;
use std::path::PathBuf;
use git2::{Repository, Signature, IndexAddOption};
use crate::models::BucketItem;

pub struct DreamRepository {
    pub base_path: PathBuf,
}

impl DreamRepository {
    pub fn new(app_data_dir: PathBuf) -> Self {
        let repo_path = app_data_dir.join("dreams_repo");
        if !repo_path.exists() {
            fs::create_dir_all(&repo_path).unwrap();
        }

        // git init if not exists
        if !repo_path.join(".git").exists() {
            Repository::init(&repo_path).expect("Failed to init git repository");
        }

        Self {
            base_path: repo_path,
        }
    }

    fn get_data_file(&self) -> PathBuf {
        self.base_path.join("dreams.json")
    }

    pub fn load_items(&self) -> Vec<BucketItem> {
        let file_path = self.get_data_file();
        if !file_path.exists() {
            return Vec::new();
        }

        let content = fs::read_to_string(file_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    }

    pub fn save_items(&self, items: &[BucketItem], message: &str) -> Result<(), String> {
        let file_path = self.get_data_file();
        let content = serde_json::to_string_pretty(items).map_err(|e| e.to_string())?;
        fs::write(&file_path, content).map_err(|e| e.to_string())?;

        self.commit_changes(message)
    }

    fn commit_changes(&self, message: &str) -> Result<(), String> {
        let repo = Repository::open(&self.base_path).map_err(|e| e.to_string())?;
        let mut index = repo.index().map_err(|e| e.to_string())?;
        
        index.add_all(["dreams.json"].iter(), IndexAddOption::DEFAULT, None).map_err(|e| e.to_string())?;
        index.write().map_err(|e| e.to_string())?;

        let tree_id = index.write_tree().map_err(|e| e.to_string())?;
        let tree = repo.find_tree(tree_id).map_err(|e| e.to_string())?;

        let signature = Signature::now("DreamAnchor", "app@example.com").map_err(|e| e.to_string())?;
        
        // Find the parent commit if it exists
        let head = repo.head();
        let parent_commits = match head {
            Ok(h) => vec![h.peel_to_commit().map_err(|e| e.to_string())?],
            Err(_) => vec![],
        };

        let parents: Vec<&git2::Commit> = parent_commits.iter().collect();

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        ).map_err(|e| e.to_string())?;

        Ok(())
    }
}
