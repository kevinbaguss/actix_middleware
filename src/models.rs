use serde::{Deserialize, Serialize};


#[derive(Debug,Deserialize,Serialize,sqlx::FromRow)]
pub struct Admin{
    pub id:i32,
    pub username:String,
    pub password:String,

}

#[derive(Debug,Deserialize)]
pub struct Login{
    pub username:String,
    pub password:String,
}

#[derive(Debug, Deserialize)]
pub struct UserInput{
    pub nama: String,
    pub jabatan: String,

}

#[derive(Serialize)]
pub struct User{
    pub id:i32,
    pub nama:String,
    pub jabatan:String,
    
}

#[derive(Deserialize)]
pub struct Update{
    pub nama:String,
    pub jabatan:String,
}

#[derive(Serialize)]
pub struct Find{
    pub nama:String,
    pub jabatan:String,
}