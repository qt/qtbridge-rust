// Copyright (C) 2023 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR BSD-3-Clause

/// Returns true for a 2xx HTTP status.
pub fn is_success(status: i32) -> bool {
    (200..300).contains(&status)
}

/// Send a request, returning `(body, status)`. A transport error yields `("", 0)`.
pub async fn send(request: reqwest::RequestBuilder) -> (String, i32) {
    match request.send().await {
        Ok(response) => {
            let status = response.status().as_u16() as i32;
            let body = response.text().await.unwrap_or_default();
            (body, status)
        }
        Err(_) => (String::new(), 0),
    }
}
