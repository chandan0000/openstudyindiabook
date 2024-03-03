use axum::{
    extract::{Multipart, State},
    http::StatusCode,
};
use chrono::{Utc};
use futures::{io, TryStreamExt};

use sea_orm::DatabaseConnection;

use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::io::StreamReader;

pub async fn book_created(
    State(_databse): State<DatabaseConnection>,
    mut multipart: Multipart,
) -> Result<(), StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();
        let timenow = Utc::now().timestamp();
        if field_name == "image" {
            let data = field.bytes().await.unwrap();
            let mut file = File::create(format!("./public/uploads/{}.png", timenow,))
                .await
                .unwrap();

            file.write(&data).await.unwrap();
        } else {
            let data = field.text().await.unwrap();
            println!("field: {}      value: {}", field_name, data);
        }
    }

    // while let Some(field) = multipart.next_field().await.unwrap() {
    //     let filename = if let Some(filename) = field.file_name() {
    //         filename.to_string()
    //     } else {

    //         continue;
    //     };

    //     // let options = UploadOptions::new().set_public_id(filename.to_string());
    //     // let upload = Upload::new(
    //     //     "153129147683852".to_string(),
    //     //     "dv4uxyxoj".to_string(),
    //     //     "yiKijdDX4c0YqRkw5wscSnxmpDw".to_string(),
    //     // );

    //     // futures::pin_mut!(body_reader);
    // }

    Ok(())
}
pub async fn upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let _filename = if let Some(filename) = field.file_name() {
            filename.to_string()
        } else {
            continue;
        };

        // let bucket = Bucket::new(
        //     "test",
        //     "us-east-1".parse().unwrap(),
        //     Credentials::default().unwrap(),
        // )
        // .unwrap();

        let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));

        let body_reader = StreamReader::new(body_with_io_error);

        futures::pin_mut!(body_reader);

        // // put_file(bucket, &filename, body_reader).await.unwrap();

        // return Ok(Response::builder()
        //     .status(StatusCode::CREATED)
        //     .body(boxed("OK".to_string()))
        //     .unwrap());
        todo!()
    }

    // Err(StatusCode::INTERNAL_SERVER_ERROR)
}
