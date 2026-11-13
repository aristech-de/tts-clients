//! Connect timeout behaviour, checked against local servers so the tests need no network.

use std::time::{Duration, Instant};

use aristech_tts_client::{
    get_client, get_client_with_options, ConnectOptions, ConnectTimeoutError, TlsOptions,
};
use tokio::net::{TcpListener, TcpStream};

const TIMEOUT: Duration = Duration::from_millis(300);
/// Generous upper bound for "returned promptly" so slow CI runners don't flake.
const PROMPTLY: Duration = Duration::from_secs(3);

/// A server that accepts TCP connections and then never sends a byte, like an overloaded or
/// misconfigured server. A TLS handshake never completes against it.
async fn silent_server() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        let mut held: Vec<TcpStream> = Vec::new();
        while let Ok((socket, _)) = listener.accept().await {
            held.push(socket);
        }
    });
    port
}

/// A plaintext HTTP/2 server that completes the handshake and then keeps the connection open.
async fn http2_server() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        while let Ok((socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut connection = h2::server::handshake(socket).await.unwrap();
                while connection.accept().await.is_some() {}
            });
        }
    });
    port
}

/// A port nothing listens on, so connecting is refused right away.
async fn closed_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    listener.local_addr().unwrap().port()
}

fn timeout_options() -> ConnectOptions {
    ConnectOptions::new().connect_timeout(TIMEOUT)
}

fn assert_connect_timeout(err: &(dyn std::error::Error + 'static), host: &str) {
    let timeout = err
        .downcast_ref::<ConnectTimeoutError>()
        .unwrap_or_else(|| panic!("expected a ConnectTimeoutError, got: {err}"));
    assert_eq!(timeout.host, host);
    assert_eq!(timeout.timeout, TIMEOUT);
    assert_eq!(
        err.to_string(),
        format!("connect to {host}: timed out after 300 ms")
    );
}

#[tokio::test]
async fn stalled_tls_handshake_times_out() {
    let host = format!("https://127.0.0.1:{}", silent_server().await);
    let started = Instant::now();
    let Err(err) =
        get_client_with_options(host.clone(), Some(TlsOptions::default()), timeout_options()).await
    else {
        panic!("connecting to a silent server must fail");
    };
    let elapsed = started.elapsed();
    assert_connect_timeout(err.as_ref(), &host);
    assert!(
        elapsed >= TIMEOUT,
        "gave up before the timeout: {elapsed:?}"
    );
    assert!(elapsed < PROMPTLY, "took too long: {elapsed:?}");
}

#[tokio::test]
async fn host_without_scheme_reports_the_normalized_host() {
    let port = silent_server().await;
    let Err(err) = get_client_with_options(
        format!("127.0.0.1:{port}"),
        Some(TlsOptions::default()),
        timeout_options(),
    )
    .await
    else {
        panic!("connecting to a silent server must fail");
    };
    assert_connect_timeout(err.as_ref(), &format!("https://127.0.0.1:{port}"));
}

#[tokio::test]
async fn connects_within_the_timeout() {
    let host = format!("http://127.0.0.1:{}", http2_server().await);
    get_client_with_options(host, None, timeout_options())
        .await
        .expect("a responsive server must connect within the timeout");
}

#[tokio::test]
async fn refused_connection_fails_fast_with_a_connect_error() {
    let host = format!("http://127.0.0.1:{}", closed_port().await);
    let options = ConnectOptions::new().connect_timeout(Duration::from_secs(30));
    let started = Instant::now();
    let Err(err) = get_client_with_options(host.clone(), None, options).await else {
        panic!("connecting to a closed port must fail");
    };
    assert!(
        err.downcast_ref::<ConnectTimeoutError>().is_none(),
        "a refused connection is not a timeout: {err}"
    );
    assert!(err.to_string().starts_with(&format!("connect to {host}: ")));
    assert!(started.elapsed() < PROMPTLY);
}

#[tokio::test]
async fn get_client_keeps_waiting_without_a_timeout() {
    let host = format!("https://127.0.0.1:{}", silent_server().await);
    let outcome = tokio::time::timeout(
        Duration::from_secs(1),
        get_client(host, Some(TlsOptions::default())),
    )
    .await;
    assert!(
        outcome.is_err(),
        "get_client has no connect timeout and must still be waiting"
    );
}

#[tokio::test]
async fn default_options_have_no_timeout() {
    assert_eq!(ConnectOptions::new().connect_timeout, None);
    assert_eq!(ConnectOptions::default().connect_timeout, None);
}
