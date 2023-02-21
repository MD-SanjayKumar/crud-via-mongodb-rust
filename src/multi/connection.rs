use crate::Details;
use crate::Utc;
use mongodb::{
    bson::de::Error,
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use mongoose::Document;
use std::{io};

pub async fn conn_str() {
    let client_option = ClientOptions::parse_with_resolver_config(
        "mongodb://localhost:27017",
        ResolverConfig::cloudflare(),
    )
    .await
    .expect("Not found");
    let client = Client::with_options(client_option).expect("Not found");
    let data: mongodb::Collection<Document> =
        client.database("crud_operation_rust").collection("user_data");

    let mut usrnm = String::new();
    let mut usradd = String::new();
    let mut usrname = String::new();
    println!("Enter username :");
    io::stdin().read_line(&mut usrname).expect("Not found");
    println!("Enter name :");
    io::stdin().read_line(&mut usrnm).expect("Not found");
    println!("Enter address :");
    io::stdin().read_line(&mut usradd).expect("Not found");

    let now = chrono::offset::Local::now();
    let res = now.format("%Y-%m-%d %H:%M:%S");
    if usrname.to_string().trim() == "" || usrnm.to_string().trim() == "" || usradd.to_string().trim() == ""{
        println!("Please enter valid input.");
    }else{
    //insert
    let detail = Details {
        id: None,
        username: usrname,
        name: usrnm,
        address: usradd,
        //datetime: Utc::now(),
        datetime: res.to_string()
    };

    let sDetails = bson::to_bson(&detail).expect("Found error");
    let docs = sDetails.as_document().unwrap();

    let result = data
        .insert_one(docs.to_owned(), None)
        .await
        .expect("Found error");
    let sid = result
        .inserted_id
        .as_object_id()
        .expect("Retrieved _id should have been of type ObjectId");
    println!("Record Inserted\n(Document ID: {:?})", sid);
}
}

pub async fn fetch_data() {
    let client_option = ClientOptions::parse_with_resolver_config(
        "mongodb://localhost:27017",
        ResolverConfig::cloudflare(),
    )
    .await
    .expect("Not found");
    let client = Client::with_options(client_option).expect("Not found");
    let data: mongodb::Collection<Document> =
        client.database("crud_operation_rust").collection("user_data");
    let mut usrname = String::new();
    println!("Enter username :");
    io::stdin().read_line(&mut usrname).expect("Not found");
    if usrname.to_string().trim() == ""{
        println!("Please enter valid username.");
    }else{
    //     //fetch
    let fetch: Document = data
        .find_one(
            doc! {
                  "username": usrname.to_string()
            },
            None,
        )
        .await
        .expect("error found")
        .expect("Missing 'Parasite' document.");
    println!(
        "Details -\nName :{:?}\nAddress :{:?}\nDate :{:?}",
        fetch.get_str("name").unwrap().trim(),
        fetch.get_str("address").unwrap().trim(),
        fetch.get_str("datetime").unwrap().trim()
        //fetch.get_datetime("datetime").unwrap().to_string()
    );
}
}
