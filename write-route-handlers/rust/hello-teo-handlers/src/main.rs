mod entities;

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::main;
use teo::prelude::{App, Response, Result, Error, teon};
use crate::entities::{AlterCreatedAtInput, AlterCreatedAtInputTrait, EchoPathArguments, StaticPathArguments, Teo, UploadInput, UploadInputTrait};

#[main]
async fn main() -> Result<()> {
    let app = App::new()?;
    app.main_namespace().define_handler("hello", || async {
        let response = Response::html(r#"
            <html>
                <head>
                    <title>Hello, Teo handlers</title>
                </head>
                <body>
                    <h1>Hello, Teo handlers!</h1>
                </body>
            </html>
        "#)?;
        Ok(response)
    });
    app.main_namespace().define_handler("empty", || async {
        Ok(Response::empty())
    });
    app.main_namespace().define_handler("echo", |path_args: EchoPathArguments| async move {
        Ok(Response::string(path_args.data()?, "text/plain"))
    });
    app.main_namespace().define_handler("static", |path_args: StaticPathArguments| async move {
        Response::send_file("static", path_args.path()?)
    });
    app.main_namespace().define_model_handler_group("Record", |group| {
        group.define_handler("alterCreatedAt", |teo: Teo, input: AlterCreatedAtInput| async move {
            if let Some(record) = teo.record().find_unique_object(teon!({
                "where": {
                    "id": input.id()
                }
            })).await? {
                record.set_created_at(*input.created_at())?;
                record.save().await?;
                Ok(Response::data(record.to_teon().await?))
            } else {
                Err(Error::not_found())?
            }
        });
    });
    app.main_namespace().define_handler("upload", |input: UploadInput| async move {
        let original_location = PathBuf::from(input.file().filepath.as_str());
        let extension = if let Some(ext) = original_location.extension() {
            ".".to_owned() + ext.to_str().unwrap()
        } else {
            "".to_owned()
        };
        let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();
        let random_file_name = nanos.to_string();
        let destination_string = format!("static/images/{}{}", random_file_name, extension);
        let destination = PathBuf::from(destination_string.as_str());
        let path = "/".to_owned() + destination_string.as_str();
        match std::fs::rename(original_location, destination) {
            Ok(_) => {
                Ok(Response::data(teon!({
                    "path": path
                })))
            },
            Err(err) => {
                Err(Error::internal_server_error_message(err.to_string()))
            }
        }
    });
    app.run().await
}