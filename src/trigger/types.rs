use std::error::Error;

pub enum TriggerType {
    Manual,
    Schedule,
    Action(TriggerActionType)
}

pub enum TriggerActionType {
    BtnClick,
}

pub trait Trigger {

    fn trigger(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}