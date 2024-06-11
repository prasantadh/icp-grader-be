use std::error::Error;

use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_post(
        "/groups",
        json!({
            "name": "L1C1",
            "year": 2030,
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/groups",
        json!({
            "name": "L1C2",
            "year": 2032,
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_get("/groups").await?.print().await?;
    Ok(())
}
