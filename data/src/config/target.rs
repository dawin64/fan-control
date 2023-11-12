use crate::{
    id::IdGenerator,
    node::{sanitize_inputs, Inputs, IsValid, Node, NodeType, NodeTypeLight, Nodes, ToNode},
    update::UpdateError,
};
use hardware::{Hardware, Value};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Target {
    pub name: String,
    #[serde(rename = "idleTemp", alias = "idle_temp")]
    pub idle_temp: u8,
    #[serde(rename = "idleSpeed", alias = "idle_speed")]
    pub idle_speed: u8,
    #[serde(rename = "loadTemp", alias = "load_temp")]
    pub load_temp: u8,
    #[serde(rename = "loadSpeed", alias = "load_speed")]
    pub load_speed: u8,
    pub input: Option<String>,

    #[serde(skip)]
    pub idle_has_been_reatch: bool,
}

#[derive(Debug, Clone)]
pub struct TargetCache {
    pub idle_temp: String,
    pub idle_speed: String,
    pub load_temp: String,
    pub load_speed: String,
}

impl Target {
    pub fn cache(&self) -> TargetCache {
        TargetCache {
            idle_temp: self.idle_temp.to_string(),
            idle_speed: self.idle_speed.to_string(),
            load_temp: self.load_temp.to_string(),
            load_speed: self.load_speed.to_string(),
        }
    }

    pub fn update(&mut self, value: Value) -> Result<Value, UpdateError> {
        if self.idle_has_been_reatch {
            if value < self.load_temp.into() {
                return Ok(self.idle_speed.into());
            }

            self.idle_has_been_reatch = false;
            return Ok(self.load_speed.into());
        }

        if value > self.idle_temp.into() {
            return Ok(self.load_speed.into());
        }

        self.idle_has_been_reatch = true;
        Ok(self.idle_speed.into())
    }
}

impl IsValid for Target {
    fn is_valid(&self) -> bool {
        self.input.is_some()
    }
}

impl Inputs for Target {
    fn clear_inputs(&mut self) {
        self.input.take();
    }

    fn get_inputs(&self) -> Vec<&String> {
        match &self.input {
            Some(input) => vec![input],
            None => Vec::new(),
        }
    }
}

impl ToNode for Target {
    fn to_node(
        mut self,
        id_generator: &mut IdGenerator,
        nodes: &Nodes,
        _hardware: &Hardware,
    ) -> Node {
        let inputs = sanitize_inputs(&mut self, nodes, NodeTypeLight::Target);
        let cache = self.cache();
        Node::new(id_generator, NodeType::Target(self, cache), inputs)
    }
}

#[cfg(test)]
mod test {

    use super::Target;

    #[test]
    fn test_update() {
        let _ = env_logger::try_init();

        let mut target = Target {
            name: "linear".to_string(),
            input: Some("temp1".into()),
            idle_temp: 40,
            idle_speed: 10,
            load_temp: 70,
            load_speed: 100,
            idle_has_been_reatch: false,
        };

        assert!(target.update(55).unwrap() == 100);
        assert!(target.update(30).unwrap() == 10);
        assert!(target.update(55).unwrap() == 10);
        assert!(target.update(70).unwrap() == 100);
    }
}
