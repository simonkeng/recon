/// Map raw model IDs to human-friendly display names.
pub fn display_name(model_id: &str) -> &str {
    match model_id {
        "claude-opus-4-6" => "Opus 4.6",
        "claude-sonnet-4-6" => "Sonnet 4.6",
        "claude-sonnet-4-5-20250514" => "Sonnet 4.5",
        "claude-haiku-4-5-20251001" => "Haiku 4.5",
        "claude-opus-4-20250514" => "Opus 4",
        "claude-sonnet-4-20250514" => "Sonnet 4",
        _ => model_id,
    }
}

/// Context window size for a given model ID.
pub fn context_window(model_id: &str) -> u64 {
    match model_id {
        "claude-opus-4-6" | "claude-sonnet-4-6" => 200_000,
        "claude-sonnet-4-5-20250514" | "claude-haiku-4-5-20251001" => 200_000,
        "claude-opus-4-20250514" | "claude-sonnet-4-20250514" => 200_000,
        _ => 200_000,
    }
}

/// Format model name with optional effort level.
pub fn format_with_effort(model_id: &str, effort: &str) -> String {
    let name = display_name(model_id);
    if effort.is_empty() || effort == "default" {
        name.to_string()
    } else {
        format!("{name} ({effort})")
    }
}
