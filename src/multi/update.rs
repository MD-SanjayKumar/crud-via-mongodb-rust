use crate::Details;
use mongodb::{
    bson::de::Error,
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use mongoose::Document;
use std::io;

pub async fn update_str() {
    let client_option = ClientOptions::parse_with_resolver_config(
        "mongodb://localhost:27017",
        ResolverConfig::cloudflare(),
    )
    .await
    .expect("Not found");
    let client = Client::with_options(client_option).expect("Not found");
    let data: mongodb::Collection<Document> =
        client.database("crud_operation").collection("details");
    let mut usrnm = String::new();
    let mut usradd = String::new();
    println!("Enter name :");
    io::stdin().read_line(&mut usrnm).expect("Not found");
    println!("Enter new address :");
    io::stdin().read_line(&mut usradd).expect("Not found");

    let fetch: Document = data
        .find_one(
            doc! {
                  "name": usrnm.to_string()
            },
            None,
        )
        .await
        .expect("error found")
        .expect("Missing 'Parasite' document.");

    let update_result = data
        .update_one(
            doc! {
               "_id": &fetch.get("_id")
            },
            doc! {
               "$set": { "address": usradd.to_string() }
            },
            None,
        )
        .await
        .expect("Error found");
    println!("Updated {} document", update_result.modified_count);
}

pub async fn delete_str() {
    let client_option = ClientOptions::parse_with_resolver_config(
        "mongodb://localhost:27017",
        ResolverConfig::cloudflare(),
    )
    .await
    .expect("Not found");
    let client = Client::with_options(client_option).expect("Not found");
    let data: mongodb::Collection<Document> =
        client.database("crud_operation").collection("details");
    let mut usrnm = String::new();
    println!("Enter name :");
    io::stdin().read_line(&mut usrnm).expect("Not found");

    let delete_result = data
        .delete_many(
            doc! {
               "name": usrnm.to_string()
            },
            None,
        )
        .await
        .expect("Error found");
    println!(
        "All records with name :{} are deleted.\nDeleted {} documents",
        usrnm.to_string(),
        delete_result.deleted_count
    );
}
