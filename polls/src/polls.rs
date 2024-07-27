use std::fmt::{self, Display, Formatter};
use rocket::get;
use rocket::http::Status;
use rocket::response::status;
use diesel::prelude::*;
use chrono::{Local, NaiveDateTime};
use crate::{establish_connection, schema::questions::dsl::*};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Question {
    pub id: Option<i32>,
    pub question_text: String,
    pub pub_date: NaiveDateTime,
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
        .select(question_text)
        .filter(pub_date.lt(Local::now().naive_local()))
        .limit(5)
        .load::<String>(connection)
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
