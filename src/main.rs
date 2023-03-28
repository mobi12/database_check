mod tableinfo;

use mysql::*;
use mysql::prelude::*;

use sqlite;

use std::env::args;

use tableinfo::TableInfo;

fn main() {
    let mut arguments: Vec<String> = args().collect();

    let url = arguments.remove(1);

    let pool = match Pool::new(url.as_str()) {
        Ok(pool) => pool,
        Err(e) => panic!("🤐 {}", e)
    };

    let mut connect = match pool.get_conn() {
        Ok(con) => con,
        Err(e) => panic!("🤐 {}", e)
    };

    let query_str = String::from("SELECT value FROM sys_dic_catalog");
    
    
    let sqlite_conn = sqlite::open("./standard.db")
        .expect("don't have db file");

    let mut server_db_catalogs: Vec<String> = connect.query(&query_str)
        .expect("Don't have any value");

    let mut local_db_catalogs: Vec<String> = vec![];

    sqlite_conn.iterate(query_str, |pairs| {
        for &(temp, catalog) in pairs.iter() {
            local_db_catalogs.push((catalog.unwrap()).to_string());
        }
        true
    }).unwrap();

    server_db_catalogs.sort();
    local_db_catalogs.sort();
        
}

struct TableInfo {
    pub name: String,
    pub columns: Vec<String>
}

fn get_table_name_infos() -> Vec<String> {
    let connection = match sqlite::open("./table_info.db") {
        Ok(con) => con,
        Err(e) => panic!("{}", e.to_string())
    };

    let table_infos = vec![];
    let query = "
        SELECT table_name FROM tables;
    ";

    connection.iterate(table_infos, |pairs| {
        for &(name) in pairs.iter() {
            tabl
        }
    })
}