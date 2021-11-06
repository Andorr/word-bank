use tide::{Middleware, Next, Request, Response, Result};

#[derive(Default)]
pub struct Authorization {
    token: String,
}

impl Authorization {
    pub fn new(token: String) -> Self {
        Self {
            token: token,
        }
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for Authorization {

    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        let unauthorized = Ok(Response::builder(401).build());

        match req.header("authorization") {
            Some(header) => {
                let token = header.last().to_string();
                let parts: Vec<&str> = token.split(" ").collect();
                if parts.len() != 2 {
                    return unauthorized
                }
                else if parts[0] != "Bearer" {
                    return unauthorized   
                }
                else if parts[1] != self.token {
                    return unauthorized
                }
            },
            None => {
                return unauthorized
            }
        };

        Ok(next.run(req).await)
    }

}