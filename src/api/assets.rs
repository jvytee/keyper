use std::collections::HashMap;

use axum::{
    extract::Path,
    http::{StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};

pub async fn assets(Path(filename): Path<String>) -> Response {
    let files: HashMap<String, (String, String)> = HashMap::from([(
        "pico.min.css".to_string(),
        (
            "text/css".to_string(),
            include_str!("../../templates/pico.min.css").to_string(),
        ),
    )]);

    if let Some((content_type, content)) = files.get(&filename) {
        let headers = [(CONTENT_TYPE, content_type)];
        (headers, content.to_string()).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            format!("{} not found", StatusCode::NOT_FOUND),
        )
            .into_response()
    }
}
