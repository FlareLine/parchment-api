use chrono::{NaiveDateTime, Utc};
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct PostContent {
  title: String,
  content: String,
  archived: bool,
}

#[derive(Serialize, Debug)]
struct ThinPost {
  id: Uuid,
  title: String,
  subtitle: String,
  posted_date: NaiveDateTime,
  archived: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
  lambda!(get_post);
  Ok(())
}

fn get_post(data: PostContent, _cxt: Context) -> Result<ThinPost, HandlerError> {
  Ok(ThinPost {
    id: Uuid::new_v4(),
    title: data.title,
    subtitle: get_subtitle_from_content(data.content),
    posted_date: Utc::now().naive_utc(),
    archived: false,
  })
}

fn get_subtitle_from_content(content: String) -> String {
  content.chars().take(97).collect::<String>() + "..."
}
