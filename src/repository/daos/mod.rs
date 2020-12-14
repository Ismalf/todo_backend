use crate::models::task::{Task, NewTask};
use crate::database_engine_pool::MysqlPool;
use diesel::{MysqlConnection, QueryResult, RunQueryDsl};
use std::collections::BTreeMap;
use serde_json::Value;

use diesel::prelude::*;

use crate::schema::{
    tasks,
    tasks::dsl::{is_done as task_completed, tasks as all_tasks, name as task_name},
};

/// Module to define different Data Access Objects

// Define an interface for all daos to implement
pub trait DAO<S, T, W> {
    fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<T>>;
    fn find_item_by_id(id: W, conn: &MysqlConnection) -> QueryResult<T>;
    fn save_item(item: S, conn: &MysqlConnection) -> QueryResult<usize>;
    fn update_item(item: T, conn: &MysqlConnection) -> QueryResult<usize>;
    fn delete_item(id: W, conn: &MysqlConnection) -> QueryResult<usize>;
}

pub struct taskDAO;

impl DAO<NewTask, Task, i32> for taskDAO {
    fn get_all(conn: &MysqlConnection) -> QueryResult<Vec<Task>> { all_tasks.order(tasks::id.asc()).load::<Task>(conn) }

    fn find_item_by_id(id: i32, conn: &MysqlConnection) -> QueryResult<Task> { all_tasks.find(id).get_result::<Task>(conn) }

    fn save_item(item: NewTask, conn: &MysqlConnection) -> QueryResult<usize> { diesel::insert_into(tasks::table).values(&item).execute(conn) }

    fn update_item(item: Task, conn: &MysqlConnection) -> QueryResult<usize> {
        let updated_task = diesel::update(&item);
        updated_task
            .set((
                task_completed.eq(&item.is_done),
                task_name.eq(&item.name)
            ))
            .execute(conn)
    }

    fn delete_item(id: i32, conn: &MysqlConnection) -> QueryResult<usize> {
        diesel::delete(all_tasks.find(id)).execute(conn)
    }
}