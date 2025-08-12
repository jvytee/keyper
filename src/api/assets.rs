use std::collections::HashMap;

use axum::{
    extract::Path,
    http::{StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};

pub async fn assets_endpoint(
    Path(filename): Path<String>,
) -> Result<Response, (StatusCode, String)> {
    let files: HashMap<String, (String, String)> = HashMap::from([(
        "pico.min.css".to_string(),
        (
            "text/css".to_string(),
            include_str!("templates/pico.min.css").to_string(),
        ),
    )]);

    if let Some((content_type, content)) = files.get(&filename) {
        let headers = [(CONTENT_TYPE, content_type)];
        Ok((headers, content.to_string()).into_response())
    } else {
        Err((
            StatusCode::NOT_FOUND,
            format!("{} not found", StatusCode::NOT_FOUND),
        ))
    }
}

#[cfg(test)]
mod tests {
    use axum::extract::Path;

    use crate::api::assets::assets_endpoint;

    #[tokio::test]
    async fn test_assets_endpoint() {
        let filename = Path("pico.min.css".to_string());
        let response = assets_endpoint(filename).await;

        assert!(response.is_ok());
    }
}
