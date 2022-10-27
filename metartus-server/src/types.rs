use std::sync::Arc;

use tokio::sync::{Notify, OnceCell};

use crate::artus::{ArtusCommand, ArtusResponse};

pub type ArtusRequest = (Arc<Notify>, ArtusCommand, Arc<OnceCell<ArtusResponse>>);