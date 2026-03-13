# recon TUI — Implementation Plan

## Context

We have a validated POC (`src/main.rs`, ~360 lines) that proves Warp tab control works via macOS Accessibility API + osascript. Now we need to build the actual TUI dashboard that monitors Claude Code sessions, shows their status/model/token usage, and lets the user jump to any session's Warp tab.

## File Structure

```
src/
  main.rs       -- entry point, terminal setup, event loop
  app.rs        -- App state, refresh logic, key handling
  session.rs    -- Session struct, JSONL parsing, token aggregation
  process.rs    -- claude process discovery via ps, CWD via lsof
  warp.rs       -- AX bindings + osascript tab switching (extracted from POC)
  ui.rs         -- ratatui table rendering, colors, formatting
  model.rs      -- model ID -> display name, context window size
```

## Dependencies (Cargo.toml)

```toml
ratatui = "0.29"
crossterm = { version = "0.28", features = ["event-stream"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time", "process"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dirs = "6"
chrono = "0.4"
core-foundation = "0.10"
core-foundation-sys = "0.8"
```

No linemux/sysinfo/notify — we use `ps`/`lsof` directly and poll every 2s.

## Implementation Steps

### Step 1: Scaffold + extract warp.rs

- Create all module files with stubs
- Move AX helpers, `find_warp_pid`, `switch_to_tab_number`, `find_tab_menu_action`, `perform_action` from `main.rs` → `warp.rs`
- Drop POC-only code: `dump_tree`, `find_tabs_via_window_menu`, CLI arg parsing
- Verify: `cargo build` compiles

### Step 2: model.rs

Simple mappings, no external deps:
- `display_name("claude-opus-4-6")` → `"Opus 4.6"`
- `context_window("claude-opus-4-6")` → `200_000`
- `format_with_effort("claude-opus-4-6", "medium")` → `"Opus 4.6 (medium)"`

### Step 3: process.rs

- `discover_claude_processes()` — run `ps -eo pid,tty,stat,args`, filter for `claude` binary, parse `--resume <session-id>` from args
- `get_process_cwd(pid)` — run `lsof -p <pid> -Fn`, extract CWD
- Cache CWDs (they don't change per session) to avoid repeated lsof calls
- Returns `Vec<ClaudeProcess>` with pid, tty, stat, optional session_id, optional cwd

### Step 4: session.rs

Core data struct:
```rust
struct Session {
    session_id: String,
    project_name: String,       // last path component of cwd
    model: Option<String>,      // raw model ID
    total_input_tokens: u64,    // input + cache_creation + cache_read
    total_output_tokens: u64,
    status: SessionStatus,      // Working/Idle/Input
    pid: i32,
    tty: String,
    last_activity: Option<String>, // ISO timestamp
    jsonl_path: Option<PathBuf>,
    last_file_size: u64,        // for incremental reads
}
```

Session discovery flow:
1. Take `ClaudeProcess` list from process.rs
2. For each: encode CWD to project dir path (`/Users/gavra/repos/yaba` → `-Users-gavra-repos-yaba`)
3. Find matching JSONL in `~/.claude/projects/<encoded>/` — by session_id if known, else most recent by mtime
4. Scan JSONL for token usage + model (full scan first time, incremental after via `last_file_size`)

JSONL parsing optimization:
- Only deserialize lines containing `"type":"assistant"` (quick string check before serde)
- Minimal serde structs with `#[serde(default)]` to skip unknown fields

Status detection:
- `R+` in ps stat → Working
- `S+` + last JSONL entry is `"type":"system","subtype":"turn_duration"` with no subsequent `"type":"user"` → Input (awaiting user)
- `S+` otherwise → Idle

### Step 5: app.rs

```rust
struct App {
    sessions: Vec<Session>,
    selected: usize,
    effort_level: String,   // from ~/.claude/settings.json
    should_quit: bool,
}
```

- `refresh()` — call process discovery → session resolution → JSONL scan → update `self.sessions`
- `handle_key(key)` — j/k/Up/Down for selection, Enter/1-9 for tab jump, r for refresh, q for quit
- `jump_to_session(idx)` — call `warp::switch_to_tab_number(idx + 1)`
- Read `effortLevel` from `~/.claude/settings.json` on startup

### Step 6: ui.rs

Single `render(frame, app)` function. Full-screen `Table` widget.

Columns: `#` | `Project` | `Status` | `Model` | `Tokens` | `Last Activity`

Colors:
- Working → green
- Idle → dark gray
- Input → yellow bold
- Perm → red bold
- Token bar: default <75%, yellow 75-90%, red >90%
- Selected row: dark gray background

Footer: `j/k navigate | Enter/1-9 jump | r refresh | q quit`

### Step 7: main.rs — event loop

```rust
#[tokio::main]
async fn main() {
    // setup terminal (crossterm raw mode + alternate screen)
    // create App, initial refresh
    // loop:
    //   poll crossterm events (200ms timeout)
    //   handle keys
    //   every 2s: refresh
    // restore terminal on exit
}
```

### Step 8: Polish

- Handle edge cases: no claude processes running, Warp not running
- Graceful error display in the TUI (not panics)
- Sort sessions consistently (by TTY name → stable tab order)

## Tab-to-Session Mapping

Sessions are sorted by TTY name (ttys000, ttys002, ...) which gives a stable ordering. Displayed as #1, #2, #3... — pressing that number sends Cmd+N to Warp. This assumes the user's Warp tabs are in matching order, which holds if each tab runs one claude session and they were opened in order. Good enough for v1.

## Token Calculation

For display `"44k / 200k"`:
- Used = sum of `(input_tokens + cache_creation_input_tokens + cache_read_input_tokens + output_tokens)` across all assistant entries
- Window = `model::context_window(model_id)` (200k for all current models)

## Verification

1. `cargo build` — compiles without errors
2. Run `cargo run` with 2+ claude sessions active in Warp tabs
3. Verify: sessions appear in the table with correct project names
4. Verify: status colors update (start a task in one session, see it go green)
5. Verify: token counts are reasonable (compare with `/context` output)
6. Verify: pressing Enter or a number switches to the correct Warp tab
7. Verify: `q` cleanly exits and restores the terminal
