use poem::{listener::TcpListener, route};
use poem_openapi::{payload::Json, Object, OneOf, OpenApi, OpenApiService};

#[derive(Object, Debug, PartialEq)]
struct A {
    v1: i32,
    v2: String,
}

#[derive(Object, Debug, PartialEq)]
struct B {
    v3: f32,
}

#[derive(OneOf, Debug, PartialEq)]
#[oai(property_name = "type")]
enum MyObj {
    A(A),
    B(B),
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/put", method = "post")]
    async fn index(&self, obj: Json<MyObj>) -> Json<MyObj> {
        obj
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:3000");
    let api_service = OpenApiService::new(Api)
        .title("Oneof")
        .server("http://localhost:3000/api");
    let ui = api_service.swagger_ui("http://localhost:3000");

    poem::Server::new(listener)
        .await?
        .run(route().nest("/api", api_service).nest("/", ui))
        .await
}
