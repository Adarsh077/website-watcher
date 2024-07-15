async fn get_result_html() -> Option<String> {
    match reqwest::get("http://www.mumresults.in").await {
        Ok(data) => match data.text().await {
            Ok(text) => Some(text),
            Err(err) => {
                println!("{:?}", err);
                None
            }
        },
        Err(err) => {
            println!("{:?}", err);
            None
        }
    }
}

async fn send_notification() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    client
        .post("https://ntfy.sh/mumresult")
        .body("http://www.mumresults.in/F24/1T01238.pdf")
        .send()
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let maybe_html = get_result_html().await;

    match maybe_html {
        Some(html) => {
            if html.contains("/F24/1T01238") {
                let _ = send_notification().await;
            }
        }
        None => {
            println!("error")
        }
    }

    Ok(())
}
