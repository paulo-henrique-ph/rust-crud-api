#[derive(Debug, Queryable, AsChangeset)]
#[table_name = "cars"]
pub struct Car<'a> {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub year: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
