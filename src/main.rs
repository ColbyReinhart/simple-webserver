// Simple webserver using rust rocket
// By Colby Reinhart
// 4-12-2023

use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;

#[macro_use] extern crate rocket;

//
// Rocket boilerplate
//

#[launch]
fn rocket() -> _
{
    rocket::build().mount("/", routes![get_resource])
}

#[get("/<path..>")]
async fn get_resource(path: PathBuf) -> Option<NamedFile>
{
    let mut route: PathBuf = Path::new("./").join(path);

    if route.is_dir()
	{
		route = route.join("index.html");
	}

	NamedFile::open(route).await.ok()
}
