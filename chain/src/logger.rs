use std::net::SocketAddr;
use std::time::Instant;

use jsonrpsee::server::logger::{self, HttpRequest, MethodKind, Params, TransportProtocol};

#[derive(Clone)]
pub(crate) struct Logger;

impl logger::Logger for Logger {
    type Instant = Instant;

    fn on_connect(&self, remote_addr: SocketAddr, request: &HttpRequest, _t: TransportProtocol) {
        tracing::info!(
            "[Logger::on_connect] remote_addr {:?}, headers: {:?}",
            remote_addr,
            request
        );
    }

    fn on_request(&self, _t: TransportProtocol) -> Self::Instant {
        tracing::info!("[Logger::on_request]");
        Instant::now()
    }

    fn on_call(&self, name: &str, params: Params, kind: MethodKind, _t: TransportProtocol) {
        tracing::info!(
            "[Logger::on_call] method: '{}', params: {:?}, kind: {}",
            name,
            params,
            kind
        );
    }

    fn on_result(
        &self,
        name: &str,
        succeess: bool,
        started_at: Self::Instant,
        _t: TransportProtocol,
    ) {
        tracing::info!(
            "[Logger::on_result] '{}', worked? {}, time elapsed {:?}",
            name,
            succeess,
            started_at.elapsed()
        );
    }

    fn on_response(&self, result: &str, started_at: Self::Instant, _t: TransportProtocol) {
        tracing::info!(
            "[Logger::on_response] result: {}, time elapsed {:?}",
            result,
            started_at.elapsed()
        );
    }

    fn on_disconnect(&self, remote_addr: SocketAddr, _t: TransportProtocol) {
        tracing::info!("[Logger::on_disconnect] remote_addr: {:?}", remote_addr);
    }
}
