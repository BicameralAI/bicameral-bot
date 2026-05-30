use bicameral_runtime::service;

pub async fn status() -> anyhow::Result<()> {
    let status = service::status();
    println!("Bicameral daemon: {}", status);
    Ok(())
}
