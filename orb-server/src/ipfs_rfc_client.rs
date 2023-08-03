pub async fn save_wasm(app_name: &String, endpoint: &String, data: Vec<u8>) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();

    let route = format!("/orb/endpoint/{}/{}", app_name, endpoint);

    let res = client
        .post("http://localhost:5001/api/v0/files/write")
        .query(&[("arg", route), ("create", "true".to_string()), ("parents", "true".to_string())])
        .multipart(reqwest::multipart::Form::new().part("file", reqwest::multipart::Part::bytes(data)))
        .send()
        .await?;

    println!("Response: {:?}", res);
    println!("Status: {:?}", String::from_utf8(res.bytes().await?.to_vec()));

    Ok(())
}
