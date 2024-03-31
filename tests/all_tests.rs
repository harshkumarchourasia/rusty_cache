use httpc_test;
use serde_json::json;
use std::fmt::Error;

#[tokio::test]
async fn set_key_test() {
    let hc = httpc_test::new_client("http://localhost:3000").unwrap();
    let res = hc
        .do_post(
            "/v1/set",
            json!({
                "key": "pie",
                "value": "3.14"}
            ),
        )
        .await
        .unwrap();

    // Extract the response body
    let body = res.text_body().unwrap();
    assert_eq!(body, "New Key was set!");

    let res = hc
        .do_post(
            "/v1/set",
            json!({
                "key": "pie",
                "value": "3.14"}
            ),
        )
        .await
        .unwrap();

    // Extract the response body
    let body = res.text_body().unwrap();
    assert_eq!(body, "Updating the key value from 3.14!")
}
