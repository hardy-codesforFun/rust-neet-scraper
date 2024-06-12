#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);
    headers.insert("Origin", "null".parse()?);
    headers.insert("Upgrade-Insecure-Requests", "1".parse()?);
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse()?);
    headers.insert("sec-ch-ua", "\"Google Chrome\";v=\"125\", \"Chromium\";v=\"125\", \"Not.A/Brand\";v=\"24\"".parse()?);
    headers.insert("sec-ch-ua-mobile", "?0".parse()?);
    headers.insert("sec-ch-ua-platform", "\"Windows\"".parse()?);

    let mut params = std::collections::HashMap::new();
    params.insert("_csrf-frontend", "rEAl2wPDaRI3dFLzym212Gqqx1TkqqUxLwJBbVIjNgXpcV-0WpQzfHo7HLyVXcKWU8WmLIDGiHdjeCwJEWVjUA==");
    params.insert("Scorecardmodel[ApplicationNumber]", "240411183516");
    params.insert("Scorecardmodel[Day]", "08");
    params.insert("Scorecardmodel[Month]", "03");
    params.insert("Scorecardmodel[Year]", "2007");

    let request = client.request(reqwest::Method::POST, "https://neet.ntaonline.in/frontend/web/scorecard/index")
        .headers(headers)
        .form(&params);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}