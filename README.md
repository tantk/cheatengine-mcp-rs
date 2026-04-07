# ce-mcp-rs

Rust MCP server for Cheat Engine. Replaces the Python `mcp_cheatengine.py` with native async I/O and proper timeouts.

## Build

```bash
cargo build --release
```

## Usage

Add to `.mcp.json`:
```json
{
  "mcpServers": {
    "cheatengine": {
      "command": "C:/dev/cheatengine-mcp-rs/target/release/ce-mcp-rs.exe"
    }
  }
}
```

Requires the CE Lua bridge (`ce_mcp_bridge.lua`) loaded in Cheat Engine.

## Environment Variables

- `CE_PIPE_NAME` — pipe name (default: `\\.\pipe\CE_MCP_Bridge_v99`)
- `CE_PIPE_TIMEOUT` — read timeout in seconds (default: 30)

## Tools (24)

Trimmed from 43 to 24 based on real-world usage. `evaluate_lua` covers all memory read/write/scan operations more flexibly. 6 new tools added for cheat table control and CJK text.

### Essential

| Tool | Use |
|---|---|
| `evaluate_lua` | Everything — reads, writes, hooks, IL2CPP calls, complex logic |
| `ping` | Connection check |
| `get_process_info` | Verify game is attached |
| `get_attached_process_quick` | Lightweight attached check (PID + name only) |
| `enum_modules` | Confirm DLLs are loaded |

### Cheat table & output (require Lua bridge v12+)

| Tool | Use |
|---|---|
| `get_ce_lua_output` | Read CE Lua console output buffer |
| `execute_ct_entry` | Enable/disable a cheat table entry by name or ID |
| `get_ct_entries` | List all cheat table entries with status |
| `freeze_address` | Freeze a memory address to a value |
| `read_utf16_string` | Read UTF-16LE string (CJK/Unicode game text) |

### Breakpoints

`set_breakpoint`, `set_data_breakpoint`, `remove_breakpoint`, `list_breakpoints`, `clear_all_breakpoints`, `get_breakpoint_hits`

### Analysis & disassembly

`disassemble`, `get_instruction_info`, `find_function_boundaries`, `analyze_function`, `find_references`, `find_call_references`, `dissect_structure`

### Scanning

`aob_scan`

### Archived (use `evaluate_lua` instead)

`read_memory`, `read_integer`, `read_string`, `read_pointer`, `read_pointer_chain`, `write_integer`, `write_memory`, `write_string`, `scan_all`, `next_scan`, `get_scan_results`, `search_string`, `checksum_memory`, `generate_signature`, `get_memory_regions`, `enum_memory_regions_full`, `get_symbol_address`, `get_address_info`, `get_rtti_classname`, `get_thread_list`

### Archived (dead weight)

`auto_assemble` (fails silently via pipe), `get_physical_address`, `start_dbvm_watch`, `stop_dbvm_watch`, `poll_dbvm_watch` (require DBVM — not used)
