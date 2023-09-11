//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "contracts")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub creation_code_hash: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub runtime_code_hash: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::code::Entity",
        from = "Column::CreationCodeHash",
        to = "super::code::Column::CodeHash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Code2,
    #[sea_orm(
        belongs_to = "super::code::Entity",
        from = "Column::RuntimeCodeHash",
        to = "super::code::Column::CodeHash",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Code1,
    #[sea_orm(has_many = "super::contract_deployments::Entity")]
    ContractDeployments,
    #[sea_orm(has_many = "super::verified_contracts::Entity")]
    VerifiedContracts,
}

impl Related<super::contract_deployments::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContractDeployments.def()
    }
}

impl Related<super::verified_contracts::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::VerifiedContracts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
