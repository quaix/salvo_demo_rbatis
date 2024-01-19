use salvo::{handler, prelude::StatusCode, writing::Json, Depot, FlowCtrl, Request, Response};

#[handler]
pub async fn handle_404(&self, _req: &Request, _depot: &Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::NOT_FOUND) = res.status_code {
        res.render(Json("404 not found"));
        ctrl.skip_rest();
    }
}
