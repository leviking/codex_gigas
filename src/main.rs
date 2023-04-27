use actix_web::{web, App, HttpResponse, HttpServer, Result};
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use base64::{Engine as _, engine::general_purpose};

async fn create_page() -> Result<HttpResponse> {
  let form = r#"
  <html>
  <head>
      <title>File Upload Form</title>
      <style>
          body {
              font-family: 'Press Start 2P', cursive;
              margin: 0;
              background-color: #000;
              color: #fff;
          }
          header {
              background-color: #000;
              color: #fff;
              padding: 10px;
              border: 2px solid #fff;
              border-radius: 5px;
              text-align: center;
              text-transform: uppercase;
          }
          h1 {
              margin: 0;
              font-size: 2em;
          }
          form {
              margin: 20px auto;
              max-width: 500px;
              padding: 20px;
              border: 2px solid #fff;
              border-radius: 5px;
              text-align: center;
          }
          label {
              display: block;
              margin-bottom: 5px;
              font-weight: bold;
              text-transform: uppercase;
          }
          input[type="text"],
          input[type="file"] {
              font-family: 'Press Start 2P', cursive;
              margin-bottom: 10px;
              padding: 10px;
              border: 2px solid #fff;
              border-radius: 5px;
              font-size: 1em;
              width: 100%;
              background-color: #000;
              color: #fff;
              text-transform: uppercase;
          }
          input[type="file"] {
              height: auto;
          }
          input[type="file"]::file-selector-button{
              font-family: 'Press Start 2P', cursive;
              margin-bottom: 10px;
              padding: 10px;
              border: 2px solid #fff;
              border-radius: 5px;
              font-size: 1em;
              background-color: #000;
              color: #fff;
              text-transform: uppercase;
          }
          input[type="file"]::file-selector-button:hover{
              background-color: #fff;
              color: #000;
          }
          input[type="submit"] {
              background-color: #ff00ff;
              color: #fff;
              border: none;
              border-radius: 5px;
              padding: 10px 20px;
              font-size: 1em;
              cursor: pointer;
              text-transform: uppercase;
              font-family: 'Press Start 2P', cursive;
              letter-spacing: 2px;
              box-shadow: 2px 2px 0px #fff;
          }
          input[type="submit"]:hover {
              background-color: #fff;
              color: #ff00ff;
              box-shadow: 2px 2px 0px #ff00ff;
          }
      </style>
      <link href="https://fonts.googleapis.com/css2?family=Press+Start+2P&display=swap" rel="stylesheet">
  </head>
  <body>
      <header>
          <h1>File Upload Form</h1>
      </header>
      <form action="/" method="POST" enctype="multipart/form-data">
          <label for="body">Body:</label>
          <input type="text" name="body" id="body">
          <label for="attachments">Attachments:</label>
          <input type="file" name="attachments" id="attachments" multiple>
          <input type="submit" value="Submit">
      </form>
  </body>
  </html>
"#;

  Ok(HttpResponse::Ok().content_type("text/html").body(form))
}

// function to take post request and return html page
async fn build_page (mut payload: Multipart) -> Result<HttpResponse>{
    let mut body = String::new();
    let mut images = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();
        let name = content_disposition.get_name().unwrap_or("");

        if name == "body" {
            while let Some(chunk) = field.next().await {
                if let Ok(data) = chunk {
                    let data_string = String::from_utf8_lossy(&data);
                    body = data_string.to_string();
                }
            }
        } else if name == "attachments" {
            let mut data = Vec::new();
            while let Some(chunk) = field.next().await {
                let chunk = chunk?;
                data.extend_from_slice(&chunk);
            }

            let content_type = field.content_type().expect("REASON").to_string();
            let data_uri = format!("data:{};base64,{}", content_type, general_purpose::STANDARD_NO_PAD.encode(&data));
  
            if !&data.is_empty() {
                images.push(format!("<img src=\"{}\" style=\"margin-top: 1rem; margin-bottom: 1rem;\">", data_uri));
            }
        } else {
            println!("Field '{}' not processed", name);
        }
    }

    let images_html = images.join("\n");

    let html = format!("
    <!DOCTYPE html>
        <html lang=\"en\">
        <head>
            <meta charset=\"UTF-8\">
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
            <title>Attachments</title>
            <style>
              img {{
                max-width: 90%;
                height: auto;
                padding-left: 5%;
                padding-right: 5%;
              }}
              html {{
                margin-right: 10px;
                margin-left: 10px;
                margin-top: 2px;
                margin-bottom: 2px;
                text-align: justify;
                background-color: white;
                font-family: sans-serif;
              }}
  
              a {{ font-weight: bold }}
  
              @media (prefers-color-scheme: dark) {{
                html {{
                  background-color: #000;
                  color: white;
                }}
  
                a {{ color: #00ccff }}
              }}
            </style>
        </head>
        <body>
            <div id=\"body-text\">{}</div>
            {}
        </body>
    </html>", body, images_html);

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(create_page))
            .route("/", web::post().to(build_page))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}