use hardware::{Hardware, HardwareType};
use serde::{Deserialize, Serialize};

use crate::{id::IdGenerator, app_graph::{Node, NodeType, NbInput}};

use super::IsValid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temp {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,

    #[serde(skip)]
    pub hardware_internal_index: Option<usize>,
}

impl Temp {
    pub fn to_node(mut self, id_generator: &mut IdGenerator, hardware: &Hardware) -> Node {
        match self.hardware_id {
            Some(ref hardware_id) => {
                match hardware.get_internal_index(hardware_id, HardwareType::Temp) {
                    Some(index) => self.hardware_internal_index = Some(index),
                    None => {
                        eprintln!(
                            "hardware {} from config not found. Fall back to no id",
                            hardware_id
                        );
                        self.hardware_id = None
                    }
                }
            }
            None => {
                if self.hardware_internal_index.is_some() {
                    eprintln!(
                        "Temp to Node: Inconsistent internal index found. name: {}",
                        self.name
                    );
                    self.hardware_internal_index = None;
                }
            }
        }

        Node {
            id: id_generator.new_id(),
            node_type: NodeType::Temp(self),
            max_input: NbInput::Zero,
            inputs: Vec::new(),
            value: None,
        }
    }
}


impl IsValid for Temp {
    fn is_valid(&self) -> bool {
        self.hardware_id.is_some() && self.hardware_internal_index.is_some()
    }
}