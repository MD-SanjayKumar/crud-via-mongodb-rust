use crate::Details;
use crate::Utc;
use mongodb::{
    bson::de::Error,
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use mongoose::Document;
use std::io;

pub async fn conn_str(data: mongodb::Collection<Document>) {
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
    let mut num: u64 = 0;
    match data.count_documents(None, None).await {
        Ok(k) => num = k,
        Err(e) => println!("Error {}", e),
    }

    if usrname.to_string().trim() == ""
        || usrnm.to_string().trim() == ""
        || usradd.to_string().trim() == ""
    {
        println!("Please enter valid input.");
    } else {
        if num == 0 {
            let detail = Details {
                id: None,
                username: usrname,
                name: usrnm,
                address: usradd,
                //datetime: Utc::now(),
                datetime: res.to_string(),
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
        } else {
            match data
                .find_one(
                    doc! {
                          "username": usrname.to_string()
                    },
                    None,
                )
                .await
            {
                Ok(v) => {
                    match v {
                        Some(k) => println!("Username already exists."),
                        None => {
                            let detail = Details {
                                id: None,
                                username: usrname,
                                name: usrnm,
                                address: usradd,
                                //datetime: Utc::now(),
                                datetime: res.to_string(),
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
                }
                Err(e) => println!("Found some error"),
            }
        }
    }
}

pub async fn fetch_data(data: mongodb::Collection<Document>) {
    let mut usrname = String::new();
    println!("Enter username :");
    io::stdin().read_line(&mut usrname).expect("Not found");
    if usrname.to_string().trim() == "" {
        println!("Please enter valid username.");
    } else {
        //     //fetch
        let fetch: Document = data
            .find_one(
                doc! {
                      "username": usrname.to_string()
                },
                None,
            )
            .await
            .unwrap()
            .expect("error found");
        println!(
            "Details -\nName :{:?}\nAddress :{:?}\nDate :{:?}",
            fetch.get_str("name").unwrap().trim(),
            fetch.get_str("address").unwrap().trim(),
            fetch.get_str("datetime").unwrap().trim() //fetch.get_datetime("datetime").unwrap().to_string()
        );
    }
}
