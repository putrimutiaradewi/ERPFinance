
use std::sync::Arc;

use sqlx::{Pool,Postgres};
use be_erp::{ update_account, login_account, tambah_akun, table_list, table_asal, table_satuan, table_jenis, table_kategori, tabel_pengeluaran, read_pemasukkan, update_data, delete_data};
use tide::{Server};

pub fn path(app: &mut Server<Pool<Postgres>>){
app.at("cek").get(table_list);
// app.at("add").post(add_table);
// app.at("update").put(update_table);
// app.at("delete").delete(delete_table);
app.at("Register").post(tambah_akun);
app.at("Lupa").put(update_account);
app.at("Login").post (login_account);
app.at("Login").post (login_account);
app.at("Asal").get(table_asal);
app.at("Satuan").get(table_satuan);
app.at("Jenis").get(table_jenis);
app.at("Kategori").get(table_kategori);
app.at("Tambah").post (tabel_pengeluaran);
app.at("Read").get(read_pemasukkan);
app.at("Update").put(update_data);
app.at("Delete").delete(delete_data);








}
