use serde_json::Error;
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
pub struct MesosState {
  pub id: String 
}

pub fn parse(json: String) -> Result<MesosState, Error> {
    let state: MesosState = from_str(&json)?;
    Ok(state)
}
