// Copyright (C) 2023 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR BSD-3-Clause

use std::cell::RefCell;
use std::rc::Rc;

use qtbridge::{QObjectHolder, invoke_method};
use qtbridge::qtbridge_type_lib::QString;
use crate::rest_service::Service;
use crate::utils::{is_success, send};

/// Posts credentials to the server's login endpoint and, on success, stores the
/// returned `token` in the shared [`Service`](crate::rest_service::Service) so
/// subsequent resource requests are authenticated.
#[derive(Default)]
pub struct BasicLogin {
    login_path: String,
    logout_path: String,
    /// Email of the logged-in user, empty when logged out.
    user: String,
    logged_in: bool,
    user_id: i32,
    /// Networking handle, injected by `RestService` in `componentComplete()`.
    service: Option<Rc<RefCell<Service>>>,
}

#[qtbridge::qobject]
impl BasicLogin {
    qproperty!("loginPath", Member = login_path, Notify = login_path_changed);
    qproperty!("logoutPath", Member = logout_path, Notify = logout_path_changed);
    // `user` and `loggedIn` share one change signal
    qproperty!("user", Member = user, Notify = user_changed);
    qproperty!("loggedIn", Member = logged_in, Notify = user_changed);
    qproperty!("userId", Member = user_id, Notify = user_changed);

    #[qsignal(qml_name = "userChanged")]
    fn user_changed(&mut self);

    #[qsignal]
    fn login_path_changed(&mut self);

    #[qsignal]
    fn logout_path_changed(&mut self);

    /// POST `{email, password}` to the login endpoint.
    #[qslot]
    fn login(&self, data: serde_json::Value) {
        let Some(service) = self.service.clone() else {
            return;
        };

        let email = get_string_from_map(&data, "email");
        let id = get_i32_from_map(&data, "id");
        let url = service.borrow().url_for(&self.login_path);
        let headers = service.borrow().common_headers();
        let client = service.borrow().client();
        let invoker = self.get_qml_method_invoker();
        service.borrow().spawn(async move {
            let mut request = client.post(&url).json(&data);
            for (key, value) in headers {
                request = request.header(key, value);
            }
            let (body, status) = send(request).await;
            invoke_method!(invoker, "on_login_finished", QString::from(email), id, QString::from(body), status);
        });
    }

    /// POST to the logout endpoint.
    #[qslot]
    fn logout(&self) {
        let Some(service) = self.service.as_ref() else {
            return;
        };
        let service = service.borrow();
        let url = service.url_for(&self.logout_path);
        let headers = service.common_headers();
        let client = service.client();
        let invoker = self.get_qml_method_invoker();

        service.spawn(async move {
            let mut request = client.post(&url).body(String::new());
            for (key, value) in headers {
                request = request.header(key, value);
            }
            let (_body, status) = send(request).await;
            invoke_method!(invoker, "on_logout_finished", status);
        });
    }

    /// Main-thread continuation for `login`: record the user on success.
    #[qslot]
    fn on_login_finished(&mut self, email: String, id: i32, body: String, status: i32) {
        let token = is_success(status)
            .then(|| serde_json::from_str::<serde_json::Value>(&body).ok())
            .flatten()
            .and_then(|json| {
                json.get("token")?
                    .as_str()
                    .map(<_>::to_owned)
            });

        if let Some(token) = token {
            self.user = email;
            self.user_id = id;
            self.logged_in = true;
            if let Some(service) = &self.service {
                service.borrow_mut().set_token(Some(token));
            }
        } else {
            self.user.clear();
            self.user_id = 0;
            self.logged_in = false;
        }
        self.user_changed();
    }

    /// Main-thread continuation for `logout`: clear the user on success.
    #[qslot]
    fn on_logout_finished(&mut self, status: i32) {
        if is_success(status) {
            self.user.clear();
            self.user_id = 0;
            self.logged_in = false;
            if let Some(service) = &self.service {
                service.borrow_mut().set_token(None);
            }
            self.user_changed();
        }
    }

    pub fn set_service(&mut self, service: Rc<RefCell<Service>>) {
        self.service = Some(service);
    }
}

fn get_string_from_map(v: &serde_json::Value, key: &str) -> String {
    let Some(map) = v.as_object() else { return String::new() };
    map[key].as_str().unwrap().to_owned()
}

fn get_i32_from_map(v: &serde_json::Value, key: &str) -> i32 {
    let Some(map) = v.as_object() else { return 0 };
    map[key].as_f64().map(|n| n as i32).unwrap()
}
