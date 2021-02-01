use diesel::prelude::*;

fn main() {
    use hello::schema::{posts, users};

    let conn = hello::establish_connection();

    let subquery = users::table
        .select(users::id)
        .filter(users::name.eq("John"));
    println!(
        "{}",
        diesel::debug_query::<diesel::sqlite::Sqlite, _>(&subquery).to_string()
    );
    let query = diesel::update(posts::table.filter(posts::user_id.eq_any(subquery)))
        .set(posts::title.eq("Z"));
    println!(
        "{}",
        diesel::debug_query::<diesel::sqlite::Sqlite, _>(&query).to_string()
    );
    query.execute(&conn).unwrap();
}
