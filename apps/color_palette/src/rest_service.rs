// Copyright (C) 2023 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR BSD-3-Clause

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::{QObjectHolder, QParserStatus};

use crate::basic_login::BasicLogin;
use crate::paginated_source::PaginatedResource;

/// Networking back end shared by all resources of one `RestService`.
///
/// Shared via `Rc<RefCell<Service>>`; children hold a clone of that pointer.
pub struct Service {
    /// Base URL, e.g. `http://127.0.0.1:49425/api`.
    pub base_url: String,
    /// `token` header value, set by `BasicLogin` after a successful login.
    pub token: Option<String>,
    runtime: tokio::runtime::Runtime,
    client: reqwest::Client,
}

impl Service {
    fn new() -> Self {
        // A dedicated multi-threaded runtime drives the async HTTP requests.
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()
            .expect("failed to build Tokio runtime");

        Self {
            base_url: String::new(),
            token: None,
            runtime,
            client: reqwest::Client::new(),
        }
    }

    /// Spawn an async task on the shared runtime.
    pub fn spawn<F>(&self, future: F)
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.runtime.spawn(future);
    }

    /// A clone of the shared `reqwest` client (cheap; it is `Arc`-backed).
    pub fn client(&self) -> reqwest::Client {
        self.client.clone()
    }

    /// Build the absolute URL for a resource `path` (e.g. `"users"` -> `".../users"`).
    pub fn url_for(&self, path: &str) -> String {
        let base = self.base_url.trim_end_matches('/').to_string();
        format!("{base}/{}", path.trim_start_matches('/'))
    }

    /// The common headers to attach to every request (auth token).
    pub fn common_headers(&self) -> Vec<(String, String)> {
        let mut headers = Vec::new();
        if let Some(token) = &self.token {
            headers.push(("token".to_string(), token.clone()));
        }
        headers
    }

    /// Update the base URL.
    pub fn set_url(&mut self, url: &str) {
        self.base_url = url.to_string();
    }

    /// Set or clear the authentication token (called by `BasicLogin`).
    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token;
    }
}

/// The QML `RestService` element.
///
/// Handles all resources as children and injects a reference to itself
/// into all children when its initialisation in QML is complete.
pub struct RestService {
    /// Default property: the paginated resources declared as children in QML.
    resources: Vec<Rc<RefCell<PaginatedResource>>>,
    /// The login object. Note: In the original, this is one of the ordinary children.
    login: Rc<RefCell<BasicLogin>>,
    /// The networking back end shared with every child.
    service: Rc<RefCell<Service>>,
}

impl Default for RestService {
    fn default() -> Self {
        Self {
            resources: Vec::new(),
            // A placeholder; QML replaces it through the `login` property.
            login: BasicLogin::default_with_attached_qobject(),
            service: Rc::new(RefCell::new(Service::new())),
        }
    }
}

#[qtbridge::qobject(Base = QParserStatus)]
impl RestService {
    qproperty!("url", Read = get_url, Write = set_url, Notify = url_changed);
    qproperty!("sslSupported", Read = ssl_supported, Constant);
    qproperty!("resources", Member = resources, Default);
    qproperty!("login", Member = login);

    #[qsignal(qml_name = "urlChanged")]
    fn url_changed(&mut self);

    fn get_url(&self) -> String {
        self.service.borrow().base_url.clone()
    }

    fn set_url(&mut self, url: String) {
        if self.service.borrow().base_url == url {
            return;
        }
        self.service.borrow_mut().set_url(&url);
        self.url_changed();
    }

    fn ssl_supported(&self) -> bool {
        // `reqwest` is built with a TLS backend, so HTTPS is always available.
        true
    }
}

impl QParserStatus for RestService {
    fn component_complete(&mut self) {
        // Inject the shared networking handle into every child.
        for resource in &self.resources {
            resource.borrow_mut().set_service(Rc::clone(&self.service));
        }
        self.login.borrow_mut().set_service(Rc::clone(&self.service));
    }
}
