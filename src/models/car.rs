#[derive(Queryable, AsChangeset, Identifiable, Serialize, Deserialize, ToSchema)]
#[allow(non_snake_case)]
#[diesel(table_name = cars)]
pub struct Car {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub year: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime
}