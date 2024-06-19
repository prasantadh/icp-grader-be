use std::error::Error;

use mongodb::bson::doc;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/subjects").await?.print().await?;

    /*
    hc.do_post(
        "/teachers",
        json!({
            "name": "teacherett",
            "email": "teacher@icp.edu.np",
        }),
    )
    .await?
    .print()
    .await?;

    hc.do_post(
        "/students",
        json!({
            "name": "studenten",
            "email": "student@icp.edu.np",
            "campus_id": "22345634",
        }),
    )
    .await?
    .print()
    .await?;

    let req: Value = hc
        .post(
            "/teachers",
            json!({
                "name": "teacher2update",
                "email": "teacher2update@icp.edu.np",
            }),
        )
        .await?;

    hc.do_patch(
        format!("/teachers/{}", req["insertedId"]["$oid"].as_str().unwrap()).as_str(),
        json!({"name": "teacherUpdated", "email": "teacherUpdated@icp.edu.np"}),
    )
    .await?
    .print()
    .await?;

    /*
    hc.do_delete(format!("/groups/{}", req["insertedId"]["$oid"].as_str().unwrap()).as_str())
        .await?
        .print()
        .await?;

    hc.do_get("/groups").await?.print().await?;
    */
    hc.do_get("/teachers").await?.print().await?;
    hc.do_get("/students").await?.print().await?;

    let req: Value = hc
        .post(
            "/subjects",
            json!({
                "name": "L1C1",
                "year": 2030,
                "semester": "Fall"
            }),
        )
        .await?;

    hc.do_patch(
        format!("/subjects/{}", req["insertedId"]["$oid"].as_str().unwrap()).as_str(),
        json!({"name": "updatedL1C1", "year": 2031, "semester": "Fall"}),
    )
    .await?
    .print()
    .await?;

    hc.do_get("/subjects").await?.print().await?;
    */

    /*
    let req: Value = hc
        .post(
            "/subjects",
            json!({
                "name": "L1C1",
                "year": 2030,
                "semester": "Fall"
            }),
        )
        .await?;

    hc.do_post(
        "/assessments",
        json! ({
            "name" : "fyp proposal" ,
            "subject_id": req["insertedId"]["$oid"].as_str().unwrap(),
            "questions" : [{
                "content": "What is the purpose of fyp?",
                "full_marks": 100,
                "reductions": [{
                    "reason": "no examples",
                    "marks": 2,
                }]
            }]
        }),
    )
    .await?
    .print()
    .await?;
        */
    Ok(())
}
