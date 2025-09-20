use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize, ToSchema)]
pub struct OrganizationId(pub String);