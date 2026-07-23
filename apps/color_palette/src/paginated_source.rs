// Copyright (C) 2023 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR BSD-3-Clause

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::{QObjectHolder, invoke_method};
use qtbridge::qtbridge_type_lib::QString;

use crate::rest_service::Service;
use crate::utils::{is_success, send};

/// Manages a paginated CRUD resource: a list of JSON items served one page at a
/// time. Each page is returned as a `Vec<serde_json::Value>` so QML delegates can
/// access fields directly (`modelData.color`, `modelData.email`, ...)
pub struct PaginatedResource {
    path: String,
    data: Vec<serde_json::Value>,
    pages: i32,
    current_page: i32,
    /// Networking handle, injected by `RestService` in `componentComplete()`.
    service: Option<Rc<RefCell<Service>>>,
}

impl Default for PaginatedResource {
    fn default() -> Self {
        Self {
            path: String::new(),
            data: Vec::new(),
            pages: 0,
            current_page: 1,
            service: None,
        }
    }
}

#[qtbridge::qobject]
impl PaginatedResource {
    qproperty!("path", Member = path);
    qproperty!("data", Member = data, Notify = data_updated);
    qproperty!("page", Read = page, Write = set_page, Notify = page_updated);
    qproperty!("pages", Member = pages, Notify = pages_updated);

    #[qsignal(qml_name = "dataUpdated")]
    fn data_updated(&mut self);

    #[qsignal(qml_name = "pageUpdated")]
    fn page_updated(&mut self);

    #[qsignal(qml_name = "pagesUpdated")]
    fn pages_updated(&mut self);

    #[qsignal(qml_name = "errorOccurred")]
    fn error_occurred(&mut self, message: String);

    fn page(&self) -> i32 {
        self.current_page
    }

    fn set_page(&mut self, page: i32) {
        if self.current_page == page || page < 1 {
            return;
        }
        self.current_page = page;
        self.page_updated();
        self.refresh_current_page();
    }

    /// GET the current page and refresh `data` / `pages`.
    #[qslot(qml_name = "refreshCurrentPage")]
    fn refresh_current_page(&self) {
        let Some(service) = self.service.as_ref() else {
            return;
        };
        let service = service.borrow();
        let url = format!("{}?page={}", service.url_for(&self.path), self.current_page);
        let headers = service.common_headers();
        let client = service.client();
        let invoker = self.get_qml_method_invoker();

        service.spawn(async move {
            let mut request = client.get(&url);
            for (key, value) in headers {
                request = request.header(key, value);
            }
            let (body, status) = send(request).await;
            if !is_success(status) {
                eprintln!("[colorpalette] GET {url} -> HTTP {status}: {body}");
            }
            invoke_method!(invoker, "on_refresh_finished", QString::from(body), status);
        });
    }

    /// POST a new item, then refresh on success.
    #[qslot]
    fn add(&self, data: serde_json::Value) {
        let Some(service) = self.service.clone() else {
            return;
        };
        let url = service.borrow().url_for(&self.path);
        let headers = service.borrow().common_headers();
        let client = service.borrow().client();
        let invoker = self.get_qml_method_invoker();

        service.borrow().spawn(async move {
            let mut request = client.post(&url).json(&data);
            for (key, value) in headers {
                request = request.header(key, value);
            }
            let (_body, status) = send(request).await;
            invoke_method!(invoker, "on_mutation_finished", status);
        });
    }

    /// PUT an updated item by id, then refresh on success.
    #[qslot]
    fn update(&self, data: serde_json::Value, id: i32) {
        let Some(service) = self.service.clone() else {
            return;
        };
        let url = format!("{}/{id}", service.borrow().url_for(&self.path));
        let headers = service.borrow().common_headers();
        let client = service.borrow().client();
        let invoker = self.get_qml_method_invoker();

        service.borrow().spawn(async move {
            let mut request = client.put(&url).json(&data);
            for (key, value) in headers {
                request = request.header(key, value);
            }
            let (_body, status) = send(request).await;
            invoke_method!(invoker, "on_mutation_finished", status);
        });
    }

    /// DELETE an item by id, then refresh on success.
    #[qslot]
    fn remove(&self, id: i32) {
        let Some(service) = self.service.clone() else {
            return;
        };
        let url = format!("{}/{id}", service.borrow().url_for(&self.path));
        let headers = service.borrow().common_headers();
        let client = service.borrow().client();
        let invoker = self.get_qml_method_invoker();

        service.borrow().spawn(async move {
            let mut request = client.delete(&url);
            for (key, value) in headers {
                request = request.header(key, value);
            }
            let (_body, status) = send(request).await;
            invoke_method!(invoker, "on_mutation_finished", status);
        });
    }

    /// Main-thread continuation for a page GET.
    #[qslot]
    fn on_refresh_finished(&mut self, body: String, status: i32) {
        if !is_success(status) {
            self.error_occurred(format!("request failed with HTTP status {status}"));
            self.refresh_request_failed();
            return;
        }
        let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&body) else {
            self.error_occurred("failed to parse server response".to_string());
            self.refresh_request_failed();
            return;
        };

        self.pages = json.get("total_pages").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        if let Some(page) = json.get("page").and_then(|v| v.as_i64()) {
            self.current_page = page as i32;
        }
        self.data = match json.get_mut("data").map(serde_json::Value::take) {
            Some(serde_json::Value::Array(arr)) => arr,
            _ => Vec::default(),
        };
        self.page_updated();
        self.pages_updated();
        self.data_updated();
    }

    /// Main-thread continuation for add/update/remove: refresh on success.
    #[qslot]
    fn on_mutation_finished(&mut self, status: i32) {
        if is_success(status) {
            self.refresh_current_page();
        }
    }

    /// Inject the shared networking handle (called by `RestService`).
    pub fn set_service(&mut self, service: Rc<RefCell<Service>>) {
        self.service = Some(service);
    }

    fn refresh_request_failed(&mut self) {
        if self.current_page != 1 {
            self.set_page(1);
        } else {
            self.pages = 0;
            self.pages_updated();
            if !self.data.is_empty() {
                self.data.clear();
                self.data_updated();
            }
        }
    }
}
