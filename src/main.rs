#![feature(proc_macro_hygiene, decl_macro)]

///
///         rscp
///         A blazingly fast Rust-based file transfer utility for quick sharing in local network.
///
///         Author  : VulnX
///         Version : 1.1.0
///         License : MIT
///         GitHub  : https://github.com/vulnx/rscp
/// 

use std::{io::{self, Write, BufWriter}, path::{PathBuf, Path}, fs::{self, File}, env};
use local_ip_address::local_ip;
use rocket::{response::{NamedFile, content::Html, Content}, Data, Config, config::Environment, logger::LoggingLevel, http::{Status, ContentType}};
use qrcode::{QrCode, render::unicode::Dense1x2};
use fs2::available_space;

#[macro_use]
extern crate rocket;

#[get("/get_available_space")]
fn get_available_space() -> io::Result<rocket::response::content::Json<String>> {
    let avail_space = available_space(".")?;
    let response_json = format!(
        "{{ \"availableSpace\": {} }}",
        avail_space
    );

    Ok(rocket::response::content::Json(response_json))
}

#[get("/upload")]
fn upload() -> Html<String> {
    Html(r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Upload</title>
    </head>
    <style>
* {
    margin: 0;
    padding: 0;
    font-size: 1.2rem;
    transition: .1s;
    font-family: sans-serif;
}
body {
    max-width: 100vw;
    max-height: 100vh;
    overflow: hidden;
}
.container {
    display: flex;
    flex-direction: column;
    padding: 15px;
}
.container * {
    margin: 20px 0;
}
.title-container {
    text-align: center;
}
.title {
    font-weight: bold;
    border-bottom: thin solid black;
}
#fileInput {
    display: none;
}
#progress {
    opacity: 0;
    height: 10px;
    background-color: #09f;
    border-radius: 10px;
}
.status-container {
    background-color: #ddd;
    border-left: 5px solid #aaa;
    padding: 10px;
    border-radius: 5px;
    opacity: 0;
}
.status-prefix {
    font-weight: bold;
    border-bottom: thin solid black;
}
button {
    outline: none;
    border: none;
    border-radius: 5px;
    background-color: #09f;
    padding: 10px;
    color: white;
    font-weight: bold;
    text-transform: uppercase;
}
button:hover, button:active, button:disabled {
    background-color: #333;
}
.modified-input-btn-container {
    display: flex;
    flex-direction: column;
}
#fileSelectedMessage {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}
    </style>
    <body>
        <div class="container">
            <div class="title-container">
                <span class="title">Upload a file</span>
            </div>
            <input type="file" id="fileInput" onchange="fileUpdated()" name="file">
            <div class="modified-input-btn-container">
                <span id="fileSelectedMessage">No file selected</span>
                <button onclick="selectFile()">Select file</button>
            </div>
            <div class="status-container">
                <span class="status-prefix">STATUS:</span>
                <span id="status-actual"></span>
            </div>
            <div id="progress" value="0" max="100"></div>
            <button id="uploadBtn" onclick="setTimeout(uploadFile, 100)">Upload</button>
        </div>
        <script>
            function selectFile() {
                document.querySelector('#fileInput').click();
            }
            function fileUpdated() {
                document.querySelector('#fileSelectedMessage').innerText = 
                    (!document.querySelector('#fileInput').files[0]) ? 'No file selected'
                    : document.querySelector('#fileInput').files[0].name;
                
                document.querySelector('#status-actual').innerText = '';
                document.querySelector('.status-container').style.opacity = '0';
                document.querySelector('#progress').style.opacity = '0';
                document.querySelector('#progress').style.width = '0';
            }
            async function uploadFile() {
                var file = document.querySelector('#fileInput').files[0];
                var progressBar = document.querySelector('#progress');
                var statusBar = document.querySelector('.status-container');
                var status = document.querySelector('#status-actual');
                var uploadBtn = document.querySelector('#uploadBtn');
                if (!file) {
                    alert('Please select a file');
                    return;
                }

                const response = await fetch('/get_available_space');
                const { availableSpace } = await response.json();
                
                if (file.size >= availableSpace) {
                    alert('Not enough space on server to upload file');
                    return;
                }

                var xhr = new XMLHttpRequest();
                xhr.open('POST', '/upload_file/' + encodeURIComponent(file.name), true);

                xhr.upload.onprogress = function (event) {
                    if (event.lengthComputable) {
                        progressBar.style.opacity = '1';
                        statusBar.style.opacity = '1';
                        var percentage = ( event.loaded / event.total ) * 100;
                        progressBar.style.width = Math.round(percentage) + '%';
                        uploadBtn.setAttribute('disabled', 'true')
                        status.innerText = 'Uploading...' + Math.round(percentage) + '%';
                    }
                }

                xhr.onload = function() {
                    if(xhr.status === 200) {
                        status.innerText = 'Uploaded';
                    } else {
                        alert('File upload failed');
                    }
                    uploadBtn.removeAttribute('disabled');
                }

                xhr.onerror = function () {
                    alert('File upload failed.');
                }

                xhr.send(file);
            }
        </script>
    </body>
</html>
"#.to_string())
}

#[post("/upload_file/<filename>", data = "<file>")]
fn upload_file(filename: String, file: Data) -> Result<Status, std::io::Error> {
    let file_path: PathBuf = ["files", &filename].iter().collect();

    let mut dest_file = BufWriter::new(File::create(&file_path)?);

    println!("[*] Storing file on system...");
    file.stream_to(&mut dest_file)?;
    println!("[+] Done");

    let abs_path = file_path.canonicalize()?;
    let abs_path = abs_path.to_string_lossy();
    let abs_path = abs_path
        .strip_prefix("\\\\?\\")        // Canonicalize() return the UNC path on Windows which needs to be stripped
        .unwrap_or(&abs_path);          // But on *NIX systems we need not strip anything.
    println!("File saved at : {}", abs_path);
    // A graceful programmatic shutdown will hopefully be added in
    // Rocket 0.5 but as of now unfornately, CTRL+C is the only way
    println!("Press CTRL+C to quit.");

    Ok(Status::Ok)
}

#[get("/download_file")]
fn download_file() -> Content<Option<NamedFile>> {
    if let Some(file_path) = env::var("file_path").ok() {
        let file = PathBuf::from(file_path);

        if file.exists() && file.is_file() {
            // A graceful programmatic shutdown will hopefully be added in
            // Rocket 0.5 but as of now unfornately, CTRL+C is the only way
            println!("File is being downloaded in your other device.\nPress CTRL+C to quit the application (after download is complete).");
            return Content(ContentType::Binary, NamedFile::open(&file).ok());
        }
    }

    Content(ContentType::Any, None)

}

#[get("/download")]
fn download() -> Html<String> {
    if let Some(file_path) = env::var("file_path").ok() {
        let file = PathBuf::from(file_path);

        if file.exists() && file.is_file() {
            let filename = file.file_name().unwrap().to_string_lossy();
            let formatted_html = r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <title>{filename}</title>
            </head>
            <body>
                <script>
                    window.onload = () => {
                        let a = document.createElement('a');
                        a.href = '/download_file';
                        a.download = '{filename}';
                        a.click();
                    }
                </script>
            </body>
            </html>
"#.replace("{filename}", &filename);
            return Html(formatted_html);
        }
    }

    Html("404".to_string())
}

fn print_n_flush<S: AsRef<str>>(prompt: S) {
    print!("{}", prompt.as_ref());
    io::stdout().flush().expect("Failed to flush stdout");
}

fn main() {
    print_n_flush(r#"
Choose
[1] Send
[2] Receive
=> "#);
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read from stdin");
    let choice : i32 = match choice.trim().parse() {
        Ok(choice) => choice,
        Err(_) => {
            println!("PLEASE ENTER A NUMBER");
            main();
            panic!()
        }
    };

    let (routes, url_format) = match choice {
        1 => {
            print_n_flush("FILE PATH: ");
            let mut file_path = String::new();
            io::stdin().read_line(&mut file_path).expect("Failed to read from stdin");
            let file_path = file_path.trim().trim_matches(|c| c == '"' || c == '\'').replace("\\ ", " ");
            let file = Path::new(&file_path);
            if !( file.exists() && file.is_file() ) {
                println!("Such file does not exist.");
                main();
            }
            env::set_var("file_path", file_path);
            (routes![download, download_file], "http://{}:8000/download")
        }
        2 => {
            if !(fs::metadata("files")
                    .map(|metadata| metadata.is_dir())
                    .unwrap_or(false)) {
                fs::create_dir("files").expect("Failed to create the directory");
            }
            (routes![upload, upload_file, get_available_space], "http://{}:8000/upload")
        }
        _ => {
            println!("INVALID CHOICE. PLEASE RETRY");
            main();
            panic!()
        }
    };

    let ip = match local_ip() {
        Ok(ip) => ip.to_string(),
        Err(local_ip_address::Error::LocalIpAddressNotFound) => {
            panic!("Failed to find the local IP address.");
        }
        Err(e) => panic!("{}", e)
    };

    let url = url_format.replace("{}", ip.as_str());
    let qr_code = QrCode::new(&url).unwrap();
    let qrcode_str = qr_code.render::<Dense1x2>().quiet_zone(false).module_dimensions(1, 1).build();
    println!("Scan the qrcode given below \n\n{}\n\nOr manually open the URL in browser : {}", qrcode_str, url);

    let config = Config::build(Environment::Production)
        .log_level(LoggingLevel::Off)
        .address("0.0.0.0")
        .port(8000)
        .finalize()
        .unwrap();
    rocket::custom(config)
        .mount("/", routes)
        .launch();
}
