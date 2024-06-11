use std::error::Error;

use serde_json::{json, Value};

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

    let req = hc
        .post::<Value>(
            "/groups",
            json!({
                "name": "L1C2",
                "year": 2032,
            }),
        )
        .await?;
    println!("{req}");
    println!("{:}", req["insertedId"]["$oid"].as_str().unwrap());

    hc.do_patch(
        format!("/groups/{}", req["insertedId"]["$oid"].as_str().unwrap()).as_str(),
        json!({"name": "Updated L1C2", "year": 2033}),
    )
    .await?
    .print()
    .await?;

    hc.do_delete(format!("/groups/{}", req["insertedId"]["$oid"].as_str().unwrap()).as_str())
        .await?
        .print()
        .await?;

    hc.do_get("/groups").await?.print().await?;
    Ok(())
}
