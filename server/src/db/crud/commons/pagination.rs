use crate::common::Pagination;
use sqlx::Postgres;
use sqlx::QueryBuilder;

/// Append pagination to QueryBuilder
#[inline]
pub fn pagination_query_builder<'a>(
    mut query: QueryBuilder<'a, Postgres>,
    pagination: &'a Pagination,
) -> QueryBuilder<'a, Postgres> {
    query.push(" limit ");
    query.push_bind(pagination.limit);

    query.push(" offset ");
    query.push_bind(pagination.offset);

    query
}
