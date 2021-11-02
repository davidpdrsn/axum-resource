use axum::{extract::Path, Router};
use axum_resource::{Resource, ResourceBuilder, RouterExt};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .resource(users_resource())
        .resource(teams_resource());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn users_resource() -> impl Resource {
    ResourceBuilder::named("users")
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
            ResourceBuilder::named("tweets")
                // GET /users/:user_id/tweets/:tweet_id
                .show(|Path(params): Path<UserTweetIds>| async move {
                    dbg!(params);
                    "user_tweets#show"
                })
                .into_router(),
        )
        // nest another resources inside /users
        .nest_collection(
            ResourceBuilder::named("things")
                // GET /users/things
                .index(|| async { "users_things#index" })
                .into_router(),
        )
}

#[derive(Deserialize, Debug)]
struct UserTweetIds {
    users_id: u64,
    tweets_id: u64,
}

fn teams_resource() -> impl Resource {
    // can also define partial resources
    ResourceBuilder::named("tweets")
        // GET /tweets
        .index(|| async { "tweets#index" })
}
