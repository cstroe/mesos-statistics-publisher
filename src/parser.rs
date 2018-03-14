use serde_json::Error;
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
pub struct SlaveDto {
    pub id: String,
    pub hostname: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskDto {
    pub id: String,
    pub slave_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct FrameworkDto {
    pub name: String,
    pub tasks: Vec<TaskDto>,
}

#[derive(Serialize, Deserialize)]
pub struct MesosStateDto {
    pub id: String,
    pub slaves: Vec<SlaveDto>,
    pub frameworks: Vec<FrameworkDto>,
}

pub fn parse(json: String) -> Result<MesosStateDto, Error> {
    Ok(from_str(&json)?)
}
