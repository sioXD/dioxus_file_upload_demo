use dioxus::{fullstack::{FileStream}, prelude::*};
use dioxus::html::HasFileData;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut is_hovering = use_signal(|| false);
    let mut last_file = use_signal(|| None::<String>);

    let base_style = "width: 300px; height: 200px;  display: flex; align-items: center; justify-content: center; cursor: pointer; transition: all 0.2s ease-in-out; border-radius: 8px;";
    let hover_style = if is_hovering() {
    "border-color: #3182ce; background: rgba(49, 130, 206, 0.1);transform: scale(1.02);"
    } else {
        "border: 2px dashed black; background: transparent; transform: scale(1);"
    };


    rsx! {
        h3 { "Upload as FileUpload" }

        div { class: "upload-area",
            label {
                r#for: "pdfFileInput",
                class: "dropzone",
                style: format!("{} {}", base_style, hover_style),

                ondragover: move |evt| {
                    evt.stop_propagation();
                    evt.prevent_default();
                    is_hovering.set(true);
                },
                ondragleave: move |_| {
                    is_hovering.set(false);
                },

                ondrop: move |evt| async move {
                    evt.stop_propagation();
                    evt.prevent_default();
                    is_hovering.set(false);

                    for file in evt.files() {
                        last_file.set(Some(file.name()));
                        _ = upload_file(file.into()).await;
                    }
                },

                input {
                    r#type: "file",
                    id: "pdfFileInput",
                    accept: ".pdf",
                    multiple: false,
                    style: "display: none;",
                    onchange: move |evt| async move {
                        evt.prevent_default();

                        for file in evt.files() {
                            last_file.set(Some(file.name()));
                            _ = upload_file(file.into()).await;
                        }
                    },
                }
                "Drop your PDF here or click to select"
            }
        }
        div {
            if let Some(name) = last_file() {
                p { "Last uploaded file: {name}" }
            }
        }
    }
}


// BACKEND
#[server]
async fn upload_file(file: FileStream) -> Result<String, ServerFnError> {
    let file_size = file.size();
    let content_type = file.content_type();
    println!("file size: {:?}", file_size);
    println!("content type: {:?}", content_type);

    Ok("Upload successful".to_string())
}