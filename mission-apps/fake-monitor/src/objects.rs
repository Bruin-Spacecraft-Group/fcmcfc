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
use crate::meminfo::MemInfo;

pub struct MemInfoResponse {
    pub info: MemInfo,
}

graphql_object!(MemInfoResponse: () |&self| {
    field total() -> Option<i32> {
        self.info.total().map(|v| v as i32)
    }

    field free() -> Option<i32> {
        self.info.free().map(|v| v as i32)
    }

    field available() -> Option<i32> {
        self.info.available().map(|v| v as i32)
    }

    field low_free() -> Option<i32> {
        self.info.low_free().map(|v| v as i32)
    }
});
