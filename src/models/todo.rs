use diesel::{PgConnection, QueryResult, RunQueryDsl};
use juniper::{GraphQLInputObject, GraphQLObject};

use crate::{schema::todos};

#[derive(GraphQLObject, Queryable, Clone)]
#[graphql(description = "Todo")]
pub struct Todo {
    pub id: i32,
    pub title: String,
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "todos"]
#[graphql(description = "New todo")]
pub struct NewTodo {
    title: String,
}

impl Todo {
    pub fn get_all_todo(connection: &PgConnection) -> QueryResult<Vec<Todo>> {
        use crate::schema::todos::dsl::*;

        todos.load(connection)
    }

    pub fn insert(connection: &PgConnection, data: NewTodo) -> QueryResult<usize> {
        diesel::insert_into(todos::table)
            .values(&data)
            .execute(connection)
    }
}
