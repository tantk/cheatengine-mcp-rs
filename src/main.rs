mod pipe;
mod tools;

use anyhow::Result;
use rmcp::ServiceExt;
use tools::CeServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Log to stderr (stdout is reserved for MCP JSON-RPC)
    tracing_subscriber::fmt()
        .with_env_filter("ce_mcp_rs=info")
        .with_writer(std::io::stderr)
        .init();

    tracing::info!("ce-mcp-rs starting");
    let service = CeServer::new().serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}
