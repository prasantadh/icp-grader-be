use diesel::prelude::*;

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::years)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Year {
    id: i32,
}

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::levels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Level {
    id: String,
    year_id: i32,
}

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::sections)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Section {
    id: String,
    level_id: String,
}
