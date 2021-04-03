//
// Copyright (C) 2018 Kubos Corporation
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use failure;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub struct MemInfo {
    total: Option<u32>,
    free: Option<u32>,
    available: Option<u32>,
    low_free: Option<u32>,
}

impl MemInfo {
    /// Create an empty MemInfo object
    pub fn new() -> Self {
        Self {
            total: None,
            free: None,
            available: None,
            low_free: None,
        }
    }

    fn parse_amount(amount: &str) -> Option<u32> {
        let mut iter = amount.split_whitespace();
        match iter.next() {
            Some(amount) => match u32::from_str(amount) {
                Ok(amount) => Some(amount),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn parse<R>(info: R) -> Result<MemInfo, failure::Error>
    where
        R: BufRead,
    {
        let mut mem_info = MemInfo::new();

        for line in info.lines() {
            let line = line?;
            let mut iter = line.split_whitespace();
            let key = iter.next();
            let val = iter.next();

            if let (Some(key), Some(val)) = (key, val) {
                match key.get(0..key.len() - 1).unwrap_or("") {
                    "MemTotal" => mem_info.total = Self::parse_amount(val),
                    "MemFree" => mem_info.free = Self::parse_amount(val),
                    "MemAvailable" => mem_info.available = Self::parse_amount(val),
                    "LowFree" => mem_info.low_free = Self::parse_amount(val),
                    _ => {}
                }
            }
        }
        Ok(mem_info)
    }

    pub fn from_proc() -> Result<MemInfo, failure::Error> {
        let file = File::open("/proc/meminfo")?;
        let reader = BufReader::new(file);
        Self::parse(reader)
    }

    /// Total system memory available in kB
    pub fn total(&self) -> Option<u32> {
        self.total
    }
    /// Total system memory free in kB
    pub fn free(&self) -> Option<u32> {
        self.free
    }
    /// Total system memory available in kB
    pub fn available(&self) -> Option<u32> {
        self.available
    }
    /// The low mark for system memory free in kB
    pub fn low_free(&self) -> Option<u32> {
        self.low_free
    }
}
