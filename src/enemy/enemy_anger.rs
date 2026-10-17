#[derive(Debug)]
pub enum EnemyType {
    Neutral,
    Impatient
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
        match self {
            EnemyAngerLevel::Neutral => if neutral_to_alert { EnemyAngerLevel::Alert } else { EnemyAngerLevel::Neutral },
            EnemyAngerLevel::Alert => if alert_to_angry { EnemyAngerLevel::Angry } else if alert_to_neutral { EnemyAngerLevel::Neutral } else { EnemyAngerLevel::Alert },
            EnemyAngerLevel::Angry => EnemyAngerLevel::Angry
        }
    }
}