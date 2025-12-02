use salvo::prelude::*;

pub mod custom_middleware_example;
pub mod jwt;
pub use jwt::auth_hoop;
mod cors;
pub use cors::cors_hoop;

#[handler]
pub async fn error_404(&self, res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        // let handle404 = Error404 {
        //     brief: if let ResBody::Error(e) = &res.body {
        //         e.brief.clone()
        //     } else {
        //         "Page not found".to_owned()
        //     },
        // };
        // res.render(JSON(""));
        ctrl.skip_rest();
    }
}
