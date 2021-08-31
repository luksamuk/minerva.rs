// lib.rs -- Uma parte de Minerva.rs
// Copyright (C) 2021 Lucas S. Vieira
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate bigdecimal;
#[macro_use] extern crate num_derive;
extern crate num_traits;
extern crate diesel_enum;
extern crate chrono;
extern crate comfy_table;
extern crate r2d2_redis;

pub mod db;
pub mod model;
pub mod controller;
pub mod routes;

pub mod inpututils;
