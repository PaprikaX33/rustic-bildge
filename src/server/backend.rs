use crate::server::AppState;
use crate::time;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_macros::debug_handler;
use serde::Serialize;
use std::sync::Arc;
use std::{
    io::{ErrorKind, Result},
    path::PathBuf,
    vec::Vec,
};
use tokio::{fs, io, io::AsyncWriteExt};

#[derive(Serialize, Debug)]
struct FieldInfo {
    name: String,
    file_name: String,
    content_type: String,
    size: usize,
}
#[derive(Serialize)]
struct MultiResponse {
    droploc: String,
    fields: Vec<FieldInfo>,
}

#[derive(Serialize)]
struct ResponseCode {
    success: bool,
    body: String,
}

#[debug_handler]
pub async fn receive(state: State<AppState>, mut multipart: Multipart) -> impl IntoResponse {
    let mut fields = Vec::new();
    while let Some(mut field) = match multipart.next_field().await {
        Ok(chunk) => chunk,
        Err(err) => {
            return (StatusCode::BAD_REQUEST, format!("Error: {}", err)).into_response();
        }
    } {
        println!("received:");
        let name = field.name().unwrap_or("<unnamed>").to_string();
        let file_name = field
            .file_name()
            .unwrap_or(&time::current_time())
            .to_string();
        let content_type = field
            .content_type()
            .unwrap_or("<no content type>")
            .to_string();
        let file_path = match filename_normalization(state.drop_location.join(&file_name)).await {
            Ok(fpath) => fpath,
            Err(err) => return error_response(err).await,
        };
        println!("Saving: {}", &file_path.display());
        let mut fd = io::BufWriter::new(match fs::File::create(file_path).await {
            Ok(fd) => fd,
            Err(err) => return error_response(err).await,
        });
        let mut size = 0;
        while let Some(chunk) = field.chunk().await.unwrap_or(None) {
            size += chunk.len();
            if let Err(err) = fd.write_all(&chunk).await {
                return error_response(err).await;
            };
        }
        fields.push(FieldInfo {
            name,
            file_name,
            content_type,
            size,
        });
        if let Err(err) = fd.flush().await {
            return error_response(err).await;
        };
    }
    //for i in fields {}
    println!("Recieved: {:?}", fields);
    let resp = MultiResponse {
        droploc: state.drop_location.display().to_string(),
        fields,
    };
    // Convert the field metadata to JSON and return it as the response
    Json(resp).into_response()
}

async fn error_response(err: std::io::Error) -> axum::http::Response<axum::body::Body> {
    println!("Error generated: {}", err);
    Json(ResponseCode {
        body: format!("{}", err).to_string(),
        success: false,
    })
    .into_response()
}

async fn filename_normalization(path: PathBuf) -> Result<PathBuf> {
    println!("path is {}", path.display());
    //NOTE: canonicalize also checks if path is exists
    let raw_file_path = path;
    println!("Checking {}", raw_file_path.display());
    if fs::try_exists(&raw_file_path).await? {
        file_rename(&raw_file_path)
    } else {
        Ok(raw_file_path)
    }
}
fn file_rename(path: &std::path::Path) -> Result<PathBuf> {
    Ok(path.with_file_name(
        [
            path.file_stem()
                .and_then(|s| s.to_str())
                .ok_or(ErrorKind::Other)?,
            "_",
            &time::current_time_lin(),
            ".",
            path.extension().and_then(|s| s.to_str()).unwrap_or(""),
        ]
        .join(""),
    ))
}
