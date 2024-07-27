use std::fmt::{self, Display, Formatter};
use rocket::{get, serde::Serialize};
use rocket_dyn_templates::{context, Template};
use diesel::prelude::*;
// use diesel::sql_types::Date;
use chrono::{Local, NaiveDate};
use crate::{establish_connection, schema::questions::dsl::*};

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Question {
    pub id: i32,
    pub question_text: String,
    pub published: bool,
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.question_text)
    }
}

#[get("/polls")]
pub fn list() -> Template {
    let connection = &mut establish_connection();
    let results = questions
        .select(Question::as_select())
        .limit(5)
        .load(connection)
        .expect("Error loading questions");

    Template::render("polls/list", context! {
        results
    })
}

#[get("/polls/<poll_id>")]
pub fn detail(poll_id: i32) -> Template {
    let connection = &mut establish_connection();
    let result = questions
        .select(Question::as_select())
        .filter(id.eq(poll_id))
        .first(connection)
        .expect("Error loading questions");

    Template::render("polls/detail", context! {
        result
    })
}
