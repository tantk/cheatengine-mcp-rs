# CE MCP Tool Review

Based on extensive cheat development for LongYinLiZhiZhuan (IL2CPP Unity game) — hundreds of tool calls across game analysis, cheat creation, debugging, and testing.

## Usage Reality

`evaluate_lua` does 95% of the work. It's the only tool that supports complex multi-step operations (IL2CPP class resolution, pointer chains, conditional logic, loops). The other 42 tools are convenience wrappers that are less capable.

## Tool Categories

### Essential (used constantly)

| Tool | Why |
|---|---|
| `evaluate_lua` | Everything — reads, writes, hooks, IL2CPP calls, complex logic |
| `ping` | Connection check before any operation |
| `get_process_info` | Verify game is attached |
| `enum_modules` | Confirm GameAssembly.dll is loaded |

### Useful (used occasionally)

| Tool | When |
|---|---|
| `set_data_breakpoint` | Find what writes to a memory address |
| `get_breakpoint_hits` | Read breakpoint results |
| `remove_breakpoint` | Clean up |
| `list_breakpoints` | Check active breakpoints |
| `clear_all_breakpoints` | Reset |
| `aob_scan` | Find byte patterns (though evaluate_lua can also do this) |

### Redundant (evaluate_lua does it better)

| Tool | Why redundant |
|---|---|
| `read_memory` | `evaluate_lua` with `readBytes()` — more flexible, can process results inline |
| `read_integer` | `evaluate_lua` with `readInteger()`/`readFloat()`/`readQword()` |
| `read_string` | `evaluate_lua` with `readString()` |
| `read_pointer` / `read_pointer_chain` | `evaluate_lua` with chained `readQword()` calls — can add null checks |
| `write_integer` / `write_memory` / `write_string` | `evaluate_lua` with `writeInteger()`/`writeBytes()` |
| `scan_all` / `next_scan` / `get_scan_results` | `evaluate_lua` with `AOBScan()`/`createMemScan()` — more control |
| `search_string` | `evaluate_lua` with CE's search functions |
| `checksum_memory` | Never needed |
| `generate_signature` | Never needed |
| `get_memory_regions` / `enum_memory_regions_full` | Never needed |
| `get_symbol_address` / `get_address_info` | `evaluate_lua` with `getAddress()` |
| `get_rtti_classname` | Never needed (IL2CPP has its own metadata) |
| `get_thread_list` | Never needed |

### Redundant (decomps replace them)

| Tool | Why redundant |
|---|---|
| `disassemble` | We have 78K resolved decomps + dump.cs — never need live disassembly |
| `get_instruction_info` | Same |
| `find_function_boundaries` | Decomps show full function bodies |
| `analyze_function` | Call graph + decomps |
| `find_references` | Decomps + grep |
| `find_call_references` | GitNexus call graph |
| `dissect_structure` | dump.cs has all struct definitions with offsets |

### Dead Weight

| Tool | Why |
|---|---|
| `get_physical_address` | Requires DBVM hypervisor — not used |
| `start_dbvm_watch` / `stop_dbvm_watch` / `poll_dbvm_watch` | Same |
| `auto_assemble` | Known to fail silently via MCP pipe — we use evaluate_lua to write bytes directly |

## Missing Tools (should add)

### 1. `get_ce_lua_output`
**Problem:** When cheats run from the CT, print statements go to CE's Lua console. We can't read them via MCP — no way to verify cheat success programmatically.
**Solution:** Read CE's Lua output buffer.
**Requires:** Lua bridge change to capture and expose console output.

### 2. `execute_ct_entry`
**Problem:** Had to tell the user to manually click cheats in CE. Can't toggle CT entries programmatically.
**Solution:** Enable/disable a cheat table entry by name or ID.
**Requires:** Lua bridge change — `memrec.Active = true/false` on AddressList entries.

### 3. `get_ct_entries`
**Problem:** Couldn't see what cheats are active without asking the user.
**Solution:** List all cheat table entries with ID, name, and enabled/disabled status.
**Requires:** Lua bridge change — iterate `AddressList.Count` and read each entry.

### 4. `freeze_address`
**Problem:** Max weight cheat — game kept overwriting our value every frame. CE has native freeze but no MCP tool.
**Solution:** Add/remove address to CE's address list with freeze option.
**Requires:** Lua bridge change — `addresslist_createMemoryRecord()` with freeze flag.

### 5. `read_utf16_string`
**Problem:** Reading Chinese text from game memory was painful. Had to read char-by-char via `readSmallInteger` in evaluate_lua.
**Solution:** Dedicated tool that reads UTF-16LE and returns decoded text.
**Requires:** Lua bridge change — read bytes and decode as UTF-16.

### 6. `get_attached_process_quick`
**Problem:** `get_process_info` returns too much data (full module list). Just need "yes/no attached + process name".
**Solution:** Lightweight check — `getOpenedProcessID()` + `process` in Lua.
**Requires:** Lua bridge change (trivial).

## Recommendation for Rust Server

1. **Implement all 43 tools** for backward compatibility
2. **Add the 6 missing tools** as Lua bridge + MCP additions (separate task)
3. In README, document which tools are essential vs redundant
4. Long-term: consider removing dead weight tools to reduce Claude's tool context overhead (43 tools is a lot of context for every MCP call)
