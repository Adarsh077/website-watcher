use mongodb::{bson::doc, Client, Collection};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MumResult {
    sha256: String,
}

async fn connect() -> Result<Client, mongodb::error::Error> {
    let uri = "";
    let client = Client::with_uri_str(uri).await?;

    Ok(client)
}

pub async fn get_saved_sha256() -> Result<String, mongodb::error::Error> {
    let client = connect().await?;

    let database = client.database("rust");
    let mum_result_collection: Collection<MumResult> = database.collection("mumresult");

    let maybe_mum_result_document_result = mum_result_collection.find_one(doc! {}).await?;

    match maybe_mum_result_document_result {
        Some(mum_result_document_result) => Ok(mum_result_document_result.sha256),
        None => Ok(String::from("")),
    }
}

pub async fn save_sha256(sha256: String) -> Result<(), mongodb::error::Error> {
    let previous_sha_256 = get_saved_sha256().await?;
    let client = connect().await?;

    let database = client.database("rust");
    let mum_result_collection: Collection<MumResult> = database.collection("mumresult");

    let filter = doc! {};

    if previous_sha_256 != "" {
        let update = doc! {
                "$set": doc!{ "sha256": sha256 }
        };
        mum_result_collection.update_one(filter, update).await?;
    } else {
        mum_result_collection
            .insert_one(MumResult { sha256 })
            .await?;
    }

    Ok(())
}
