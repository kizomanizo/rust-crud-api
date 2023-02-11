use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{ extjson::de::Error, oid::ObjectId, doc }, //Add , Document if you decomment patchi method
    results::{ InsertOneResult, UpdateResult, DeleteResult },
    Client, Collection,
};
use futures::stream::TryStreamExt;
use crate::models::user::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        // Hii await hapa chini imeongezwa na linter tu.
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustTestApi");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            email: new_user.email,
            phone: new_user.phone,
            status: new_user.status,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating a new user in MongoDB");
        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's details from MongoDB");
        Ok(user_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "email": new_user.email,
                    "phone": new_user.phone,
                    "status": new_user.status,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn patch_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "email": new_user.email,
                    "phone": new_user.phone,
                    "status": new_user.status,
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    // pub async fn patchi_user(
    //     &self, id: &String,
    //     new_user: User
    // ) -> Result<UpdateResult, Error> {
    //     let obj_id = ObjectId::parse_str(id).unwrap();
    //     let filter = doc! {"_id": obj_id};
    //     let mut doc = Document::new();
    //     if !new_user.name.is_empty() { doc.insert("name", new_user.name) } else {doc.insert("name", filter.get_str("name").to_owned().unwrap())};
    //     if !new_user.email.is_empty() { doc.insert("name", new_user.email) }else {doc.insert("email", filter.get_str("email").to_owned().unwrap())};
    //     if !new_user.phone == 0 { doc.insert("name", new_user.phone) }else {doc.insert("phone", filter.get_i32("phone").to_owned().unwrap())};
    //     if !new_user.status { doc.insert("name", new_user.status) }else {doc.insert("status", filter.get_str("status").to_owned().unwrap())};
    //     let new_doc = doc! { "$set": doc };
    //     let updated_doc = self
    //         .col
    //         .update_one(filter, new_doc, None)
    //         .await
    //         .ok()
    //         .expect("Error updating user");
    //     Ok(updated_doc)
    // }

    // Delete a specific user from MongoDB
    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");
        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
        }

}