use std::fmt::{self, Display, Formatter};
use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use diesel::prelude::*;
use diesel::sql_types::Date;
use chrono::{Local, NaiveDate};
use crate::{establish_connection, schema::{choices::question_id, questions::dsl::*}};
use crate::schema::questions::dsl::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Question {
    pub id: i32,
    pub question_text: String,
    pub pub_date: NaiveDate,
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.question_text)
    }
}

#[get("/polls")]
pub fn index() -> status::Custom<String> {
    let connection = &mut establish_connection();
    let results = questions
        .select(Question::as_select())
        .limit(5)
        .load(connection)
        .expect("Error loading questions");

    let mut response = String::from("Questions:\n");
    for question in results {
        response.push_str(&format!("{}\n", question));
    }

    status::Custom(
        Status::Ok,
        response
   )

}

#[get("/polls/<poll_id>")]
pub fn detail(poll_id: i32) -> status::Custom<String> {
    let connection = &mut establish_connection();
    let result = questions
        .select(question_text)
        .filter(id.eq(poll_id))
        .limit(1)
        .load::<String>(connection)
        .expect("Error loading questions");

    let mut response = String::from("Question:\n");
    response.push_str(&format!("{:?}\n", result));

    status::Custom(
        Status::Ok,
        response
   )
}
