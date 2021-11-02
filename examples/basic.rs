use axum::{extract::Path, Router};
use axum_resource::Resource;
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(
            Resource::named("users")
                // GET /users
                .index(|| async { "users#index" })
                // POST /users
                .create(|| async { "users#create" })
                // GET /users/new
                .new(|| async { "users#new" })
                // GET /users/:id
                .show(|| async { "users#show" })
                // GET /users/:id/edit
                .edit(|| async { "users#edit" })
                // PATCH or PUT /users/:id
                .update(|| async { "users#update" })
                // DELETE /users/:id
                .destroy(|| async { "users#destroy" })
                // nest another resources inside /users/:id
                .nest(
                    Resource::named("tweets")
                        // GET /users/:user_id/tweets/:tweet_id
                        .show(|Path(params): Path<UserTweetIds>| async move {
                            dbg!(params);
                            "user_tweets#show"
                        })
                        .into_router(),
                )
                // nest another resources inside /users
                .nest_collection(
                    Resource::named("things")
                        // GET /users/things
                        .index(|| async { "users_things#index" })
                        .into_router(),
                )
                .into_router(),
        )
        .merge(
            // can also define partial resources
            Resource::named("tweets")
                // GET /tweets
                .index(|| async { "tweets#index" })
                .into_router(),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Debug)]
struct UserTweetIds {
    users_id: u64,
    tweets_id: u64,
}
