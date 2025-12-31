---
task_id: TASK_001
title: Research and Implement OpenCode Default Model Detection
project: context-driven-development
created: "2024-12-31"
status: completed
---

# TASK_001: Research and Implement OpenCode Default Model Detection

## Summary

Researched the OpenCode codebase to understand how it determines which model to use for new sessions, then implemented automatic detection of the user's last used model to pass when launching OpenCode from the `cdd run` command. This ensures the user's preferred model (e.g., Claude Sonnet 4.5) is used instead of a hardcoded default.

## Goals

- ✅ Understand how OpenCode determines the default model for new sessions
- ✅ Find where the last used model is stored
- ✅ Implement automatic detection and passing of the user's preferred model
- ✅ Avoid hardcoding model preferences in CDD

## Research Findings

### Model Selection Priority in OpenCode

OpenCode determines the model through this priority chain:

1. **Explicit model parameter** (if provided when creating the session)
2. **Last used model in the session** (for continuing sessions)
3. **Config file `model` setting** (`~/.config/opencode/opencode.json`)
4. **Last used model from state** (`~/.local/state/opencode/model.json` - TUI only)
5. **First available provider's best model** (sorted by priority)

### Key Storage Locations

#### 1. TUI State File (Most Recent Models)
**Location:** `~/.local/state/opencode/model.json`

**Structure:**
```json
{
  "recent": [
    {"providerID": "anthropic", "modelID": "claude-sonnet-4-5"},
    {"providerID": "openai", "modelID": "gpt-5-chat"}
  ],
  "favorite": [
    {"providerID": "anthropic", "modelID": "claude-sonnet-4-5"}
  ],
  "variant": {}
}
```

**Key Finding:** The **first item in the `recent` array** is the last used model.

#### 2. Config File (Explicit Default)
**Location:** `~/.config/opencode/opencode.json`

**Structure:**
```json
{
  "model": "anthropic/claude-sonnet-4-5-20250514"
}
```

#### 3. Session Messages (Per-Session Model)
**Location:** `~/.local/share/opencode/storage/message/`

Each message stores the model used, but querying is slow due to thousands of files.

### Relevant OpenCode Source Code

#### Model Resolution Logic
**File:** `/packages/opencode/src/session/prompt.ts` (lines 566-571)

```typescript
async function lastModel(sessionID: string) {
  for await (const item of MessageV2.stream(sessionID)) {
    if (item.info.role === "user" && item.info.model) return item.info.model
  }
  return Provider.defaultModel()
}
```

#### Default Model Selection
**File:** `/packages/opencode/src/provider/provider.ts` (lines 1054-1067)

```typescript
export async function defaultModel() {
  const cfg = await Config.get()
  if (cfg.model) return parseModel(cfg.model)

  const provider = await list()
    .then((val) => Object.values(val))
    .then((x) => x.find((p) => !cfg.provider || Object.keys(cfg.provider).includes(p.id)))
  if (!provider) throw new Error("no providers found")
  const [model] = sort(Object.values(provider.models))
  if (!model) throw new Error("no models found")
  return {
    providerID: provider.id,
    modelID: model.id,
  }
}
```

**Model Priority:** `["gpt-5", "claude-sonnet-4", "big-pickle", "gemini-3-pro"]`

#### TUI Model Store
**File:** `/packages/opencode/src/cli/cmd/tui/context/local.tsx` (lines 114-137)

Manages the `~/.local/state/opencode/model.json` file with recent and favorite models.

## Implementation

### Changes Made

1. **Added `get_opencode_last_model()` function** in `src/main.rs`:
   - Reads `~/.local/state/opencode/model.json`
   - Parses JSON to extract `recent[0]`
   - Returns model in format `"providerID/modelID"`
   - Returns `None` if file doesn't exist

2. **Updated OpenCode launch logic**:
   - If last model is detected → pass `--model anthropic/claude-sonnet-4-5`
   - If not found → let OpenCode use its default

3. **Added dependencies**:
   - `serde_json = "1.0"` for JSON parsing
   - `dirs = "5.0"` for cross-platform home directory resolution

### Code Example

```rust
fn get_opencode_last_model() -> Option<String> {
    let state_file = dirs::home_dir()?.join(".local/state/opencode/model.json");
    
    if !state_file.exists() {
        return None;
    }
    
    let content = fs::read_to_string(&state_file).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    
    let recent = json.get("recent")?.as_array()?;
    let last_model = recent.first()?;
    
    let provider_id = last_model.get("providerID")?.as_str()?;
    let model_id = last_model.get("modelID")?.as_str()?;
    
    Some(format!("{}/{}", provider_id, model_id))
}
```

## File System Diff

```
context-driven-development/
├── Cargo.toml
│   └── + Added dependencies: serde_json = "1.0", dirs = "5.0"
├── Cargo.lock
│   └── ~ Updated with new dependency tree
└── src/
    └── main.rs
        ├── + Added get_opencode_last_model() function
        ├── ~ Updated OpenCode launch to use detected model
        └── + Added imports: use std::path::Path
```

## Lessons Learned

1. **OpenCode stores model preferences in multiple places**:
   - TUI state for recent/favorite models
   - Config file for explicit defaults
   - Session messages for per-session tracking

2. **The `~/.local/state/opencode/model.json` file is the most reliable source** for the user's current preference, as it's updated every time they switch models in the TUI.

3. **Model format differs between storage and CLI**:
   - Storage: `{"providerID": "anthropic", "modelID": "claude-sonnet-4-5"}`
   - CLI flag: `--model anthropic/claude-sonnet-4-5`

4. **Graceful fallback is important**: If we can't detect the model, OpenCode will use its own defaults, so the feature degrades gracefully.

## Validation

- ✅ `cargo build` - Builds without errors
- ✅ `cargo clippy` - No warnings
- ✅ Manual test: Read `~/.local/state/opencode/model.json` and verify correct model is extracted
- ✅ `cdd` launches OpenCode with `--model anthropic/claude-sonnet-4-5` when model.json exists
- ✅ `cdd` launches OpenCode without `--model` flag when model.json doesn't exist (graceful fallback)

## References

- OpenCode GitHub: https://github.com/sst/opencode
- Key source files analyzed:
  - `/packages/opencode/src/session/prompt.ts`
  - `/packages/opencode/src/provider/provider.ts`
  - `/packages/opencode/src/cli/cmd/tui/context/local.tsx`
  - `/packages/opencode/src/global/index.ts`
