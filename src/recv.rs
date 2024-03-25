use actix_web::{web, HttpRequest, HttpResponse};
use futures_util::stream::StreamExt;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use tokio::io::{AsyncWriteExt, BufWriter};

use crate::{shutdown, server};

pub async fn upload() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("./upload.html"))
}

pub async fn upload_handler(
    mut payload: web::Payload,
    req: HttpRequest,
    stop_handle: web::Data<shutdown::StopHandle>,
    action: web::Data<server::Action>,
) -> HttpResponse {
    let filename = req
        .match_info()
        .get("filename")
        .unwrap_or("unknown-file.bin");
    let total_size = req
        .match_info()
        .get("size")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let mut write_path = match &**action {
        server::Action::Upload { recv_path } => recv_path.clone(),
        _ => {
            panic!("file_path not found.");
        },
    };
    let available_space = fs2::available_space(&write_path).unwrap();
    write_path.push(filename);
    if total_size >= available_space {
        eprintln!("[!] Cannot fit on filesystem, please choose a smaller file.");
        return HttpResponse::PayloadTooLarge().finish();
    }

    let mut size_written = 0;
    let file = tokio::fs::File::create(&write_path)
        .await
        .unwrap();
    let mut buf_writer = BufWriter::new(file);
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.blue/black}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("━━━"));
    println!("[*] Storing on filesystem...");
    while let Some(chunk) = payload.next().await {
        let chunk = chunk.unwrap();
        buf_writer.write_all(&chunk).await.unwrap();
        size_written += chunk.len();
        pb.set_position(size_written as u64);
    }
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.green/black}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("━━━"));
    pb.finish_with_message("Saved on filesystem");

    buf_writer.flush().await.unwrap();
    println!("\n[+] File stored at : {:?}", dunce::canonicalize(&write_path).unwrap_or_else(|_| write_path));
    println!("[*] Gracefully shutting down, please wait...");
    stop_handle.stop().await;
    HttpResponse::Ok().finish()
}