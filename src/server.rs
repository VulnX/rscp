use std::path::PathBuf;
use rand::Rng;
use actix_web::{web, App, HttpServer, /*middleware*/};

use crate::{recv, send, shutdown, qr};

#[derive(Clone, Debug)]
pub enum Action {
    Upload { recv_path: PathBuf },
    Download { file_path: PathBuf },
}

pub async fn start_server(action: Action) -> std::io::Result<()> {
    // env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let stop_handle = web::Data::new(shutdown::StopHandle::default());
    let action_clone = web::Data::new(action.clone());
    let action_clone2 = action.clone();
    let mut rng = rand::thread_rng();
    let port = rng.gen_range(49152..=65535);
    let srv = HttpServer::new({
        let stop_handle = stop_handle.clone();

        move || {
            let mut app = App::new()
                // .wrap(middleware::Logger::default())
                .app_data(stop_handle.clone())
                .app_data(action_clone.clone());
                match action.clone() {
                    Action::Upload { recv_path: _ } => {
                        app = app.service(web::resource("/upload").route(web::get().to(recv::upload)))
                        .service(
                            web::resource("/upload/{filename}/{size}")
                                .route(web::post().to(recv::upload_handler)),
                        );
                    }
                    Action::Download { file_path: _ } => {
                        app = app.service(web::resource("/download").route(web::get().to(send::download)));
                    }
                }
                app
                
        }
    })
    .bind(("0.0.0.0", port.clone()))
    .unwrap()
    .run();

    stop_handle.register(srv.handle()).await;
    qr::show_qr(action_clone2, port);
    println!("[*] Server is listening on 0.0.0.0:{}", port);
    srv.await
}

