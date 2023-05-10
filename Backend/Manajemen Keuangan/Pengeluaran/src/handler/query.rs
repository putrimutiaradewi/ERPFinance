use std::vec;
use serde::Deserialize;
use sqlx::PgPool;
use tide::{Request, Response};
use tide::{Body, http};
use serde_json::{json, Error};


use crate::ws_response;


// #[derive(serde::Serialize, Debug ,Deserialize)]
// struct Get {
//     nama : Option<String>,
// }

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Test {
    nama : Option<String>,
    buku_id : Option<i32>,
}


#[derive(serde::Serialize, Debug ,Deserialize)]
struct Buku {
    buku_id : Option<i32>,
    nama_buku : Option<String>,
}

#[derive(serde::Serialize, Debug ,Deserialize)]
struct DelParam {
    buku_id : Option<i32>
}

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Account {
    username : String,
    password : String,
    email : String,

}   
#[derive(serde::Serialize, Debug ,Deserialize)]
struct LoginResult {
    status : String,
    info : String,
}

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Ul {
    username : String,
    password : String,

}   

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Register {
    username : String,
    alamat : String,
    telephone : String,
    password : String,

}   
#[derive(serde::Serialize, Debug ,Deserialize)]
struct hewan {
    name : String,
}   

#[derive(serde::Serialize, Debug ,Deserialize)]
struct pengeluaran {
   
	jumlah : String,
	tanggal : String,
	id_jenis_transaksi : i32,
	quantity : String,
	id_satuan : i32,
	id_asal : i32,
	id_kategori: i32,
	kurs : String,
	sisa_saldo : String,

}   

#[derive(serde::Serialize, Debug ,Deserialize)]
struct ReadData {
    id: i32, 
	jumlah : Option<String>,
	tanggal : Option<String>,
	id_jenis_transaksi : Option<i32>,
	quantity : Option<String>,
	id_satuan : Option<i32>,
	id_asal : Option<i32>,
	id_kategori: Option<i32>,
	kurs : Option<String>,
	sisa_saldo : Option<String>,

}   

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Update {
    id:i32,
	jumlah : Option<String>,
	tanggal : Option<String>,
	id_jenis_transaksi : Option<i32>,
	quantity : Option<String>,
	id_satuan : Option<i32>,
	id_asal : Option<i32>,
	id_kategori: Option<i32>,
	kurs : Option<String>,
	sisa_saldo : Option<String>,

}   

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Delete {
    id:i32,
	jumlah : Option<String>,
	tanggal : Option<String>,
	id_jenis_transaksi : Option<i32>,
	quantity : Option<String>,
	id_satuan : Option<i32>,
	id_asal : Option<i32>,
	id_kategori: Option<i32>,
	kurs : Option<String>,
	sisa_saldo : Option<String>,

}   

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Get{
    name : Option<String>,
}

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Asal {
    name : Option<String>,
    id_asal: i32,
}
#[derive(serde::Serialize, Debug ,Deserialize)]
struct Jenis {
    name : Option<String>,
    id_jenis: i32,
}
#[derive(serde::Serialize, Debug ,Deserialize)]
struct Kategori {
    name : Option<String>,
    id_kategori: i32,
}

#[derive(serde::Serialize, Debug ,Deserialize)]
struct Satuan{
    name : Option<String>,
    id_satuan: i32,
}

//create
pub async fn tabel_pengeluaran (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : pengeluaran = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("INSERT INTO manajemen_keuangan (jumlah , tanggal, id_jenis_transaksi, quantity, id_satuan, id_asal, id_kategori, kurs, sisa_saldo) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9);")   
     .bind(param.jumlah)
     .bind(param.tanggal)
     .bind(param.id_jenis_transaksi)
     .bind(param.quantity)
     .bind(param.id_satuan)
     .bind(param.id_asal)
     .bind(param.id_kategori)
     .bind(param.kurs)
     .bind(param.sisa_saldo)
     
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Pengeluaran")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal Pengeluaran")
        }

     }

}

//read
pub async fn read_pemasukkan(req : Request<PgPool>) -> tide::Result<Response>{
    // let param : Get = req.query()?;
    let pool = req.state();
    let nama :Vec<ReadData> = sqlx::query_as!(
ReadData, "SELECT id, jumlah , tanggal, id_jenis_transaksi, quantity, id_satuan, id_asal, id_kategori, kurs, sisa_saldo from manajemen_keuangan;")
.fetch_all(pool).await?;
println!("table : {:#?} ", nama);

let response = Response::builder(200)
            .body(Body::from_json(&nama)? ).build();
        Ok(response)
}

//update
pub async fn update_data (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Update = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("UPDATE manajemen_keuangan SET jumlah=$2 , tanggal=$3, id_jenis_transaksi=$4, quantity=$5, id_satuan=$6, id_asal=$7, id_kategori=$8, kurs=$9, sisa_saldo=$10 where id=$1")
     .bind(param.id)
     .bind(param.jumlah)
     .bind(param.tanggal)
     .bind(param.id_jenis_transaksi)
     .bind(param.quantity)
     .bind(param.id_satuan)
     .bind(param.id_asal)
     .bind(param.id_kategori)
     .bind(param.kurs)
     .bind(param.sisa_saldo)
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil Update")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal Update ")
        }

     }
}

//delete
pub async fn delete_data ( req : Request<PgPool>) -> tide::Result<Response> {
    match req.query(){
        Ok(x) => {
            let param : Delete =x;
            let pool = req.state();
             
             match
             sqlx::query("DELETE FROM manajemen_keuangan WHERE id=$1")
             .bind(param.id)
             .execute(pool).await
             {
                Ok(_x) => {ws_response("OK", "Berhasil Delete")},
                Err(e) => {
                    println!("error delete : {:?}",e);
                    ws_response("Error", "Gagal Delete ")
                }
        
             }
        }
        Err(e) => {
            println!("Error : {:?}",e);
            let msg = format!("{:?}",e);
            ws_response("Error", msg.as_str())

        }
    }

}
   








//asal
pub async fn table_asal(req : Request<PgPool>) -> tide::Result<Response>{
    // let param : Get = req.query()?;
    let pool = req.state();
    let nama :Vec<Asal> = sqlx::query_as!(
Asal,"SELECT name, id_asal from asal;")
.fetch_all(pool).await?;
println!("table : {:#?} ", nama);

let response = Response::builder(200)
            .body(Body::from_json(&nama)? ).build();
        Ok(response)
}



//jenis
pub async fn table_jenis(req : Request<PgPool>) -> tide::Result<Response>{
    // let param : Get = req.query()?;
    let pool = req.state();
    let nama :Vec<Jenis> = sqlx::query_as!(
Jenis,"SELECT name, id_jenis from jenis_transaksi;")
.fetch_all(pool).await?;
println!("table : {:#?} ", nama);

let response = Response::builder(200)
            .body(Body::from_json(&nama)? ).build();
        Ok(response)
}

//kategori
pub async fn table_kategori(req : Request<PgPool>) -> tide::Result<Response>{
    // let param : Get = req.query()?;
    let pool = req.state();
    let nama :Vec<Kategori> = sqlx::query_as!(
Kategori,"SELECT name, id_kategori from kategori;")
.fetch_all(pool).await?;
println!("table : {:#?} ", nama);

let response = Response::builder(200)
            .body(Body::from_json(&nama)? ).build();
        Ok(response)
}

//satuan
pub async fn table_satuan(req : Request<PgPool>) -> tide::Result<Response>{
    // let param : Get = req.query()?;
    let pool = req.state();
    let nama :Vec<Satuan> = sqlx::query_as!(
Satuan,"SELECT name, id_satuan from satuan;")
.fetch_all(pool).await?;
println!("table : {:#?} ", nama);

let response = Response::builder(200)
            .body(Body::from_json(&nama)? ).build();
        Ok(response)
}



























pub async fn table_list(req: Request<PgPool>) -> tide::Result<Response> {

        let animals = vec![
            hewan { name: "Ayam".to_string() },
            hewan { name: "Bebek".to_string() },
            hewan { name: "Kucing".to_string() },
        ];
        println!("{:?}", animals);
        let response_body = serde_json::to_value(animals)?;
        let mut res = Response::new(200);
        res.set_body(response_body);
        Ok(res)
    } 

    
    

pub async fn add_table(mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Buku = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("INSERT INTO table2 (buku_id,nama_buku) VALUES($1,$2);")
     .bind(param.buku_id)
     .bind(param.nama_buku)
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil insert ke table2")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal insert ke table2")
        }

     }

}

pub async fn update_table (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Buku = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("UPDATE table2 SET nama_buku=$2 WHERE buku_id=$1")
     .bind(param.buku_id)
     .bind(param.nama_buku)
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil Update ke table2")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal Update ke table2")
        }

     }

}

pub async fn delete_table ( req : Request<PgPool>) -> tide::Result<Response> {
    match req.query(){
        Ok(x) => {
            let param : DelParam =x;
            let pool = req.state();
             
             match
             sqlx::query("DELETE FROM table2 WHERE buku_id=$1")
             .bind(param.buku_id)
             .execute(pool).await
             {
                Ok(_x) => {ws_response("OK", "Berhasil Delete ke table2")},
                Err(e) => {
                    println!("error delete : {:?}",e);
                    ws_response("Error", "Gagal Delete ke table2")
                }
        
             }
        }
        Err(e) => {
            println!("Error : {:?}",e);
            let msg = format!("{:?}",e);
            ws_response("Error", msg.as_str())

        }
    }


   



}
pub async fn add_account (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Account = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("INSERT INTO login (username, email, password) VALUES ($1,$2,sha256($3));")
     .bind(param.username)
     .bind(param.email)
     .bind(param.password.as_bytes())
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil login")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal login")
        }

     }

}

pub async fn user_account (mut req : Request<PgPool>) -> tide::Result<Response> {
    let form: Register = req.body_json().await?;
    let pool = req.state();
    let mut resp = Response::new(http::StatusCode::Ok);
    if let Ok (record) = sqlx::query!(
        "SELECT username FROM login WHERE username = $1 and password=sha256($2::text::bytea)",
        form.username, form.password
    ).fetch_one(pool).await{

       let ret = LoginResult {
        status:"Ok".to_string(),
        info:"Login Berhasil".to_string(),
       };

       resp.set_status(200);
       resp.set_body(Body::from_json(&ret)?);
    }else {
        
        let ret = serde_json::json!({
        "status": "Error",
        "info": "username/password is invalid"
    });
    resp.set_status(http::StatusCode::Ok);
    resp.set_body(Body::from_json(&ret)?);
    };

    Ok(resp)
}
    

pub async fn update_password (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Account = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("UPDATE login SET password=sha256($3) WHERE username=$1")
     .bind(param.username)
     .bind(param.email)
     .bind(param.password.as_bytes())
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil Update")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal Update ")
        }

     }


}

pub async fn tambah_akun (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Register = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("INSERT INTO users (username, alamat, password, telephone) VALUES ($1,$2,sha256($3),$4);")
     .bind(param.username)
     .bind(param.alamat)
     .bind(param.password.as_bytes())
     .bind(param.telephone)
     
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil register")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal register")
        }

     }

}

pub async fn update_account (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Ul = req.body_json().await?;
    let pool = req.state();
     
     match
     sqlx::query("UPDATE users SET password=sha256($2) WHERE username=$1")
     .bind(param.username)
     .bind(param.password.as_bytes())
     .execute(pool).await
     {
        Ok(_x) => {ws_response("OK", "Berhasil Update")},
        Err(e) => {
            println!("error insert : {:?}",e);
            ws_response("Error", "Gagal Update ")
        }

     }
     
    }
    pub async fn login_account (mut req : Request<PgPool>) -> tide::Result<Response> {
    let param : Ul = req.body_json().await?;
    let pool = req.state();
    let mut resp = Response::new(http::StatusCode::Ok);

    if let Ok(record) = sqlx::query!(
        "SELECT username FROM users WHERE username = $1  and password = sha256($2::text::bytea)",
        param.username, param.password,
    ).fetch_one(pool).await{

        let ret = LoginResult{
            status: "Ok".to_string(),
            info: "Login berhasil".to_string(),

        };
        resp.set_status(200);
        resp.set_body(Body::from_json(&ret)?);
    } else {
        let ret = serde_json::json!(
            {
                "status": "Error",
                "info": "Username/password Invalid"
            }
        );
        resp.set_status(http::StatusCode::Ok);
        resp.set_body(Body::from_json(&ret)?);
    }
    Ok(resp)
}



