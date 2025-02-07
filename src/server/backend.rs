use crate::server::AppState;
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_macros::debug_handler;
use serde::Serialize;
use std::sync::Arc;
use std::{io::Result, path::PathBuf, vec::Vec};
use tokio::fs;

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
    if *state.parent {
        if let Err(errm) = dirgen(state.drop_location.clone()).await {
            tokio::spawn(
                async move { while multipart.next_field().await.unwrap_or(None).is_some() {} },
            );
            println!(
                "Unable to access {}: {}",
                state.drop_location.display(),
                errm
            );
            return Json(ResponseCode {
                body: format!("{}", errm).to_string(),
                success: false,
            })
            .into_response();
        }
    }
    while let Some(mut field) = match multipart.next_field().await {
        Ok(chunk) => chunk,
        Err(err) => {
            return (StatusCode::BAD_REQUEST, format!("Error: {}", err)).into_response();
        }
    } {
        let name = field.name().unwrap_or("<unnamed>").to_string();
        let file_name = field.file_name().unwrap_or("<no filename>").to_string();
        let content_type = field
            .content_type()
            .unwrap_or("<no content type>")
            .to_string();

        let mut size = 0;
        while let Some(chunk) = field.chunk().await.unwrap_or(None) {
            size += chunk.len();
        }

        fields.push(FieldInfo {
            name,
            file_name,
            content_type,
            size,
        });
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

async fn dirgen(path: Arc<PathBuf>) -> Result<()> {
    if !fs::try_exists(&*path).await? {
        fs::create_dir_all(&*path).await?
    }
    Ok(())
}
