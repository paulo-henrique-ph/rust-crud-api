#[derive(Insertable)]
#[table_name = "cars"]
pub struct CarDto<'a> {
    pub brand: String,
    pub model: String,
    pub year: String,
}
