use anyhow::{anyhow, Result};
use serde_json::Value;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::windows::named_pipe::ClientOptions;
use tokio::time::timeout;

const DEFAULT_PIPE_NAME: &str = r"\\.\pipe\CE_MCP_Bridge_v99";

#[derive(Debug)]
pub struct PipeClient {
    pipe_name: String,
    timeout: Duration,
}

impl PipeClient {
    pub fn new() -> Self {
        let pipe_name =
            std::env::var("CE_PIPE_NAME").unwrap_or_else(|_| DEFAULT_PIPE_NAME.to_string());
        let timeout_secs: u64 = std::env::var("CE_PIPE_TIMEOUT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        Self {
            pipe_name,
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    pub async fn send_command(&self, method: &str, params: Value) -> Result<String> {
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
        });

        let req_bytes = serde_json::to_vec(&request)?;
        let header = (req_bytes.len() as u32).to_le_bytes();

        // Connect to pipe
        let mut pipe = timeout(self.timeout, async {
            loop {
                match ClientOptions::new().open(&self.pipe_name) {
                    Ok(pipe) => return Ok(pipe),
                    Err(e) if e.raw_os_error() == Some(231) => {
                        // ERROR_PIPE_BUSY — retry after short delay
                        tokio::time::sleep(Duration::from_millis(50)).await;
                    }
                    Err(e) => {
                        return Err(anyhow!(
                            "CE Bridge not running (pipe not found): {}",
                            e
                        ))
                    }
                }
            }
        })
        .await
        .map_err(|_| anyhow!("Timeout connecting to CE pipe"))??;

        // Write request
        pipe.write_all(&header).await?;
        pipe.write_all(&req_bytes).await?;

        // Read response header (4 bytes, with timeout)
        let mut resp_header = [0u8; 4];
        timeout(self.timeout, pipe.read_exact(&mut resp_header))
            .await
            .map_err(|_| {
                anyhow!(
                    "CE pipe read timed out after {}s. CE may be frozen or Lua bridge stuck.",
                    self.timeout.as_secs()
                )
            })??;

        let resp_len = u32::from_le_bytes(resp_header) as usize;
        if resp_len > 16 * 1024 * 1024 {
            return Err(anyhow!("Response too large: {} bytes", resp_len));
        }

        // Read response body (with timeout)
        let mut resp_body = vec![0u8; resp_len];
        timeout(self.timeout, pipe.read_exact(&mut resp_body))
            .await
            .map_err(|_| {
                anyhow!(
                    "CE pipe read timed out after {}s reading body ({} bytes expected)",
                    self.timeout.as_secs(),
                    resp_len
                )
            })??;

        let response: Value = serde_json::from_slice(&resp_body).map_err(|e| {
            anyhow!(
                "Invalid JSON from CE: {} (first 200 bytes: {:?})",
                e,
                String::from_utf8_lossy(&resp_body[..resp_body.len().min(200)])
            )
        })?;

        if let Some(error) = response.get("error") {
            return Ok(
                serde_json::json!({"success": false, "error": error.to_string()}).to_string(),
            );
        }
        if let Some(result) = response.get("result") {
            return Ok(result.to_string());
        }
        Ok(response.to_string())
    }
}
