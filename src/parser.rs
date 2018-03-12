use serde_json::Error;
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
pub struct FrameworkDto {
  pub name: String
}

#[derive(Serialize, Deserialize)]
pub struct MesosStateDto {
  pub id: String,
  pub frameworks: Vec<FrameworkDto>,
}

pub fn parse(json: String) -> Result<MesosStateDto, Error> {
    let state: MesosStateDto = from_str(&json)?;
    Ok(state)
}
