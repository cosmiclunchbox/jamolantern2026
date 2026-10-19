#[derive(Debug)]
pub enum EnemyType {
    Idle,
    Impatient,
    Enraged
}

impl EnemyType {
    pub fn from_numeric_id(id: u32) -> EnemyType {
        match id {
            0 => EnemyType::Idle,
            1 => EnemyType::Impatient,
            2 => EnemyType::Enraged,
            _ => panic!("Unrecognized enemy type id.")
        }
    }
}


#[derive(Debug)]
pub enum EnemyAngerLevel {
    Neutral,
    Alert,
    Angry
}

impl EnemyAngerLevel {
    pub fn transition(
        &self,
        neutral_to_alert: bool,
        alert_to_neutral: bool,
        alert_to_angry: bool,
    ) -> EnemyAngerLevel {
        let new_anger_level = match self {
            EnemyAngerLevel::Neutral => if neutral_to_alert { EnemyAngerLevel::Alert } else { EnemyAngerLevel::Neutral },
            EnemyAngerLevel::Alert => if alert_to_angry { EnemyAngerLevel::Angry } else if alert_to_neutral { EnemyAngerLevel::Neutral } else { EnemyAngerLevel::Alert },
            EnemyAngerLevel::Angry => EnemyAngerLevel::Angry
        };
        //println!("{:?}", new_anger_level);
        new_anger_level
    }
}