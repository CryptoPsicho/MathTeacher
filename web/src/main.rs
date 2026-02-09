use dioxus::prelude::*;
// use dioxus::prelude::*;
use gloo_net::http::Request;
use serde::Serialize;

#[derive(Serialize)]
struct WorksheetRequest {
    tables: Vec<u8>,
    count: u32,
}

fn main() {
    dioxus::launch(app);
}

fn app() -> Element {
    let selected = use_signal(|| vec![false; 10]);
    let mut status = use_signal(String::new);
    let mut count = use_signal(|| 30u32);
    let mut download_url = use_signal(|| Option::<String>::None);

    let mut create_request = move || {
        let tables: Vec<u8> = selected
            .read()
            .iter()
            .enumerate()
            .filter_map(|(index, enabled)| enabled.then_some((index + 1) as u8))
            .collect();

        if tables.is_empty() {
            status.set("Select at least one table.".to_string());
            return;
        }

        status.set("Generating worksheet...".to_string());
        download_url.set(None);
        let count_value = *count.read();
        let mut status = status.to_owned();
        spawn(async move {
            let payload = WorksheetRequest {
                tables,
                count: count_value,
            };

            let response = match Request::post(&api_endpoint())
                .header("Content-Type", "application/json")
                .json(&payload)
                .expect("payload")
                .send()
                .await
            {
                Ok(value) => value,
                Err(_) => {
                    status.set("Failed to generate worksheet.".to_string());
                    return;
                }
            };

            if !response.ok() {
                status.set("Failed to generate worksheet.".to_string());
                return;
            }

            let blob = match response.binary().await {
                Ok(bytes) => bytes,
                Err(_) => {
                    status.set("Failed to generate worksheet.".to_string());
                    return;
                }
            };

            let array = js_sys::Uint8Array::from(blob.as_slice());
            let parts = js_sys::Array::new();
            parts.push(&array.buffer());
            let pdf_blob = web_sys::Blob::new_with_u8_array_sequence(&parts).expect("blob");
            let url = web_sys::Url::create_object_url_with_blob(&pdf_blob).expect("blob url");
            download_url.set(Some(url));
            status.set("Worksheet ready.".to_string());
        });
    };
    let on_create = move |_| create_request();
    let on_regenerate = move |_| create_request();

    rsx! {
        div { class: "app",
            link {
                rel: "stylesheet",
                href: "/assets/style.css"
            }
            header { class: "hero",
                h1 { "Math Teacher" }
                p { "Pick the tables, then generate a printable worksheet." }
            }
            section { class: "panel",
                h2 { "Tables" }
                div { class: "grid",
                    TableSwitch { index: 0, selected: selected }
                    TableSwitch { index: 1, selected: selected }
                    TableSwitch { index: 2, selected: selected }
                    TableSwitch { index: 3, selected: selected }
                    TableSwitch { index: 4, selected: selected }
                    TableSwitch { index: 5, selected: selected }
                    TableSwitch { index: 6, selected: selected }
                    TableSwitch { index: 7, selected: selected }
                    TableSwitch { index: 8, selected: selected }
                    TableSwitch { index: 9, selected: selected }
                }
                div { class: "controls",
                    div { class: "field",
                        label { class: "field-label", "Questions" }
                        input {
                            class: "field-input",
                            r#type: "number",
                            min: "1",
                            max: "30",
                            value: "{count.read()}",
                            oninput: move |event| {
                                let value = event.value().parse::<u32>().unwrap_or(30).clamp(1, 30);
                                count.set(value);
                            }
                        }
                    }
                    button { class: "cta", onclick: on_create, "Create" }
                    button { class: "ghost", onclick: on_regenerate, "Regenerate" }
                    if let Some(url) = download_url.read().as_ref() {
                        a {
                            class: "ghost",
                            href: "{url}",
                            download: "worksheet.pdf",
                            "Download"
                        }
                    }
                }
                p { class: "status", "{status.read()}" }
            }
        }
    }
}

fn api_endpoint() -> String {
    let window = web_sys::window().expect("window");
    let location = window.location();
    let protocol = location.protocol().unwrap_or_else(|_| "http:".to_string());
    let hostname = location
        .hostname()
        .unwrap_or_else(|_| "127.0.0.1".to_string());
    format!("{protocol}//{hostname}:4001/api/worksheet")
}

#[component]
fn TableSwitch(index: usize, selected: Signal<Vec<bool>>) -> Element {
    let table = index + 1;
    let active = selected.read()[index];
    rsx! {
        label { class: "switch",
            input {
                r#type: "checkbox",
                checked: active,
                onchange: move |_| {
                    let mut next = selected.read().clone();
                    if let Some(slot) = next.get_mut(index) {
                        *slot = !*slot;
                    }
                    selected.set(next);
                }
            }
            span { class: "slider" }
            span { class: "label", "Table {table}" }
        }
    }
}
