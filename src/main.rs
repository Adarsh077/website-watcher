mod mongodb_ops;
use sha2::{Digest, Sha256};

fn get_sha256(html: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(html.as_bytes());
    let result = hasher.finalize();

    format!("{:X}", result)
}

async fn get_result_html() -> Option<String> {
    match reqwest::get("").await {
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
    client.post("").body("Site code updated!").send().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let maybe_html = get_result_html().await;

    match maybe_html {
        Some(html) => {
            let previous_sha_256 = mongodb_ops::get_saved_sha256().await?;

            let new_sha_256 = get_sha256(html);

            if previous_sha_256 != new_sha_256 {
                // TODO: Notify
                mongodb_ops::save_sha256(new_sha_256).await?;

                let _ = send_notification().await;
            }
        }
        None => {
            println!("error")
        }
    }

    Ok(())
}
