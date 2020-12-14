use serde::{Serialize, Deserialize};
use diesel::{Insertable, Queryable, Identifiable};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use crate::schema::tasks;

///
/// Activity structure defining the corresponding attributes
/// Not mutable! Therefore only getters are available
///
#[derive(Debug, Insertable, Deserialize)]
#[table_name="tasks"]
pub struct NewTask {
    pub name: String,
    // the task itself
    pub is_done: String,
}

#[derive(Deserialize, Serialize, Queryable, Identifiable)]
pub struct Task {
    pub id: i32,
    // DB independent id type
    pub name: String,
    // the task itself
    pub is_done: String, // indicates if the task is marked as complete
}

