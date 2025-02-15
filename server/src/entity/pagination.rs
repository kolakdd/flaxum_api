use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct Pagination {
    #[validate(range(min = 1, max = 50))]
    pub limit: i64,
    #[validate(range(min = 0))]
    pub offset: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: 20,
            offset: 0,
        }
    }
}
