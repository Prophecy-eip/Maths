pub mod response;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProphecyRequest {
    key: String,
    attacking_position: String,
    attacking_regiment: maths::dto::regiment::RegimentDto,
    defending_regiment: maths::dto::regiment::RegimentDto,
}

impl ProphecyRequest {
    pub fn new(
        key: String,
        attacking_position: String,
        attacking_regiment: maths::dto::regiment::RegimentDto,
        defending_regiment: maths::dto::regiment::RegimentDto,
    ) -> Self {
        Self {
            key,
            attacking_position,
            attacking_regiment,
            defending_regiment,
        }
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_attacking_position(&self) -> &String {
        &self.attacking_position
    }

    pub fn get_attacking_regiment(&self) -> &maths::dto::regiment::RegimentDto {
        &self.attacking_regiment
    }

    pub fn get_defending_regiment(&self) -> &maths::dto::regiment::RegimentDto {
        &self.defending_regiment
    }

    pub fn convert_attacking_position(&self) -> maths::fight::AttackPosition {
        match self.attacking_position.as_str() {
            "front" => maths::fight::AttackPosition::FRONT,
            "flank" => maths::fight::AttackPosition::FLANK,
            "back" => maths::fight::AttackPosition::BACK,
            _ => maths::fight::AttackPosition::FRONT,
        }
    }

    pub fn convert_regiment(&self, attacking: bool) -> maths::regiment::Regiment {
        match attacking {
            true => self.attacking_regiment.hydrate(),
            false => self.defending_regiment.hydrate(),
        }
    }
}
