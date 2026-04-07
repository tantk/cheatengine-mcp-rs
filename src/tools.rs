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
pub struct GetSymbolAddressParams {
    #[schemars(description = "Symbol name (e.g., 'Engine.GameEngine')")]
    pub symbol: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetAddressInfoParams {
    #[schemars(description = "Memory address to look up")]
    pub address: String,
    #[schemars(description = "Include module info")]
    #[serde(default = "default_true")]
    pub include_modules: bool,
    #[schemars(description = "Include symbol info")]
    #[serde(default = "default_true")]
    pub include_symbols: bool,
    #[schemars(description = "Include section info")]
    #[serde(default)]
    pub include_sections: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetRttiClassnameParams {
    #[schemars(description = "Address of object to identify")]
    pub address: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReadMemoryParams {
    #[schemars(description = "Memory address to read from")]
    pub address: String,
    #[schemars(description = "Number of bytes to read")]
    #[serde(default = "default_256")]
    pub size: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReadIntegerParams {
    #[schemars(description = "Memory address to read from")]
    pub address: String,
    #[schemars(description = "Type: byte, word, dword, qword, float, double")]
    #[serde(default = "default_dword", rename = "type")]
    pub value_type: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReadStringParams {
    #[schemars(description = "Memory address to read from")]
    pub address: String,
    #[schemars(description = "Maximum string length")]
    #[serde(default = "default_256")]
    pub max_length: i64,
    #[schemars(description = "Read as wide/UTF-16 string")]
    #[serde(default)]
    pub wide: bool,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReadPointerParams {
    #[schemars(description = "Base address")]
    pub address: String,
    #[schemars(description = "Offsets for pointer chain")]
    pub offsets: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ReadPointerChainParams {
    #[schemars(description = "Base address")]
    pub base: String,
    #[schemars(description = "Offsets for pointer chain")]
    pub offsets: Vec<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ChecksumMemoryParams {
    #[schemars(description = "Memory address")]
    pub address: String,
    #[schemars(description = "Size of memory region")]
    pub size: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ScanAllParams {
    #[schemars(description = "Value to scan for")]
    pub value: String,
    #[schemars(description = "Scan type: exact, string, array")]
    #[serde(default = "default_exact", rename = "type")]
    pub scan_type: String,
    #[schemars(description = "Memory protection filter (e.g., '+W-C')")]
    #[serde(default = "default_wc_protection")]
    pub protection: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetScanResultsParams {
    #[schemars(description = "Maximum number of results")]
    #[serde(default = "default_100")]
    pub max: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct NextScanParams {
    #[schemars(description = "Value to scan for")]
    pub value: String,
    #[schemars(description = "Scan type: exact, increased, decreased, changed, unchanged, bigger, smaller")]
    #[serde(default = "default_exact")]
    pub scan_type: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct WriteIntegerParams {
    #[schemars(description = "Memory address to write to")]
    pub address: String,
    #[schemars(description = "Value to write")]
    pub value: serde_json::Value,
    #[schemars(description = "Type: byte, word, dword, qword, float, double")]
    #[serde(default = "default_dword", rename = "type")]
    pub value_type: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct WriteMemoryParams {
    #[schemars(description = "Memory address to write to")]
    pub address: String,
    #[schemars(description = "Raw bytes to write")]
    pub bytes: Vec<i64>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct WriteStringParams {
    #[schemars(description = "Memory address to write to")]
    pub address: String,
    #[schemars(description = "String value to write")]
    pub value: String,
    #[schemars(description = "Write as wide/UTF-16 string")]
    #[serde(default)]
    pub wide: bool,
}

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
pub struct SearchStringParams {
    #[schemars(description = "String to search for in memory")]
    pub string: String,
    #[schemars(description = "Search as wide/UTF-16")]
    #[serde(default)]
    pub wide: bool,
    #[schemars(description = "Maximum results")]
    #[serde(default = "default_100")]
    pub limit: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GenerateSignatureParams {
    #[schemars(description = "Address to generate signature for")]
    pub address: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetMemoryRegionsParams {
    #[schemars(description = "Maximum regions to return")]
    #[serde(default = "default_100")]
    pub max: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EnumMemoryRegionsFullParams {
    #[schemars(description = "Maximum regions to return")]
    #[serde(default = "default_500")]
    pub max: i64,
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
pub struct GetPhysicalAddressParams {
    #[schemars(description = "Virtual address to translate")]
    pub address: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct StartDbvmWatchParams {
    #[schemars(description = "Address to watch")]
    pub address: String,
    #[schemars(description = "Watch mode: 'w' (writes), 'r' (reads), 'x' (execute)")]
    #[serde(default = "default_w")]
    pub mode: String,
    #[schemars(description = "Maximum log entries")]
    #[serde(default = "default_1000")]
    pub max_entries: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct StopDbvmWatchParams {
    #[schemars(description = "Address of watch to stop")]
    pub address: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PollDbvmWatchParams {
    #[schemars(description = "Address of watch to poll")]
    pub address: String,
    #[schemars(description = "Maximum results to return")]
    #[serde(default = "default_1000")]
    pub max_results: i64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct EvaluateLuaParams {
    #[schemars(description = "Lua code to execute in Cheat Engine")]
    pub code: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AutoAssembleParams {
    #[schemars(description = "AutoAssembler script to execute")]
    pub script: String,
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
fn default_500() -> i64 {
    500
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
fn default_1000() -> i64 {
    1000
}
fn default_dword() -> String {
    "dword".into()
}
fn default_exact() -> String {
    "exact".into()
}
fn default_w() -> String {
    "w".into()
}
fn default_wc_protection() -> String {
    "+W-C".into()
}
fn default_x_protection() -> String {
    "+X".into()
}

// ============================================================================
// MCP Tool implementations
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

    #[tool(description = "Run an AutoAssembler script (injection, code caves, etc).")]
    async fn auto_assemble(
        &self,
        Parameters(params): Parameters<AutoAssembleParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "auto_assemble",
                serde_json::json!({"script": params.script}),
            )
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

    #[tool(description = "Get list of threads in the attached process.")]
    async fn get_thread_list(&self) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command("get_thread_list", serde_json::json!({}))
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Resolve a symbol name (e.g., 'Engine.GameEngine') to an address.")]
    async fn get_symbol_address(
        &self,
        Parameters(params): Parameters<GetSymbolAddressParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "get_symbol_address",
                serde_json::json!({"symbol": params.symbol}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get symbolic name and module info for an address.")]
    async fn get_address_info(
        &self,
        Parameters(params): Parameters<GetAddressInfoParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "get_address_info",
                serde_json::json!({
                    "address": params.address,
                    "include_modules": params.include_modules,
                    "include_symbols": params.include_symbols,
                    "include_sections": params.include_sections,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Identify the class name of an object at address using RTTI.")]
    async fn get_rtti_classname(
        &self,
        Parameters(params): Parameters<GetRttiClassnameParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "get_rtti_classname",
                serde_json::json!({"address": params.address}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    // --- MEMORY READING ---

    #[tool(description = "Read raw bytes from memory.")]
    async fn read_memory(
        &self,
        Parameters(params): Parameters<ReadMemoryParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "read_memory",
                serde_json::json!({"address": params.address, "size": params.size}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Read a number from memory. Types: byte, word, dword, qword, float, double.")]
    async fn read_integer(
        &self,
        Parameters(params): Parameters<ReadIntegerParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "read_integer",
                serde_json::json!({"address": params.address, "type": params.value_type}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Read a string from memory (ASCII or Wide/UTF-16).")]
    async fn read_string(
        &self,
        Parameters(params): Parameters<ReadStringParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "read_string",
                serde_json::json!({
                    "address": params.address,
                    "max_length": params.max_length,
                    "wide": params.wide,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Read a pointer chain. Returns the final address and value.")]
    async fn read_pointer(
        &self,
        Parameters(params): Parameters<ReadPointerParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let offsets = params.offsets.unwrap_or_else(|| vec![0]);
        let result = self
            .pipe
            .send_command(
                "read_pointer_chain",
                serde_json::json!({"base": params.address, "offsets": offsets}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Follow a multi-level pointer chain and return analysis of every step.")]
    async fn read_pointer_chain(
        &self,
        Parameters(params): Parameters<ReadPointerChainParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "read_pointer_chain",
                serde_json::json!({"base": params.base, "offsets": params.offsets}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Calculate MD5 checksum of a memory region to detect changes.")]
    async fn checksum_memory(
        &self,
        Parameters(params): Parameters<ChecksumMemoryParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "checksum_memory",
                serde_json::json!({"address": params.address, "size": params.size}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    // --- SCANNING ---

    #[tool(description = "Unified Memory Scanner. Types: exact, string, array. Protection: +W-C.")]
    async fn scan_all(
        &self,
        Parameters(params): Parameters<ScanAllParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "scan_all",
                serde_json::json!({
                    "value": params.value,
                    "type": params.scan_type,
                    "protection": params.protection,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get results from the last scan_all operation.")]
    async fn get_scan_results(
        &self,
        Parameters(params): Parameters<GetScanResultsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "get_scan_results",
                serde_json::json!({"max": params.max}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Next scan to filter results. Types: exact, increased, decreased, changed, unchanged, bigger, smaller.")]
    async fn next_scan(
        &self,
        Parameters(params): Parameters<NextScanParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "next_scan",
                serde_json::json!({"value": params.value, "scan_type": params.scan_type}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    // --- MEMORY WRITING ---

    #[tool(description = "Write a number to memory. Types: byte, word, dword, qword, float, double.")]
    async fn write_integer(
        &self,
        Parameters(params): Parameters<WriteIntegerParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "write_integer",
                serde_json::json!({
                    "address": params.address,
                    "value": params.value,
                    "type": params.value_type,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Write raw bytes to memory.")]
    async fn write_memory(
        &self,
        Parameters(params): Parameters<WriteMemoryParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "write_memory",
                serde_json::json!({"address": params.address, "bytes": params.bytes}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Write a string to memory (ASCII or Wide/UTF-16).")]
    async fn write_string(
        &self,
        Parameters(params): Parameters<WriteStringParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "write_string",
                serde_json::json!({
                    "address": params.address,
                    "value": params.value,
                    "wide": params.wide,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

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

    #[tool(description = "Quickly search for a text string in memory.")]
    async fn search_string(
        &self,
        Parameters(params): Parameters<SearchStringParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "search_string",
                serde_json::json!({
                    "string": params.string,
                    "wide": params.wide,
                    "limit": params.limit,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Generate a unique AOB signature for this specific address.")]
    async fn generate_signature(
        &self,
        Parameters(params): Parameters<GenerateSignatureParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "generate_signature",
                serde_json::json!({"address": params.address}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get list of valid memory regions nearby common bases.")]
    async fn get_memory_regions(
        &self,
        Parameters(params): Parameters<GetMemoryRegionsParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "get_memory_regions",
                serde_json::json!({"max": params.max}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Enumerate ALL memory regions in the process.")]
    async fn enum_memory_regions_full(
        &self,
        Parameters(params): Parameters<EnumMemoryRegionsFullParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "enum_memory_regions_full",
                serde_json::json!({"max": params.max}),
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

    // --- DBVM / HYPERVISOR TOOLS ---

    #[tool(description = "Translate Virtual Address to Physical Address (requires DBVM).")]
    async fn get_physical_address(
        &self,
        Parameters(params): Parameters<GetPhysicalAddressParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "get_physical_address",
                serde_json::json!({"address": params.address}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Start invisible DBVM hypervisor watch. Modes: w, r, x.")]
    async fn start_dbvm_watch(
        &self,
        Parameters(params): Parameters<StartDbvmWatchParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "start_dbvm_watch",
                serde_json::json!({
                    "address": params.address,
                    "mode": params.mode,
                    "max_entries": params.max_entries,
                }),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Stop DBVM watch and return results.")]
    async fn stop_dbvm_watch(
        &self,
        Parameters(params): Parameters<StopDbvmWatchParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "stop_dbvm_watch",
                serde_json::json!({"address": params.address}),
            )
            .await
            .map_err(|e| ce_error(e.to_string()))?;
        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Poll DBVM watch logs without stopping. Returns register state.")]
    async fn poll_dbvm_watch(
        &self,
        Parameters(params): Parameters<PollDbvmWatchParams>,
    ) -> Result<CallToolResult, ErrorData> {
        let result = self
            .pipe
            .send_command(
                "poll_dbvm_watch",
                serde_json::json!({
                    "address": params.address,
                    "max_results": params.max_results,
                }),
            )
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
