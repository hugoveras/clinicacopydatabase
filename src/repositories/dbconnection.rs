use std::io::ErrorKind;
use sqlx::postgres;
use sqlx::postgres::PgPoolOptions;

pub async fn getdbconnection(dbhost:String, dbport:String, dbuser:String, dbpass:String, dbname: String) -> Result<sqlx::postgres::PgPool, std::io::Error>{
    let options = postgres::PgConnectOptions::new()
        .host(dbhost.as_str())
        .port(dbport.parse().unwrap())
        .username(dbuser.as_str())
        .database(dbname.as_str())
        .password(dbpass.as_str());

    match PgPoolOptions::new().connect_with(options).await {
        Ok(dbconnection)=>{
            Ok(dbconnection)
        },

        Err(why)=>{
            println!("------>Error {:?}",why);
            Err(std::io::Error::new(ErrorKind::Other, why.to_string()))
        }
    }
}