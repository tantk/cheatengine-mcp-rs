# Handover: Rust CE MCP Server

## Current State

**Repo:** `C:\dev\cheatengine-mcp-rs` → https://github.com/tantk/cheatengine-mcp-rs
**Binary:** `target/release/ce-mcp-rs.exe` (3.9MB, compiles clean)
**MCP config:** `.mcp.json` in repo root points to the Rust binary

## What's Done

### Rust MCP Server (complete)
- 24 tools registered and verified via MCP smoke test (`tools/list` returns all 24)
- Async named pipe client with 30s configurable timeout (`CE_PIPE_TIMEOUT` env var)
- Pipe name configurable via `CE_PIPE_NAME` (default: `\\.\pipe\CE_MCP_Bridge_v99`)
- rmcp 0.16 SDK, tokio async runtime, stdio transport for Claude Code
- Uses `Parameters(p): Parameters<T>` pattern for tool params (rmcp 0.16 API)

### Lua Bridge Updates (in cheatenginemcp repo)
- 6 new command handlers added to `C:\dev\cheatenginemcp\MCP_Server\ce_mcp_bridge.lua`
- Handlers registered in `commandHandlers` table at bottom of file
- Autorun script already in place: `C:\Program Files\Cheat Engine\autorun\mcpstart.lua`

### Tool Inventory (24 tools)

**Essential (5):** `ping`, `evaluate_lua`, `get_process_info`, `get_attached_process_quick`, `enum_modules`

**New tools requiring bridge v12 (5):** `get_ce_lua_output`, `execute_ct_entry`, `get_ct_entries`, `freeze_address`, `read_utf16_string`

**Breakpoints (6):** `set_breakpoint`, `set_data_breakpoint`, `remove_breakpoint`, `list_breakpoints`, `clear_all_breakpoints`, `get_breakpoint_hits`

**Analysis (7):** `disassemble`, `get_instruction_info`, `find_function_boundaries`, `analyze_function`, `find_references`, `find_call_references`, `dissect_structure`

**Scanning (1):** `aob_scan`

### Archived Tools (25 removed)
**Redundant — evaluate_lua does it better (20):** `read_memory`, `read_integer`, `read_string`, `read_pointer`, `read_pointer_chain`, `write_integer`, `write_memory`, `write_string`, `scan_all`, `next_scan`, `get_scan_results`, `search_string`, `checksum_memory`, `generate_signature`, `get_memory_regions`, `enum_memory_regions_full`, `get_symbol_address`, `get_address_info`, `get_rtti_classname`, `get_thread_list`

**Dead weight (5):** `auto_assemble` (fails silently via pipe), `get_physical_address`, `start_dbvm_watch`, `stop_dbvm_watch`, `poll_dbvm_watch` (require DBVM)

## What's NOT Done — Testing

### Blocker
The Python MCP server (`mcp_cheatengine.py`) in another Claude Code session holds the CE pipe. Only one client can connect at a time.

### To unblock
1. Close the other Claude Code session that uses the Python CE MCP server
2. Restart Cheat Engine (autorun loads the updated bridge with 6 new handlers)
3. Verify pipe exists: `powershell -Command "Test-Path '\\.\pipe\CE_MCP_Bridge_v99'"`

### Tests to run (in order)
1. `ping` — proves pipe connection works end-to-end
2. `get_attached_process_quick` — proves new bridge handlers work
3. `get_process_info` — verify full process info
4. `evaluate_lua` with `return 1+1` — proves Lua execution
5. `get_ct_entries` — proves cheat table access
6. `execute_ct_entry` — toggle a CT entry
7. `get_ce_lua_output` — verify print capture works
8. `freeze_address` — test address freezing
9. `read_utf16_string` — test with a known game string address

### Timeout test
1. Call `ping` with CE running — should succeed
2. Kill CE process
3. Call `ping` again — should return error within 30s, NOT hang forever

## What Still Needs Doing

### After testing passes
1. Update `.mcp.json` in `C:\dev\longyin-cheats` to point to Rust binary
2. Update `.mcp.json` in `C:\dev\cheatenginemcp` to point to Rust binary
3. Archive Python server: `mv MCP_Server/mcp_cheatengine.py MCP_Server/legacy/`
4. Commit and push both repos
5. Commit and push this repo with test results

## Key Files

| File | What |
|---|---|
| `C:\dev\cheatengine-mcp-rs\src\main.rs` | MCP server entry point (stdio transport) |
| `C:\dev\cheatengine-mcp-rs\src\pipe.rs` | Async named pipe client with timeout |
| `C:\dev\cheatengine-mcp-rs\src\tools.rs` | All 24 tool definitions |
| `C:\dev\cheatengine-mcp-rs\.mcp.json` | MCP config pointing to Rust binary |
| `C:\dev\cheatenginemcp\MCP_Server\ce_mcp_bridge.lua` | Lua bridge (updated with 6 new handlers) |
| `C:\dev\cheatenginemcp\MCP_Server\mcp_cheatengine.py` | OLD Python server (to be archived) |
| `C:\Program Files\Cheat Engine\autorun\mcpstart.lua` | CE autorun that loads the bridge |
