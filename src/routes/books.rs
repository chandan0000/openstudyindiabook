use std::io::Read;

use axum::{
    extract::{Multipart, State},
    http::StatusCode,
};
use chrono::Utc;
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
            // println!("data {:?}", data.bytes());

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




// use axum::{
//     body::{boxed},
//     extract::{Multipart, Query},
//     http::StatusCode,
//     response::Response,
//     routing::{on, MethodFilter},
//     Router,
// };
// use axum_extra::body::AsyncReadBody;
// use futures::TryStreamExt;
// use s3::{creds::Credentials, Bucket};
// use std::{io, net::SocketAddr, pin::Pin};
// use tokio::io::{AsyncRead, AsyncWrite};
// use tokio_util::io::StreamReader;

// #[tokio::main]
// async fn main() {
//     let router = Router::new()
//         .route("/upload", on(MethodFilter::POST, upload))
//         .route("/download/*key", on(MethodFilter::GET, download));

//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

//     axum::Server::bind(&addr)
//         .serve(router.into_make_service())
//         .await
//         .unwrap();
// }

// pub async fn upload(mut multipart: Multipart) -> Result<Response, StatusCode> {
//     while let Some(field) = multipart.next_field().await.unwrap() {
//         let filename = if let Some(filename) = field.file_name() {
//             filename.to_string()
//         } else {
//             continue;
//         };

//         let bucket = Bucket::new(
//             "test",
//             "us-east-1".parse().unwrap(),
//             Credentials::default().unwrap(),
//         )
//         .unwrap();

//         let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));

//         let body_reader = StreamReader::new(body_with_io_error);

//         futures::pin_mut!(body_reader);

//         put_file(bucket, &filename, body_reader).await.unwrap();

//         return Ok(Response::builder()
//             .status(StatusCode::CREATED)
//             .body(boxed("OK".to_string()))
//             .unwrap());
//     }

//     Err(StatusCode::INTERNAL_SERVER_ERROR)
// }

// async fn put_file(
//     bucket: Bucket,
//     filename: &str,
//     mut reader: Pin<&mut (dyn AsyncRead + Send)>,
// ) -> Result<(), ()> {
//     bucket
//         .put_object_stream(&mut reader, filename)
//         .await
//         .unwrap();

//     Ok(())
// }

// pub async fn download(Query(params): Query<Vec<(String, String)>>) -> Result<Response, StatusCode> {
//     let filename = params[0].1.to_string();

//     let bucket = Bucket::new(
//         "test",
//         "us-east-1".parse().unwrap(),
//         Credentials::default().unwrap(),
//     )
//     .unwrap();

//     let (tx, rx) = tokio::io::duplex(65_536);

//     futures::pin_mut!(tx);

//     let body = AsyncReadBody::new(rx);

//     get_file(bucket, &filename, tx).await.unwrap();

//     let response = Response::builder().body(boxed(body)).unwrap();

//     Ok(response)
// }

// async fn get_file(
//     bucket: Bucket,
//     filename: &str,
//     mut writer: Pin<&mut (dyn AsyncWrite + Send)>,
// ) -> Result<(), ()> {
//     bucket.get_object_stream(filename, &mut writer).await.unwrap();

//     Ok(())
// }