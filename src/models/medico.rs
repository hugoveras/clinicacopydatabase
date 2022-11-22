#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug,Default,Clone,sqlx::FromRow )]
pub struct MedicoDataCSV {
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "NOMBRE")]
    pub nombre: String,
    #[serde(rename = "APELLIDOS")]
    pub apellidos: String,
    #[serde(rename = "TITLE")]
    pub title: String,
    #[serde(rename = "HORARIO")]
    pub horario: String,
    #[serde(rename = "CONSULTORIO")]
    pub consultorio: String,
    #[serde(rename = "EXTENSION")]
    pub extension: String,
    #[serde(rename = "EMAIL")]
    pub email: String,
    #[serde(rename = "TELEFONO")]
    pub telefono: String,
    #[serde(rename = "MOVIL")]
    pub movil: String,
    #[serde(rename = "ESPECIALIDAD")]
    pub especialidad: String,
    //pub Fotos: String, campo duplicado en el CSV causa errores
}
