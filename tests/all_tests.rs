use httpc_test;
use serde_json::json;

#[tokio::test]
async fn tests() {
    let hc = httpc_test::new_client("http://localhost:3000").unwrap();

    //Delete key
    //Set key
    //Get key
    //Set Key
    //Get key
    //Delete key
    //Get key

    //Delete key
    hc.do_delete("/v1/unset/pie").await.unwrap();

    //Set key
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
    let body = res.text_body().unwrap();
    assert_eq!(body, "New Key was set!");

    // Get key
    let res = hc.do_get("/v1/get/pie").await.unwrap();
    let body = res.text_body().unwrap();
    assert_eq!(body, "3.14");

    //Set Key
    let res = hc
        .do_post(
            "/v1/set",
            json!({
                "key": "pie",
                "value": "3.141"}
            ),
        )
        .await
        .unwrap();
    let body = res.text_body().unwrap();
    assert_eq!(body, "Updating the key value from 3.14!");

    // Get key
    let res = hc.do_get("/v1/get/pie").await.unwrap();
    let body = res.text_body().unwrap();
    assert_eq!(body, "3.141");

    // Delete key
    let res = hc.do_delete("/v1/unset/pie").await.unwrap();
    let body = res.text_body().unwrap();
    assert_eq!(body, "Key unset");

    //Get key
    let res = hc.do_get("/v1/get/pie").await.unwrap();
    assert_eq!(res.status().to_string(), "204 No Content");
}
