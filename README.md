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

## Tools (43)

All tools from the Python server are implemented:

**Process & Modules:** `get_process_info`, `enum_modules`, `get_thread_list`, `get_symbol_address`, `get_address_info`, `get_rtti_classname`

**Memory Reading:** `read_memory`, `read_integer`, `read_string`, `read_pointer`, `read_pointer_chain`, `checksum_memory`

**Scanning:** `scan_all`, `get_scan_results`, `next_scan`, `aob_scan`, `search_string`, `generate_signature`, `get_memory_regions`, `enum_memory_regions_full`

**Memory Writing:** `write_integer`, `write_memory`, `write_string`

**Analysis & Disassembly:** `disassemble`, `get_instruction_info`, `find_function_boundaries`, `analyze_function`, `find_references`, `find_call_references`, `dissect_structure`

**Debugging & Breakpoints:** `set_breakpoint`, `set_data_breakpoint`, `remove_breakpoint`, `list_breakpoints`, `clear_all_breakpoints`, `get_breakpoint_hits`

**DBVM / Hypervisor:** `get_physical_address`, `start_dbvm_watch`, `stop_dbvm_watch`, `poll_dbvm_watch`

**Scripting & Control:** `evaluate_lua`, `auto_assemble`, `ping`
