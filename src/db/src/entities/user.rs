use super::sea_orm_active_enums::UserType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
	#[sea_orm(primary_key, auto_increment = false)]
	pub id: Uuid,
	pub r#type: UserType,
	#[sea_orm(column_type = "Text")]
	pub password: crate::secret::PasswordHash,
	pub last_login_at: TimeDateTimeWithTimeZone,
	pub to_change_password: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::session::Entity")]
	Session,
}

impl Related<super::session::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Session.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
