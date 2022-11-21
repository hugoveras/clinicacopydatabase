mod models;

use std::env;
use csv::{Reader};

use csv::Error;

async fn  getCSVfile(filename:String, tenantid:String){
    println!("start reading the file {:?}", filename);
    let mut resp_reader = csv::Reader::from_path(filename);
    match resp_reader {
        Ok(mut reader)=>{
            for record in reader.deserialize() {
                let resp_record: Result<crate::models::medico::MedicoData, Error > = record;
                match resp_record {
                    Ok(record)=>{

                        //llamar aqui la funcion que inserta a la base de datos
                        //llamar aqui la funcion que lee la foto del disco y la sube a la base de datos

                        println!(
                            "Id = {}, title ={} nombre= {}  apellidos= {}.",
                            record.ID,
                            record.TITLE,
                            record.NOMBRE,
                            record.APELLIDOS
                        );

                    },
                    Err(why)=>{
                        println!("Error found: {:?}", why);
                    }

                }
            }
        },
        Err(why)=>{
            println!("Error found: {:?}", why);
        }
    };
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error>  {
    let args: Vec<String> = env::args().collect();

    let filepath = &args[1];
    let ipdataserver = &args[2];
    let portdataserver = &args[3];
    let databasename = &args[4];
    let username = &args[5];
    let password = &args[6];
    let tenantid = &args[7];

    println!("Archivo de fotos ......................{}", filepath);
    println!("IP del servidor de base de datos.......{}", ipdataserver);
    println!("Puerto del servidor de base de datos...{}", portdataserver);
    println!("Nombre de la base de datos.............{}", databasename);
    println!("Nombrel usuario........................{}", username);
    println!("Password de la base de datos...........{}", password);
    println!("Tenant id ..............................{}", tenantid);
    //--------------------------------------------------------------------------------------------
    getCSVfile(filepath.to_string(), tenantid.to_string()).await;
    Ok(())

}