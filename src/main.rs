#[macro_use]
extern crate rocket;

mod paste_id;

use paste_id::PasteId;
use rocket::data::{Data, ToByteUnit};
use rocket::response::Debug;
use rocket::tokio::fs::File;

// Home page
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

// Upload route
#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String, Debug<std::io::Error>> {
    let id = PasteId::new(5);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    // Write the paste out, limited to 128KiB, and return the URL.
    paste.open(128.kibibytes()).into_file(filename).await?;
    Ok(url)
}

// Retrieving pastes
#[get("/<id>")]
async fn retrieve(id: &str) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload, retrieve])
}
