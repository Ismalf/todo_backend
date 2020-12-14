pub mod daos;

use crate::daos;

#[cfg(test)]
mod tests {
    use crate::database_engine_pool::get_conn;
    use diesel::{Connection, RunQueryDsl};
    use diesel::result::Error;

    #[test]
    fn insert() {
        let conn = get_conn(&pool).as_deref().unwrap();
        conn.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(users)
                .values(name.eq("Ruby"))
                .execute(&conn)?;

            let all_names = users.select(name).load::<String>(&conn)?;
            assert_eq!(vec!["Sean", "Tess", "Ruby"], all_names);

            Ok(())
        });
    }

    #[test]
    fn update() {}

    #[test]
    fn delete() {}
}