use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::extract::FromRef;
        use leptos::LeptosOptions;
        use leptos_router::RouteListing;
        use sqlx::SqlitePool;

        #[derive(FromRef, Debug, Clone)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub pool: SqlitePool,
            pub routes: Vec<RouteListing>,
        }
    }
}
