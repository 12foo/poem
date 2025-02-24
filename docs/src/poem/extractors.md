# Extractors

The extractor is used to extract something from the HTTP request.

`Poem` provides some commonly used extractors for extracting something from HTTP requests.

You can use one or more extractors as the parameters of the function, up to 16.

In the following example, the `index` function uses 3 extractors to extract the remote address, HTTP method and URI.

```rust
#[handler]
fn index(remote_addr: SocketAddr, method: Method, uri: &Uri) {}
```

# Built-in extractors

 - **Option&lt;T>**

    Extracts `T` from the incoming request, returns `None` if it
 fails.

 - **&Request**

    Extracts the `Request` from the incoming request.

 - **RemoteAddr**

   Extracts the remote peer's address [`RemoteAddr`] from request.

 - **Method**

    Extracts the `Method` from the incoming request.

 - **Version**

    Extracts the `Version` from the incoming request.

 - **&Uri**

    Extracts the `Uri` from the incoming request.

 - **&HeaderMap**

    Extracts the `HeaderMap` from the incoming request.

 - **Data&lt;&T>**

    Extracts the `Data` from the incoming request.

 - **TypedHeader&lt;T>**

    Extracts the `TypedHeader` from the incoming request.

 - **Path&lt;T>**

    Extracts the `Path` from the incoming request.

 - **Query&lt;T>**

    Extracts the `Query` from the incoming request.

 - **Form&lt;T>**

    Extracts the `Form` from the incoming request.

 - **Json&lt;T>**

    Extracts the `Json` from the incoming request.

    _This extractor will take over the requested body, so you should avoid
 using multiple extractors of this type in one handler._

 - **TempFile**

    Extracts the `TempFile` from the incoming request.

    _This extractor will take over the requested body, so you should avoid
 using multiple extractors of this type in one handler._

 - **Multipart**

    Extracts the `Multipart` from the incoming request.

    _This extractor will take over the requested body, so you should avoid
 using multiple extractors of this type in one handler._

 - **Body**

    Extracts the `Body` from the incoming request.

    _This extractor will take over the requested body, so you should avoid
 using multiple extractors of this type in one handler._

 - **String**

    Extracts the body from the incoming request and parse it into utf8 string.

    _This extractor will take over the requested body, so you should avoid
 using multiple extractors of this type in one handler._

 - **Vec&lt;u8>**

    Extracts the body from the incoming request and collect it into
 `Vec<u8>`.

    _This extractor will take over the requested body, so you should avoid
 using multiple extractors of this type in one handler._

 - **Bytes**

    Extracts the body from the incoming request and collect it into
 `Bytes`.

    _This extractor will take over the requested body, so you should avoid
 using multiple extractors of this type in one handler._

 - **WebSocket**

    Ready to accept a websocket connection.

## Handling of extractor errors

By default, the extractor will return a `400 Bad Request` when an error occurs, but sometimes you may want to change 
this behavior, so you can handle the error yourself.

In the following example, when the `Query` extractor fails, it will return a `500 Internal Server Error` response and the reason for the error.

```rust
use poem::web::Query;
use poem::error::ParseQueryError;
use poem::{IntoResponse, Response};
use poem::http::StatusCode;

#[derive(Debug, Deserialize)]
struct Params {
    name: String,
}

#[handler]
fn index(res: Result<Query<Params>, ParseQueryError>) -> Response {
    match res {
        Ok(Query(params)) => params.name.into_response(),
        Err(err) => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(err.to_string()),
    }
}
```

## Custom extractor

You can also implement your own extractor.

 The following is an example of a custom token extractor, which extracts the
 token from the `MyToken` header.
 
```rust
use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
};

use poem::{handler, route, route::get, Endpoint, Error, FromRequest, Request, RequestBody};

struct Token(String);

// Error type for Token extractor
#[derive(Debug)]
struct MissingToken;

impl Display for MissingToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "missing token")
    }
}

impl StdError for MissingToken {}

impl From<MissingToken> for Error {
    fn from(err: MissingToken) -> Self {
        Error::bad_request(err)
    }
}

// Implements a token extractor
#[poem::async_trait]
impl<'a> FromRequest<'a> for Token {
    type Error = MissingToken;

    async fn from_request(
        req: &'a Request,
        body: &mut RequestBody,
    ) -> Result<Self, Self::Error> {
        let token = req
            .headers()
            .get("MyToken")
            .and_then(|value| value.to_str().ok())
            .ok_or(MissingToken)?;
        Ok(Token(token.to_string()))
    }
}

#[handler]
async fn index(token: Token) {
    assert_eq!(token.0, "token123");
}
```