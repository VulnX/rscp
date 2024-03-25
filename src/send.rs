use actix_web::{web, CustomizeResponder, HttpRequest, HttpResponse, Responder};

use crate::{server, shutdown};

pub async fn download(
    req: HttpRequest,
    stop_handle: web::Data<shutdown::StopHandle>,
    action: web::Data<server::Action>,
) -> CustomizeResponder<HttpResponse> {
    let file_path = match &**action {
        server::Action::Download { file_path } => file_path.clone(),
        _ => {
            panic!("file_path not found.");
        },
    };
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    println!("Sending file {:?}...", file_path);
    let file = actix_files::NamedFile::open_async(&file_path)
        .await
        .unwrap();
    stop_handle.stop().await;
    println!("[*] Server will automatically close once the file has been completely downloaded by client");
    file.into_response(&req).customize().insert_header((
        "Content-Disposition",
        format!("attachment; filename=\"{}\"", file_name),
    ))
}
