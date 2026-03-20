use std::io::Write;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::app::App;
use crate::tmux;

#[derive(Serialize, Deserialize)]
struct ParkFile {
    parked_at: String,
    sessions: Vec<ParkedSession>,
}

#[derive(Serialize, Deserialize)]
struct ParkedSession {
    session_id: String,
    tmux_session: String,
    cwd: String,
}

fn park_file_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".local").join("state").join("recon").join("parked.json"))
}

fn is_symlink(path: &Path) -> bool {
    std::fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

fn ensure_secure_parent_dir(parent: &Path) -> Result<(), String> {
    std::fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {e}"))?;

    if is_symlink(parent) {
        return Err("Refusing to use symlinked park directory".to_string());
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(parent, std::fs::Permissions::from_mode(0o700))
            .map_err(|e| format!("Failed to secure park directory permissions: {e}"))?;
    }

    Ok(())
}

fn write_park_file_secure(path: &Path, content: &str) -> Result<(), String> {
    if is_symlink(path) {
        return Err("Refusing to write through symlinked park file".to_string());
    }

    let parent = path
        .parent()
        .ok_or_else(|| "Invalid park file path (missing parent)".to_string())?;
    if is_symlink(parent) {
        return Err("Refusing to use symlinked park directory".to_string());
    }

    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp_path = parent.join(format!(".parked.json.tmp-{}-{nonce}", std::process::id()));

    let mut open_opts = std::fs::OpenOptions::new();
    open_opts.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        open_opts.mode(0o600);
    }

    let mut file = open_opts
        .open(&tmp_path)
        .map_err(|e| format!("Failed to create temporary park file: {e}"))?;

    if let Err(e) = file.write_all(content.as_bytes()) {
        let _ = std::fs::remove_file(&tmp_path);
        return Err(format!("Failed to write temporary park file: {e}"));
    }
    if let Err(e) = file.sync_all() {
        let _ = std::fs::remove_file(&tmp_path);
        return Err(format!("Failed to sync temporary park file: {e}"));
    }
    drop(file);

    std::fs::rename(&tmp_path, path).map_err(|e| {
        let _ = std::fs::remove_file(&tmp_path);
        format!("Failed to finalize park file: {e}")
    })?;

    Ok(())
}

pub fn park() {
    let mut app = App::new();
    app.refresh();

    let parked: Vec<ParkedSession> = app
        .sessions
        .iter()
        .filter_map(|s| {
            // Use the session ID from the JSONL filename, not s.session_id.
            // For resumed sessions, s.session_id is a new ID but the JSONL
            // (and `claude --resume`) uses the original session ID.
            let resume_id = s
                .jsonl_path
                .file_stem()
                .and_then(|f| f.to_str())
                .map(|f| f.to_string())
                .unwrap_or_else(|| s.session_id.clone());
            Some(ParkedSession {
                session_id: resume_id,
                tmux_session: s.tmux_session.as_ref()?.clone(),
                cwd: s.cwd.clone(),
            })
        })
        .collect();

    if parked.is_empty() {
        eprintln!("No live sessions to park.");
        return;
    }

    let park_file = ParkFile {
        parked_at: chrono::Utc::now().to_rfc3339(),
        sessions: parked,
    };

    let path = match park_file_path() {
        Some(p) => p,
        None => {
            eprintln!("Could not determine home directory.");
            return;
        }
    };

    if let Some(parent) = path.parent() {
        if let Err(e) = ensure_secure_parent_dir(parent) {
            eprintln!("{e}");
            return;
        }
    }

    let json = match serde_json::to_string_pretty(&park_file) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("Failed to serialize: {e}");
            return;
        }
    };

    if let Err(e) = write_park_file_secure(&path, &json) {
        eprintln!("{e}");
        return;
    }

    eprintln!(
        "Parked {} session(s) to {}",
        park_file.sessions.len(),
        path.display()
    );
    for s in &park_file.sessions {
        eprintln!(
            "  {} ({})",
            s.tmux_session,
            &s.session_id[..8.min(s.session_id.len())]
        );
    }
}

pub fn unpark() {
    let path = match park_file_path() {
        Some(p) => p,
        None => {
            eprintln!("Could not determine home directory.");
            return;
        }
    };

    if is_symlink(&path) {
        eprintln!("Refusing to read symlinked park file.");
        return;
    }

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Nothing parked.");
            return;
        }
    };

    let park_file: ParkFile = match serde_json::from_str(&content) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to read park file: {e}");
            return;
        }
    };

    if park_file.sessions.is_empty() {
        eprintln!("Park file is empty.");
        let _ = std::fs::remove_file(&path);
        return;
    }

    eprintln!(
        "Restoring {} session(s) from {}...",
        park_file.sessions.len(),
        park_file.parked_at
    );

    for s in &park_file.sessions {
        match tmux::resume_session(&s.session_id, Some(&s.tmux_session)) {
            Ok(name) => {
                eprintln!(
                    "  Restored {} ({})",
                    name,
                    &s.session_id[..8.min(s.session_id.len())]
                );
            }
            Err(e) => {
                eprintln!("  Failed to restore {}: {e}", s.tmux_session);
            }
        }
    }

    eprintln!("Done. Park file kept at {}", path.display());
}
