#![allow(unused)]

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    // hc.do_get("/hello?name=Felix").await?.print().await?;

    hc.do_get("/controls").await?.print().await?;

    let change_modus = hc.do_post(
        "/modus",
        json!({
            "modus": "Continuous"
        }),
    );

    change_modus.await?.print().await?;

    let change_pos = hc.do_post(
        "/position",
        json!({
            "lat": 52.51604,
            "lon": 13.37691
        }),
    );

    change_pos.await?.print().await?;

    Ok(())
}
