mod models;
mod repositories;

use std::{env, fs::File, io::{BufReader, ErrorKind}, io::Read, path::Path};
use csv;
use csv::Error;

use sqlx::{Postgres, Pool};

use models::medico::MedicoDataCSV;

async fn process_csv(path:String, tenantid:String, pool: Pool<Postgres>){
    println!("Leyendo CSV......");

    let csv_path = Path::new(&path).join("Doctors.csv");

    match csv::Reader::from_path(csv_path) {
        Ok(mut reader)=>{
            for record in reader.deserialize() {
                let resp_record: Result<MedicoDataCSV, Error> = record;
                match resp_record {
                    Ok(record)=>{
                        println!("Insertando record...");
                        let res = insert_medico(record.clone(), tenantid.clone(), pool.clone(), path.clone()).await;
                        println!("Resultado: {:?}", res);
                    },
                    Err(why)=>{
                        println!("Error found: {:?}", why);
                    }
                };

                println!("================================================================================================");
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

    if args.len() != 8 {
        return Err(std::io::Error::new(ErrorKind::Other, String::from("Numero incorrecto de argumentos")));
    }

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
    
    println!("Conectando a la base de datos...");
    let resp_dbconnection=repositories::dbconnection::getdbconnection(
        ipdataserver.clone(), 
        portdataserver.clone(), 
        username.clone(), 
        password.clone(), 
        databasename.clone()
    ).await;

    match resp_dbconnection {
        Ok(pool) => {
            println!("Conectado a la base de datos");
            process_csv(filepath.to_string(), tenantid.to_string(), pool.clone()).await;

            Ok(())
        },
        Err(why) => {
            println!("{:?}", why);
            Ok(())
        }
    }
}

pub async fn insert_medico(medico_data_csv:MedicoDataCSV, tenant_id: String, db_connection: Pool<Postgres>, path: String) -> String {
    let mut photo:Vec<u8> = vec![];
    let photo_filename = format!("{} {}.jpg", medico_data_csv.nombre.trim(), medico_data_csv.apellidos.trim());

    let photo_path = Path::new(&path).join("Fotos").join(photo_filename.clone());
    println!("Foto: {:?}",photo_path);

    match File::open(photo_path) {
        Ok(f) => {
            let mut reader = BufReader::new(f);
            match reader.read_to_end(&mut photo) {
                Ok(_) => {
                }, 
                Err(why) => {
                    println!("Error leyendo el archivo {}: {:?}", photo_filename, why);
                }
            };
        }, Err(why) => {
            println!("Error abriendo el archivo {}", photo_filename);
            println!("{:?}", why);
        }
    };

    let sqlquery = format!("INSERT INTO clinicaschema.medicos(
        titulo,
        nombres,
        apellidos,
        especialidad,
        consultorio,
        horario,
        telefono,
        email,
        extension,
        paginaweb,
        foto,
        movil,
        empresa,
        tenantid,
        status,
        fax, foto_url, twitter, instagram, whatsapp) VALUES($1, $2, $3, $4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20)
        RETURNING id"
    );
    let rows_result = sqlx::query(sqlquery.as_str())
        .bind(&medico_data_csv.title.trim().replace("NULL", ""))
        .bind(&medico_data_csv.nombre.trim().replace("NULL", ""))
        .bind(&medico_data_csv.apellidos.trim().replace("NULL", ""))
        .bind(&medico_data_csv.especialidad.trim().replace("NULL", ""))
        .bind(&medico_data_csv.consultorio.trim().replace("NULL", ""))
        .bind(&medico_data_csv.horario.trim().replace("NULL", ""))
        .bind(&medico_data_csv.telefono.trim().replace("NULL", ""))
        .bind(&medico_data_csv.email.trim().replace("NULL", ""))
        .bind(&medico_data_csv.extension.trim().replace("NULL", ""))
        .bind("")
        .bind(&photo)
        .bind(&medico_data_csv.movil.trim().replace("NULL", ""))
        .bind("")
        .bind(tenant_id)
        .bind("Activo")
        .bind("")
        .bind("")
        .bind("")
        .bind("")
        .bind(&medico_data_csv.movil.trim().replace("NULL", ""))
        .fetch_one(&db_connection)
        .await;

    match rows_result {
        Ok(_) => {
            String::from("Insertado")
        },
        Err(why) => {
            why.to_string()
        }
    }
}
