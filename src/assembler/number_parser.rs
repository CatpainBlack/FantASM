/*
Copyright (c) 2019, Guy Black
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

The views and conclusions contained in the software and documentation are those
of the authors and should not be interpreted as representing official policies,
either expressed or implied, of the FantASM project.
*/

use regex::Regex;

lazy_static! {
static ref HEX: Regex = Regex::new(r"^0(?P<h1>[0-9a-fA-F]+)[hH]$|0x(?P<h2>[0-9a-fA-F]+)$|^\$(?P<h3>[0-9a-fA-F]+)$").unwrap();
static ref BIN: Regex = Regex::new(r"^(?P<b1>[0-1]+)b$|^(0b|%)(?P<b2>[0-1]+)$").unwrap();
static ref DEC: Regex = Regex::new(r"^\d+$").unwrap();
}

pub trait NumberParser {
    fn to_number(&self) -> Option<i64>;
}

impl NumberParser for String {
    fn to_number(&self) -> Option<i64> {
        if let Some(captures) = HEX.captures(&self) {
            for name in HEX.capture_names() {
                if let Some(n) = name {
                    if let Some(val) = captures.name(n) {
                        return Some(i64::from_str_radix(val.as_str(), 16).unwrap());
                    }
                }
            }
        }

        if let Some(captures) = BIN.captures(&self) {
            for name in BIN.capture_names() {
                if let Some(n) = name {
                    if let Some(val) = captures.name(n) {
                        return Some(i64::from_str_radix(val.as_str(), 2).unwrap());
                    }
                }
            }
        }

        if DEC.is_match(&self) {
            return Some(i64::from_str_radix(&self, 10).unwrap());
        }
        None
    }
}