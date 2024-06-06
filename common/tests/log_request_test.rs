use axum::http::HeaderMap;
use common::utils::log_request;
use lazy_static::lazy_static;
use std::sync::Mutex;
use tracing_subscriber::EnvFilter;

lazy_static! {
    static ref LOGS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

struct TestWriter;

impl std::io::Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let log = String::from_utf8_lossy(buf).to_string();
        LOGS.lock().unwrap().push(log);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> tracing_subscriber::fmt::writer::MakeWriter<'a> for TestWriter {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        TestWriter
    }
}

lazy_static! {
    static ref INIT: () = {
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("info"))
            .with_writer(TestWriter)
            .finish();
        tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
    };
}

#[tokio::test]
async fn test_log_request_with_test_client_header() {
    // Ensure the global subscriber is set
    lazy_static::initialize(&INIT);

    // Prepare the header map with X-Test-Client
    let mut headers = HeaderMap::new();
    headers.insert("X-Test-Client", "IntegrationTest".parse().unwrap());

    // Call the log_request function
    log_request(&headers, Some(123), "Test action");

    // Capture the logs
    let logs = LOGS.lock().unwrap().clone();

    // Check the logs contain the expected messages
    assert!(logs
        .iter()
        .any(|log| log.contains("<-- TEST -- Request from test client: \"IntegrationTest\"")));
    assert!(logs
        .iter()
        .any(|log| log.contains("<-- TEST -- Test action with id: 123")));
}

#[tokio::test]
async fn test_log_request_without_test_client_header() {
    // Ensure the global subscriber is set
    lazy_static::initialize(&INIT);

    // Prepare the header map without X-Test-Client
    let headers = HeaderMap::new();

    // Call the log_request function
    log_request(&headers, Some(456), "Another action");

    // Capture the logs
    let logs = LOGS.lock().unwrap().clone();

    // Check the logs contain the expected message
    assert!(logs
        .iter()
        .any(|log| log.contains("Another action with id: 456")));
}
