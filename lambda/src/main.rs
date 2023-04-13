use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};
use simple_error::SimpleError;
use tracing::{info, instrument};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let event_format = fmt::format::json();
    tracing_subscriber::fmt().event_format(event_format).init();
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

// use instrument attribute for async fn
// https://github.com/tokio-rs/tracing#in-asynchronous-code
#[instrument]
async fn handler(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();
    let message = event["message"].as_str();
    match message {
        Some(message) => {
            info!("message: {message} in event");
            Ok(json!({ "message": format!("Hello, {}!", message) }))
        }
        None => {
            info!("no message provided");
            Err(Box::new(SimpleError::new("No message provided")))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::{HeaderMap, HeaderValue};
    use lambda_runtime::Context;

    #[tokio::test]
    async fn handler_with_msg() -> Result<(), Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "lambda-runtime-aws-request-id",
            HeaderValue::from_static("my-id"),
        );
        headers.insert(
            "lambda-runtime-deadline-ms",
            HeaderValue::from_static("123"),
        );
        let context = Context::try_from(headers).unwrap();
        let event = json!({
            "message": "AWS Lambda on Rust"
        });
        let expected = json!({
            "message": "Hello, AWS Lambda on Rust!",
        });
        let response = handler(LambdaEvent::new(event, context)).await?;
        assert_eq!(response, expected);
        Ok(())
    }
    #[tokio::test]
    async fn handler_without_msg() -> Result<(), Error> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "lambda-runtime-aws-request-id",
            HeaderValue::from_static("my-id"),
        );
        headers.insert(
            "lambda-runtime-deadline-ms",
            HeaderValue::from_static("123"),
        );
        let context = Context::try_from(headers).unwrap();
        let event = json!({
            "someKey": "someValue"
        });
        let response = handler(LambdaEvent::new(event, context)).await;
        if let Err(e) = response {
            assert_eq!(e.to_string(), "No message provided");
        }
        Ok(())
    }
}
