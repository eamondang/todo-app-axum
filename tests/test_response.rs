use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn test_response() -> Result<()> {
  let res = httpc_test::new_client("http://localhost:8000")?;

  res.do_get("/").await?.print().await?;

  let req_login = res.do_post(
    "/api/login",
    json!({
      "username" : "eamon",
      "pwd" : "1234",
    }),
  );
  req_login.await?.print().await?;

  Ok(())
}
