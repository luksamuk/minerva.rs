// util.rs -- Uma parte de Minerva.rs
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

use diesel::pg::data_types::PgNumeric;
use serde::Serialize;

#[derive(Serialize)]
#[serde(remote = "PgNumeric")]
pub enum Numeric {
    Positive {
        weight: i16,
        scale: u16,
        digits: Vec<i16>
    },
    Negative {
        weight: i16,
        scale: u16,
        digits: Vec<i16>
    },
    NaN
}

fn into_chunks<'a>(integral_part: &'a str, decimal_part: &'a str) -> (Vec<i16>, Vec<i16>) {
    let mut partsz: i32 = decimal_part.len() as i32;
    let mut index: i32 = 0;
    let mut decimal_chunks = Vec::new();
    while index < partsz {
        let chunk = if (index + 4) > partsz {
            let index = index as usize;
            &decimal_part[index..]
        } else {
            let index = index as usize;
            &decimal_part[index..index + 4]
        };
        let chunk = format!("{:0<4}", chunk);
        decimal_chunks.push(chunk.parse::<i16>().unwrap());
        index = index + 4;
    }
    partsz = integral_part.len() as i32;
    index = partsz;
    let mut integral_chunks = Vec::new();
    while index > 0 {
        let chunk = if (index - 4) < 0 {
            let index = index as usize;
            &integral_part[..index]
        } else {
            let index = index as usize;
            &integral_part[index-4..index]
        };
        let chunk = format!("{:0>4}", chunk);
        integral_chunks.push(chunk.parse::<i16>().unwrap());
        index = index - 4;
    }
    let integral_chunks = integral_chunks.into_iter().rev().collect();
    (integral_chunks, decimal_chunks)
}

pub fn string_to_numeric(s: String) -> PgNumeric {
    let s = s.trim();
    
    let dot_pos = match s.find('.') {
        None => s.len(),
        Some(n) => n,
    };
    
    let is_negative = &s[..1] == "-";
    
    let integral_part =
        if is_negative || (&s[..1] == "+") {
            &s[1..dot_pos]
        } else {
            &s[0..dot_pos]
        };
    
    let decimal_part =
        if dot_pos == s.len() {
            "0"
        } else {
            &s[dot_pos+1..]
        };

    let (mut ichunks, mut dchunks) = into_chunks(integral_part, decimal_part);
    let weight = (ichunks.len() - 1) as i16;
    let scale  = decimal_part.len() as u16;
    ichunks.append(&mut dchunks);
    
    if is_negative {
        PgNumeric::Negative {
            weight: weight,
            scale: scale,
            digits: ichunks,
        }
    } else {
        PgNumeric::Positive {
            weight: weight,
            scale: scale,
            digits: ichunks,
        }
    }
}

pub fn numeric_to_string(num: PgNumeric) -> String {
    if let PgNumeric::NaN = num {
        return String::from("NaN");
    }
    
    let weight = match num {
        PgNumeric::Positive { weight, scale: _, digits: _ } => weight,
        PgNumeric::Negative { weight, scale: _, digits: _ } => weight,
        PgNumeric::NaN => panic!("convertendo NaN em String"),
    } as usize;
    let digits = match num {
        PgNumeric::Positive { weight: _, scale: _, ref digits } => digits.clone(),
        PgNumeric::Negative { weight: _, scale: _, ref digits } => digits.clone(),
        PgNumeric::NaN => panic!("convertendo NaN em String"),
    };
    let is_negative = match num {
        PgNumeric::Negative{ weight: _,scale: _, digits: _ }  => true,
        PgNumeric::Positive{ weight: _,scale: _, digits: _ } => false,
        PgNumeric::NaN => panic!("convertendo NaN em String"),
    };

    let mut result = String::new();
    if is_negative { result.push('-'); }

    if digits.len() == 0 {
        result.push_str("0.0");
    } else {
        let integral_parts = &digits[..weight+1];
        let fraction_parts = &digits[weight+1..];
        
        if integral_parts.len() == 0 {
            result.push('0');
        } else {
            for part in integral_parts {
                result.push_str(&format!("{}", part));
            }
        }

        result.push('.');

        if fraction_parts.len() == 0 {
            result.push('0');
        } else {
            for part in fraction_parts {
                result.push_str(&format!("{}", part));
            }
        }
    }
    // Remove extra zeroes at end

    result
}
