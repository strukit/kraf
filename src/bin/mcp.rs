use kraf::tools::mcp_test::ReadTool;
use rmcp::transport::stdio;
use rmcp::ServiceExt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("[kraf-mcp] pid={}", std::process::id());

    let service = ReadTool.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
