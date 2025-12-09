use actix_web::{web, App, HttpResponse, HttpServer};

use std::net::TcpListener;
use serde::Deserialize;

pub mod routes;
pub mod configuration;
pub mod startup;


