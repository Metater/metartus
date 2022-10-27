mod artus;
mod types;

use std::sync::Arc;

use artus::{Artus, ArtusResponse};
use axum::{Router, routing::get, Extension};
use tokio::{sync::{Notify, OnceCell}, join, time::Instant};

use crate::{artus::ArtusCommand, types::ArtusRequest};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let (tx, rx) = flume::unbounded::<ArtusRequest>();

    let artus = Artus::new(rx);

    let app = Router::new()
        .route("/", get(index))
        .layer(Extension(Arc::new(tx)));

    let server_task = axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service());

    let artus_task = artus.start();

    let tasks = join!(server_task, artus_task);
    tasks.0.unwrap();
    tasks.1.unwrap();
}

async fn index(db: Extension<Arc<flume::Sender<ArtusRequest>>>) -> String {
    let notifier = Arc::new(Notify::new());
    let response = Arc::new(OnceCell::new());
    db.0.send((notifier.clone(), ArtusCommand::Index, response.clone())).unwrap();
    let i = Instant::now();
    notifier.notified().await;
    //response.get().unwrap()
    format!("{}", i.elapsed().as_secs_f64())
}