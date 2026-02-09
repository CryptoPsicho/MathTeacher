use axum::{
    body::Body,
    extract::Json,
    http::{header, HeaderValue, Method, Response, StatusCode},
    routing::post,
    Router,
};
use bytes::Bytes;
use printpdf::{BuiltinFont, Mm, PdfDocument};
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::collections::VecDeque;
use serde::Deserialize;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct WorksheetRequest {
    tables: Vec<u8>,
    count: Option<usize>,
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new().route("/api/worksheet", post(generate_pdf)).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4001")
        .await
        .expect("bind server address");
    axum::serve(listener, app).await.expect("start server");
}

async fn generate_pdf(Json(payload): Json<WorksheetRequest>) -> Result<Response<Body>, StatusCode> {
    if payload.tables.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let tables: Vec<u8> = payload
        .tables
        .into_iter()
        .filter(|value| (1..=10).contains(value))
        .collect();

    if tables.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let total = payload.count.unwrap_or(30).clamp(1, 30);
    let operations = build_operations(&tables, total);
    let pdf_bytes = render_pdf(&operations).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut response = Response::new(Body::from(Bytes::from(pdf_bytes)));
    let headers = response.headers_mut();
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_static("inline; filename=worksheet.pdf"),
    );
    Ok(response)
}

fn build_operations(tables: &[u8], total: usize) -> Vec<String> {
    let mut rng = thread_rng();
    let mut operations = Vec::with_capacity(total);
    let mut used_zero = false;
    let mut used_one = false;
    let mut used_ten = false;
    let mut recent_multipliers: VecDeque<u8> = VecDeque::with_capacity(4);

    for _ in 0..total {
        let table = tables.choose(&mut rng).copied().unwrap_or(1);
        let candidates: Vec<u8> = (0..=10)
            .filter(|value| match *value {
                0 => !used_zero,
                1 => !used_one,
                10 => !used_ten,
                _ => true,
            })
            .filter(|value| !recent_multipliers.contains(value))
            .collect();
        let multiplier = *candidates
            .choose(&mut rng)
            .unwrap_or(&rng.gen_range(2..=9));
        match multiplier {
            0 => used_zero = true,
            1 => used_one = true,
            10 => used_ten = true,
            _ => {}
        }
        recent_multipliers.push_back(multiplier);
        if recent_multipliers.len() > 4 {
            recent_multipliers.pop_front();
        }
        operations.push(format!("{} x {} = ___", multiplier, table));
    }

    operations
}

fn render_pdf(operations: &[String]) -> Result<Vec<u8>, printpdf::Error> {
    let (doc, page, layer) = PdfDocument::new(
        "Math Worksheet",
        Mm(210.0_f32),
        Mm(297.0_f32),
        "Layer 1",
    );
    let layer = doc.get_page(page).get_layer(layer);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;

    let page_width = 210.0_f32;
    let page_height = 297.0_f32;
    let margin_x = 18.0_f32;
    let margin_top = 18.0_f32;
    let margin_bottom = 18.0_f32;
    let gutter = 12.0_f32;
    let rows = (operations.len() + 1) / 2;
    let available_height = (page_height - margin_top - margin_bottom).max(0.0);
    let pt_per_mm = 2.834_65_f32;
    let available_height_pt = available_height * pt_per_mm;
    let leading_ratio = 1.0_f32;
    let font_size = if rows > 1 {
        let total_units = rows as f32 + (rows as f32 - 1.0) * leading_ratio;
        available_height_pt / total_units
    } else {
        24.0
    };
    let line_spacing_pt = if rows > 1 {
        font_size * (1.0 + leading_ratio)
    } else {
        0.0
    };
    let line_spacing = line_spacing_pt / pt_per_mm;
    let top_y = page_height - margin_top;
    let column_width = (page_width - margin_x * 2.0 - gutter).max(0.0) / 2.0;
    let left_x = margin_x;
    let right_x = margin_x + column_width + gutter;

    for (index, operation) in operations.iter().enumerate() {
        let column = if rows == 0 { 0 } else { index / rows };
        let row = if rows == 0 { 0 } else { index % rows };
        let x = if column == 0 { left_x } else { right_x };
        let y = top_y - line_spacing * row as f32;
        layer.use_text(operation, font_size, Mm(x), Mm(y), &font);
    }

    doc.save_to_bytes()
}
