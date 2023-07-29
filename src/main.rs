#[macro_use]
extern crate rocket;

mod paste_id;

use paste_id::PasteId;
use rocket::tokio::fs::File;
use std::path::Path;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retrieve])
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[get("/<id>")]
async fn retrieve(id: &str) -> Option<File> {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let filename = Path::new(upload_dir).join(id);
    File::open(filename).await.ok()
}
