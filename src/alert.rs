use sea_orm::entity::prelude::*;


#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name="alert")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub alert_message: String,
    pub created_on: String,
}

#[derive(Copy, Debug, Clone ,EnumIter, DeriveRelation)]
pub enum Relation {}



impl ActiveModelBehavior for ActiveModel {}