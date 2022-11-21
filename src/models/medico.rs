use serde::Deserialize;


#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug,Default,Clone,sqlx::FromRow )]
pub struct MedicoData {
    pub ID: i32,
    pub TITLE: String,
    pub NOMBRE: String,
    pub APELLIDOS: String,
    pub ESPECIALIDAD: String,
    pub CONSULTORIO: String,
    pub HORARIO: String,
    pub TELEFONO: String,
    pub MOVIL: String,
    pub EMAIL: String,
    pub EXTENSION: String,
    // pub paginaweb: String,
    // pub fax: Option<String>,
    // pub empresa: Option<String>,
    // pub status: Option<String>,
    // pub foto_url: Option<String>,
    // pub twitter: Option<String>,
    // pub instagram: Option<String>,
    // pub whatsapp: Option<String>
}