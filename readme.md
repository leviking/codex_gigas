# Codex Gigas

A simple image uploader web application that allows users to upload multiple images along with some text. The server returns an HTML page displaying the uploaded images and the text entered by the user.

## Features
Accepts multiple image files and text input
Displays uploaded images inline on the resulting HTML page
Base64 encodes image data for embedding in the HTML
Built with Rust and Actix Web

## Getting Started
### Prerequisites
Install Rust and its package manager, Cargo.
Setup
Clone the repository:


```
git clone
cd codex_gigas
```

Build the project:

`cargo build --release`

### Run the server:

```
cargo run --release
The server is now running at http://localhost:8000.
```

### Docker
```
docker build -t codex_gigas .
docker run -p 8000:8000 codex_gigas
```

## Usage
Send a POST request to the / endpoint with a multipart/form-data content type, including image files and a "body" text field.
The server will respond with an HTML page containing the uploaded images and the text provided in the "body" field.
