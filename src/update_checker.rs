use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub download_url: String,
    pub release_notes: String,
    pub published_at: String,
}

#[derive(Debug, Clone)]
pub struct UpdateChecker {
    pub current_version: String,
    pub repo_owner: String,
    pub repo_name: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    html_url: String,
    body: Option<String>,
    published_at: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

impl UpdateChecker {
    pub fn new(repo_owner: String, repo_name: String) -> Self {
        let current_version = env!("CARGO_PKG_VERSION").to_string();
        Self {
            current_version,
            repo_owner,
            repo_name,
        }
    }

    pub async fn check_for_updates(&self) -> Result<Option<UpdateInfo>, String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            self.repo_owner, self.repo_name
        );

        let client = reqwest::Client::builder()
            .user_agent("inventory-app")
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch releases: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("GitHub API returned status: {}", response.status()));
        }

        let release: GitHubRelease = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse release data: {}", e))?;

        // Remove 'v' prefix if present for comparison
        let latest_version = release.tag_name.trim_start_matches('v');
        let current_version = self.current_version.trim_start_matches('v');

        // Compare versions using semver
        let current = semver::Version::parse(current_version)
            .map_err(|e| format!("Invalid current version: {}", e))?;
        let latest = semver::Version::parse(latest_version)
            .map_err(|e| format!("Invalid latest version: {}", e))?;

        if latest > current {
            // Find the appropriate asset for the current platform
            let platform = get_platform_identifier();
            let asset = release
                .assets
                .iter()
                .find(|a| a.name.contains(&platform))
                .ok_or_else(|| {
                    format!("No release asset found for platform: {}", platform)
                })?;

            Ok(Some(UpdateInfo {
                version: release.tag_name,
                download_url: asset.browser_download_url.clone(),
                release_notes: release.body.unwrap_or_else(|| "No release notes available.".to_string()),
                published_at: release.published_at,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn download_installer(&self, download_url: &str) -> Result<PathBuf, String> {
        let client = reqwest::Client::builder()
            .user_agent("inventory-app")
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        let response = client
            .get(download_url)
            .send()
            .await
            .map_err(|e| format!("Failed to download installer: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Download failed with status: {}", response.status()));
        }

        // Extract filename from URL
        let filename = download_url
            .split('/')
            .last()
            .ok_or_else(|| "Invalid download URL".to_string())?;

        // Get Downloads directory
        let downloads_dir = directories::UserDirs::new()
            .and_then(|dirs| dirs.download_dir().map(|p| p.to_path_buf()))
            .ok_or_else(|| "Could not find Downloads directory".to_string())?;

        let file_path = downloads_dir.join(filename);

        // Download the file
        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read download data: {}", e))?;

        std::fs::write(&file_path, &bytes)
            .map_err(|e| format!("Failed to save installer: {}", e))?;

        Ok(file_path)
    }

    pub fn open_installer(path: &PathBuf) -> Result<(), String> {
        open::that(path).map_err(|e| format!("Failed to open installer: {}", e))
    }
}

fn get_platform_identifier() -> String {
    match std::env::consts::OS {
        "macos" => "macos",
        "windows" => "windows",
        "linux" => "linux",
        other => other,
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        let current = semver::Version::parse("0.1.0").unwrap();
        let latest = semver::Version::parse("0.2.0").unwrap();
        assert!(latest > current);
    }

    #[test]
    fn test_platform_identifier() {
        let platform = get_platform_identifier();
        assert!(!platform.is_empty());
    }
}

