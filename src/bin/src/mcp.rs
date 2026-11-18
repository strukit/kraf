use kraf_tools::mcp_test::ReadTool;
use rmcp::ServiceExt;
use rmcp::transport::stdio;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("[kraf-mcp] pid={}", std::process::id());

    let service = ReadTool.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
