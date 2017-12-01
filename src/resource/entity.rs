use std::io::Error;

use resource::ResourceBuilder;

use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entity {
    pub id: String,
    pub player: bool,
    pub display: char,
    pub size: usize,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Entity) -> bool {
        self.id == other.id
    }
}

impl ResourceBuilder for Entity {
    fn owned_id(&self) -> String {
        self.id.to_owned()
    }

    fn new(data: &str) -> Result<Entity, Error> {
        let entity: Entity = serde_json::from_str(data)?;

        Ok(entity)
    }
}
