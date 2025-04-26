 
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx:: MySqlPool;
use crate::models::{User, UserInput, Update, Find};

//memasukan input ke database

#[post("/user")]
pub async fn insert_user(
    user_data: web ::Json<UserInput>,
    db: web::Data<MySqlPool>,

) -> impl Responder{

if user_data.nama.trim().is_empty() || user_data.jabatan.trim().is_empty() {
    return HttpResponse::BadRequest().body("Nama dan jabatan tidak boleh kosong");
}
let result= sqlx::query!(
    "INSERT INTO rustdb (nama,jabatan) VALUES (?,?)",
    user_data.nama,
    user_data.jabatan,
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("data user berhasil ditambah"),
        Err(e) =>{
            println!("insert gagal: {:?}",e);
            HttpResponse::InternalServerError().body("gagal menambah user")
        }
    }

}

 //melihat semua user

#[get("/user")]
pub async fn get_all_user(db: web::Data<MySqlPool>) -> impl Responder{
    let result = sqlx::query_as!(
        User,
        "SELECT id, nama, jabatan FROM rustdb"
    )
    .fetch_all(db.get_ref())
    .await;

    match result {
        Ok(rustdb) =>HttpResponse::Ok().json(rustdb),
        Err(e) =>{
            println!("error {:?}",e);
            HttpResponse::InternalServerError().body("gagal mengambil data")
        }
        
    }

}

//update data
#[put("/update/{id}")]
pub async fn updatedata(
    path: web::Path<i32>,
    user_data: web::Json<Update>,
    db: web::Data<MySqlPool>,

) -> impl Responder{
    let user_id = path.into_inner();

    let result = sqlx::query!(
        "UPDATE rustdb SET nama = ?, jabatan = ? WHERE id= ?",
        user_data.nama,
        user_data.jabatan,
        user_id
    )
    .execute(db.get_ref())
    .await;

    match result {
        
        Ok(_)=> HttpResponse::Ok().body("data berhasil di update"),
        Err(e) => HttpResponse::InternalServerError().body(format!("gagall update {}", e))
    }

}



//hapus data
#[delete("/delete/{id}")]
pub async fn delete(
    path: web::Path<i32>,
    db: web::Data<MySqlPool>,

) -> impl Responder{
    let user_id = path.into_inner();

    let result = sqlx::query!(
        "DELETE FROM rustdb WHERE id= ?",
        user_id
    )
    .execute(db.get_ref())
    .await;

    match result{
        Ok(_) => HttpResponse::Ok().body("Data berhasil dihapus"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Gagal hapus data {}",e))
    }
}

#[get("/user/{id}")]
pub async fn search(
    path: web::Path<i32>,
    db: web::Data<MySqlPool>,

) -> impl Responder{
    let user_id = path.into_inner();

    let result = sqlx::query_as!(
        Find,
        "SELECT nama, jabatan FROM rustdb WHERE id= ?",
        user_id
    )
    .fetch_one(db.get_ref())
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        
        Err(e) =>{
            println!("gagal karena : {:?}",e);
            HttpResponse::InternalServerError().body("data tidak ditemukan")
        }

    }
    
}

