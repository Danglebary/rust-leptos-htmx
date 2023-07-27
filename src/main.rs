use poem::{ listener::TcpListener, Route, EndpointExt };
use poem_openapi::OpenApiService;
use todo::{ controller::TodosApi, db };

mod todo;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load();

    let pool = db::connect(config).await;

    let api_service = OpenApiService::new(TodosApi, "Todos", "1.0.0").server(
        "http://localhost:3000/todos"
    );
    let ui = api_service.openapi_explorer();

    let route = Route::new().nest("/todos", api_service).nest("/ui", ui).data(pool);

    poem::Server::new(TcpListener::bind("127.0.0.1:3000")).run(route).await?;

    Ok(())
}
