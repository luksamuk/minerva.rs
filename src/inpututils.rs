// inpututils.rs -- Uma parte de Minerva.rs
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

use std::io::{ stdin, stdout, Write };
use bigdecimal::BigDecimal;
use std::str::FromStr;

pub fn get_input() -> String {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

pub fn get_input_opt() -> Option<String> {
    let buf = get_input();
    if buf.is_empty() {
        None
    } else {
        Some(buf)
    }
}

pub fn get_bool() -> bool {
    let input = get_input().to_uppercase();
    input == "S"
}

pub fn get_numeric() -> BigDecimal {
    BigDecimal::from_str(&get_input()).unwrap()
}

pub fn prompt(msg: &'static str) {
    print!("{}", msg);
    stdout().flush().expect("Flush no stdout");
}
