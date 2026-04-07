use crate::pipe::PipeClient;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::{schemars, tool, tool_handler, tool_router, ServerHandler};
use serde::Deserialize;
use std::borrow::Cow;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CeServer {
    pipe: Arc<PipeClient>,
    tool_router: rmcp::handler::server::router::tool::ToolRouter<CeServer>,
}

fn ce_error(msg: String) -> ErrorData {
    ErrorData {
        code: ErrorCode(-32603),
        message: Cow::from(msg),
        data: None,
    }
}

// ============================================================================
// Parameter structs
// ============================================================================

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AobScanParams {
    #[schemars(description = "AOB pattern (e.g., '48 89 5C 24')")]
    pub pattern: String,
    #[schemars(description = "Memory protection filter (e.g., '+X')")]
    #[serde(default = "default_x_protection")]
    pub protection: String,
    #[schemars(description = "Maximum results")]
    #[serde(default = "default_100")]
    pub limit: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DisassembleParams {
    #[schemars(description = "Address to disassemble from")]
    pub address: String,
    #[schemars(description = "Number of instructions")]
    #[serde(default = "default_20")]
    pub count: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetInstructionInfoParams {
    #[schemars(description = "Address of instruction")]
    pub address: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FindFunctionBoundariesParams {
    #[schemars(description = "Address within the function")]
    pub address: String,
    #[schemars(description = "Maximum bytes to search")]
    #[serde(default = "default_4096")]
    pub max_search: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AnalyzeFunctionParams {
    #[schemars(description = "Address of function to analyze")]
    pub address: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FindReferencesParams {
    #[schemars(description = "Address to find references to")]
    pub address: String,
    #[schemars(description = "Maximum results")]
    #[serde(default = "default_50")]
    pub limit: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FindCallReferencesParams {
    #[schemars(description = "Function address to find callers of")]
    pub function_address: String,
    #[schemars(description = "Maximum results")]
    #[serde(default = "default_100")]
    pub limit: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DissectStructureParams {
    #[schemars(description = "Address of structure")]
    pub address: String,
    #[schemars(description = "Size of structure")]
    #[serde(default = "default_256")]
    pub size: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SetBreakpointParams {
    #[schemars(description = "Address to set breakpoint at")]
    pub address: String,
    #[schemars(description = "Breakpoint identifier")]
    pub id: Option<String>,
    #[schemars(description = "Capture register state on hit")]
    #[serde(default = "default_true")]
    pub capture_registers: bool,
    #[schemars(description = "Capture stack on hit")]
    #[serde(default)]
    pub capture_stack: bool,
    #[schemars(description = "Stack depth to capture")]
    #[serde(default = "default_16")]
    pub stack_depth: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SetDataBreakpointParams {
    #[schemars(description = "Address to watch")]
    pub address: String,
    #[schemars(description = "Breakpoint identifier")]
    pub id: Option<String>,
    #[schemars(description = "Access type: 'r' (read), 'w' (write), 'rw' (access)")]
    #[serde(default = "default_w")]
    pub access_type: String,
    #[schemars(description = "Size of watched region in bytes")]
    #[serde(default = "default_4")]
    pub size: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RemoveBreakpointParams {
    #[schemars(description = "Breakpoint ID to remove")]
    pub id: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetBreakpointHitsParams {
    #[schemars(description = "Breakpoint ID (or all if omitted)")]
    pub id: Option<String>,
    #[schemars(description = "Clear hit buffer after reading")]
    #[serde(default)]
    pub clear: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EvaluateLuaParams {
    #[schemars(description = "Lua code to execute in Cheat Engine")]
    pub code: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetCeLuaOutputParams {
    #[schemars(description = "If true, clear the output buffer after reading")]
    #[serde(default)]
    pub clear: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ExecuteCtEntryParams {
    #[schemars(description = "Cheat table entry name or ID")]
    pub entry: String,
    #[schemars(description = "true to enable, false to disable")]
    pub active: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FreezeAddressParams {
    #[schemars(description = "Memory address to freeze")]
    pub address: String,
    #[schemars(description = "Value to freeze at")]
    pub value: serde_json::Value,
    #[schemars(description = "Value type: byte, word, dword, qword, float, double")]
    #[serde(default = "default_dword", rename = "type")]
    pub value_type: String,
    #[schemars(description = "false to unfreeze/remove")]
    #[serde(default = "default_true")]
    pub freeze: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReadUtf16StringParams {
    #[schemars(description = "Memory address to read from")]
    pub address: String,
    #[schemars(description = "Maximum number of characters to read")]
    #[serde(default = "default_256")]
    pub max_length: i64,
}

// ============================================================================
// Default value functions
// ============================================================================

fn default_true() -> bool {
    true
}
fn default_256() -> i64 {
    256
}
fn default_100() -> i64 {
    100
}
fn default_20() -> i64 {
    20
}
fn default_50() -> i64 {
    50
}
fn default_4096() -> i64 {
    4096
}
fn default_16() -> i64 {
    16
}
fn default_4() -> i64 {
    4
}
fn default_dword() -> String {
    "dword".into()
}
fn default_w() -> String {
    "w".into()
}
fn default_x_protection() -> String {
    "+X".into()
}

// ============================================================================
// MCP Tool implementations (29 tools)
// ============================================================================

#[tool_router]
impl CeServer {
    pub fn new() -> Self {
        Self {
            pipe: Arc::new(PipeClient::new()),
            tool_router: Self::tool_router(),
        }
    }

    // --- CONNECTIVITY ---

    #[tool(description = "Check connectivity and get version info.")]
    async fn ping(&self) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command("ping", serde_json::json!({}))
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    // --- SCRIPTING & CONTROL ---

    #[tool(description = "Execute arbitrary Lua code in Cheat Engine.")]
    async fn evaluate_lua(
        &self,
        Parameters(params): Parameters<EvaluateLuaParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command("evaluate_lua", serde_json::json!({"code": params.code}))
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    // --- PROCESS & MODULES ---

    #[tool(description = "Get current process ID, name, modules count and architecture.")]
    async fn get_process_info(&self) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command("get_process_info", serde_json::json!({}))
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "List all loaded modules (DLLs) with their base addresses and sizes.")]
    async fn enum_modules(&self) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command("enum_modules", serde_json::json!({}))
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    // --- SCANNING ---

    #[tool(description = "Scan for an Array of Bytes pattern. Example: '48 89 5C 24'.")]
    async fn aob_scan(
        &self,
        Parameters(params): Parameters<AobScanParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "aob_scan",
                serde_json::json!({
                    "pattern": params.pattern,
                    "protection": params.protection,
                    "limit": params.limit,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    // --- ANALYSIS & DISASSEMBLY ---

    #[tool(description = "Disassemble instructions starting at an address.")]
    async fn disassemble(
        &self,
        Parameters(params): Parameters<DisassembleParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "disassemble",
                serde_json::json!({"address": params.address, "count": params.count}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get detailed info about a single instruction (size, bytes, opcode).")]
    async fn get_instruction_info(
        &self,
        Parameters(params): Parameters<GetInstructionInfoParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "get_instruction_info",
                serde_json::json!({"address": params.address}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Find the start and end of a function containing the address.")]
    async fn find_function_boundaries(
        &self,
        Parameters(params): Parameters<FindFunctionBoundariesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "find_function_boundaries",
                serde_json::json!({
                    "address": params.address,
                    "max_search": params.max_search,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Analyze a function to find all CALL instructions it makes.")]
    async fn analyze_function(
        &self,
        Parameters(params): Parameters<AnalyzeFunctionParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "analyze_function",
                serde_json::json!({"address": params.address}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Find instructions that access (reference) this address.")]
    async fn find_references(
        &self,
        Parameters(params): Parameters<FindReferencesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "find_references",
                serde_json::json!({"address": params.address, "limit": params.limit}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Find all locations that CALL this function.")]
    async fn find_call_references(
        &self,
        Parameters(params): Parameters<FindCallReferencesParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "find_call_references",
                serde_json::json!({"address": params.function_address, "limit": params.limit}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Use CE auto-guess to interpret memory at address as a structure.")]
    async fn dissect_structure(
        &self,
        Parameters(params): Parameters<DissectStructureParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "dissect_structure",
                serde_json::json!({"address": params.address, "size": params.size}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    // --- DEBUGGING & BREAKPOINTS ---

    #[tool(description = "Set a hardware execution breakpoint. Non-breaking/Logging only.")]
    async fn set_breakpoint(
        &self,
        Parameters(params): Parameters<SetBreakpointParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "set_breakpoint",
                serde_json::json!({
                    "address": params.address,
                    "id": params.id,
                    "capture_registers": params.capture_registers,
                    "capture_stack": params.capture_stack,
                    "stack_depth": params.stack_depth,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Set a hardware data breakpoint (watchpoint). Types: r, w, rw.")]
    async fn set_data_breakpoint(
        &self,
        Parameters(params): Parameters<SetDataBreakpointParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "set_data_breakpoint",
                serde_json::json!({
                    "address": params.address,
                    "id": params.id,
                    "access_type": params.access_type,
                    "size": params.size,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Remove a breakpoint by its ID.")]
    async fn remove_breakpoint(
        &self,
        Parameters(params): Parameters<RemoveBreakpointParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "remove_breakpoint",
                serde_json::json!({"id": params.id}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "List all active breakpoints.")]
    async fn list_breakpoints(&self) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command("list_breakpoints", serde_json::json!({}))
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Remove ALL breakpoints.")]
    async fn clear_all_breakpoints(&self) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command("clear_all_breakpoints", serde_json::json!({}))
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get breakpoint hits. Set clear=true to flush buffer.")]
    async fn get_breakpoint_hits(
        &self,
        Parameters(params): Parameters<GetBreakpointHitsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "get_breakpoint_hits",
                serde_json::json!({"id": params.id, "clear": params.clear}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    // --- NEW TOOLS (require Lua bridge v12+ support) ---

    #[tool(description = "Read CE Lua console output buffer. Use to verify cheat/script results.")]
    async fn get_ce_lua_output(
        &self,
        Parameters(params): Parameters<GetCeLuaOutputParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "get_lua_output",
                serde_json::json!({"clear": params.clear}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Enable or disable a cheat table entry by name or ID.")]
    async fn execute_ct_entry(
        &self,
        Parameters(params): Parameters<ExecuteCtEntryParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "execute_ct_entry",
                serde_json::json!({"entry": params.entry, "active": params.active}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "List all cheat table entries with ID, name, and enabled/disabled status.")]
    async fn get_ct_entries(&self) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command("get_ct_entries", serde_json::json!({}))
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Freeze a memory address to a value (CE keeps writing it every tick). Set freeze=false to unfreeze.")]
    async fn freeze_address(
        &self,
        Parameters(params): Parameters<FreezeAddressParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "freeze_address",
                serde_json::json!({
                    "address": params.address,
                    "value": params.value,
                    "type": params.value_type,
                    "freeze": params.freeze,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Read a UTF-16LE string from memory and return decoded text. For CJK/Unicode game strings.")]
    async fn read_utf16_string(
        &self,
        Parameters(params): Parameters<ReadUtf16StringParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "read_utf16_string",
                serde_json::json!({
                    "address": params.address,
                    "max_length": params.max_length,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Quick check if a process is attached. Returns process ID and name only.")]
    async fn get_attached_process_quick(&self) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command("get_attached_process_quick", serde_json::json!({}))
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }
}

#[tool_handler]
impl ServerHandler for CeServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "cheatengine".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                title: None,
                description: None,
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "Cheat Engine MCP Bridge — read/write memory, scan, debug, execute Lua".into(),
            ),
        }
    }
}
