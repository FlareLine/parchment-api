use std::error::Error;
use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, error};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;
use simple_logger;

#[derive(Deserialize)]
struct PostContent {
  title: String,
  subtext: String,
  content: String,
  archived: bool,
}

#[derive(Serialize)]
struct ThinPost {
  id: i32,
  title: String,
  subtext: String,
  archived: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
  simple_logger::init_with_level(log::Level::Debug)?;
  lambda!(create_post);
  Ok(())
}

fn create_post(data: PostContent, cxt: Context) -> Result<ThinPost, HandlerError> {
  if data.title == "" {
    error!("Invalid post title in request {}", cxt.aws_request_id);
    bail!("Post title is invalid.");
  }
  if data.subtext == "" {
    error!("Invalid post subtext in request {}", cxt.aws_request_id);
    bail!("Post subtext is invalid.");
  }
  if data.content == "" {
    error!("Invalid post content in request {}", cxt.aws_request_id);
    bail!("Post content is invalid.");
  }

  Ok(ThinPost {
    id: 420,
    title: data.title,
    subtext: data.subtext,
    archived: data.archived,
  })
}
