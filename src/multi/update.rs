use crate::Details;
use mongodb::{
    bson::de::Error,
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use mongoose::Document;
use std::io;

pub async fn update_str(data: mongodb::Collection<Document>) {
    let mut usrname = String::new();
    let mut usrnm = String::new();
    let mut usradd = String::new();
    println!("Enter username :");
    io::stdin().read_line(&mut usrname).expect("Not found");
    if usrname.to_string().trim() == "" {
        println!("Please enter valid username.");
    } else {
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
        let mut case_match = true;
        while case_match {
            let mut v = String::new();
            println!(
                "- - - - - -\n1.Update name\n2.Update address\n- - - - - -\nSelect one option:"
            );
            io::stdin().read_line(&mut v).expect("Not found");
            match v.as_str().trim() {
                "1" => {
                    println!("Enter new name :");
                    io::stdin().read_line(&mut usrnm).expect("Not found");
                    if usrnm.to_string().trim() == "" {
                        println!("Please enter valid name.");
                    } else {
                        let update_result = data
                            .update_one(
                                doc! {
                                   "_id": &fetch.get("_id")
                                },
                                doc! {
                                   "$set": { "name": usrnm.to_string() }
                                },
                                None,
                            )
                            .await
                            .expect("Error found");
                        println!(
                            "Name updated successfully.\nUpdated {} document",
                            update_result.modified_count
                        );
                        case_match = false;
                    }
                }
                "2" => {
                    println!("Enter new address :");
                    io::stdin().read_line(&mut usradd).expect("Not found");
                    if usradd.to_string().trim() == "" {
                        println!("Please enter valid address.");
                    } else {
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
                        println!(
                            "Address updated successfully.\nUpdated {} document",
                            update_result.modified_count
                        );
                        case_match = false;
                    }
                }
                _ => println!("Please select one option."),
            }
        }
    }
}

pub async fn delete_str(data: mongodb::Collection<Document>) {
    let mut usrname = String::new();
    println!("Enter username :");
    io::stdin().read_line(&mut usrname).expect("Not found");
    if usrname.to_string().trim() == "" {
        println!("Please enter valid username.");
    } else {
        let delete_result = data
            .delete_many(
                doc! {
                   "username": usrname.to_string()
                },
                None,
            )
            .await
            .expect("Error found");
        println!(
            "> {}Deleted Successfully.\nDeleted {} documents",
            usrname.to_string(),
            delete_result.deleted_count
        );
    }
}
