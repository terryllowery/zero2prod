use actix_web::{App, HttpResponse, HttpServer, web};

use serde::Deserialize;
use std::net::TcpListener;

pub mod configuration;
pub mod routes;
pub mod startup;
