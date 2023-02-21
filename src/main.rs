use bson::oid::ObjectId;
use mongodb::{
    bson::de::Error,
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use mongoose::Document;
use serde::{Deserialize, Serialize};
use std::io;
#[path = "multi/connection.rs"]
mod connection;
#[path = "multi/update.rs"]
mod update;
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Details {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    username: String,
    name: String,
    address: String,
    // #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    // datetime: chrono::DateTime<Utc>,
    datetime: String,
}

#[tokio::main]
async fn main() {
    let client_option = ClientOptions::parse_with_resolver_config(
        "mongodb://localhost:27017",
        ResolverConfig::cloudflare(),
    )
    .await
    .expect("Not found");
    let client = Client::with_options(client_option).expect("Not found");
    let data: mongodb::Collection<Document> = client
        .database("crud_operation_rust")
        .collection("user_data");
    let mut opt = true;
    while opt {
        let mut choice = String::new();
        println!("- - - - - -\n1.Insert record\n2.Check record\n3.Update record\n4.Delete record\n5.Exit\n- - - - - -\nSelect one option:");
        io::stdin().read_line(&mut choice).expect("Not found");
        match choice.as_str().trim() {
            "1" => connection::conn_str(data.clone()).await,
            "2" => connection::fetch_data(data.clone()).await,
            "3" => update::update_str(data.clone()).await,
            "4" => update::delete_str(data.clone()).await,
            "5" => {
                opt = false;
                println!("Exited");
            }
            _ => println!("Check the input."),
        }
    }
}
