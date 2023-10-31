#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::Body as AxumBody,
        extract::{Path, State},
        http::Request,
        response::{IntoResponse, Response},
        routing::{get, post},
        Router,
    };
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

    use hoops_app::app::*;
    use hoops_app::fileserv::file_and_error_handler;
    use hoops_app::state::AppState;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let pool = SqlitePoolOptions::new()
        .connect("sqlite:hoops.db")
        .await
        .expect("Could not make pool.");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("could not run SQLx migrations");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
        routes: routes.clone(),
    };

    /// Provide db pool to server functions as global context
    async fn server_fn_handler(
        State(app_state): State<AppState>,
        path: Path<String>,
        headers: http::HeaderMap,
        raw_query: axum::extract::RawQuery,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        log::debug!("{:?}", path);

        leptos_axum::handle_server_fns_with_context(
            path,
            headers,
            raw_query,
            move || {
                provide_context(app_state.pool.clone());
            },
            request,
        )
        .await
    }

    /// Provide db pool to routes as global context
    async fn routes_handler(State(app_state): State<AppState>, req: Request<AxumBody>) -> Response {
        let AppState {
            leptos_options,
            pool,
            routes,
        } = app_state;
        let handler = leptos_axum::render_route_with_context(
            leptos_options.clone(),
            routes.clone(),
            move || {
                provide_context(pool.clone());
            },
            App,
        );

        handler(req).await.into_response()
    }

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
