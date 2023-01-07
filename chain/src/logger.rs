use std::net::SocketAddr;
use std::time::Instant;

use jsonrpsee::core::middleware::{self, Headers, MethodKind, Params};

#[derive(Clone)]
pub(crate) struct Logger;

impl middleware::HttpMiddleware for Logger {
    type Instant = Instant;

    fn on_request(&self, remote_addr: SocketAddr, headers: &Headers) -> Self::Instant {
        tracing::info!(
            "[Middleware::on_request] remote_addr {}, headers: {:?}",
            remote_addr,
            headers
        );
        Instant::now()
    }

    fn on_call(&self, name: &str, params: Params, kind: MethodKind) {
        tracing::info!(
            "[Middleware::on_call] method: '{}', params: {:?}, kind: {}",
            name,
            params,
            kind
        );
    }

    fn on_result(&self, name: &str, succeess: bool, started_at: Self::Instant) {
        tracing::info!(
            "[Middleware::on_result] '{}', worked? {}, time elapsed {:?}",
            name,
            succeess,
            started_at.elapsed()
        );
    }

    fn on_response(&self, result: &str, started_at: Self::Instant) {
        tracing::info!(
            "[Middleware::on_response] result: {}, time elapsed {:?}",
            result,
            started_at.elapsed()
        );
    }
}
