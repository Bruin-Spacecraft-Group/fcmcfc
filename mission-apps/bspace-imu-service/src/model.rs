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


use juniper::*;

/// Common response fields structure for requests
/// which don't return any specific data
#[derive(GraphQLObject)]
pub struct GenericResponse {
    /// Any errors encountered by the request
    pub errors: String,
    /// Request completion success or failure
    pub success: bool,
}

/// Return field for 'ack' query
///
/// Indicates last mutation executed by the service
#[derive(GraphQLEnum, Clone, Copy)]
pub enum AckCommand {
    /// No mutations have been executed
    None,
    /// No-Op
    Noop,
    /// GPS time and/or rv values were updated
    Update,
}

/// RV input fields for `update` mutation
#[derive(GraphQLInputObject)]
pub struct RVInput {
    /// X, Y, Z ECI position values
    pub eci_pos: Vec<f64>,
    /// X, Y, Z ECI velocity values
    pub eci_vel: Vec<f64>,
    /// GPS time at Epoch
    pub time_epoch: i32,
}


#[derive(GraphQLObject)]
pub struct IMUData {
    /// X-axis
    pub x: f64,
    /// Y-axis
    pub y: f64,
    /// Z-axis
    pub z: f64,
}

graphql_object!(IMUData: () where Scalar = <S> |&self| {
    field accel() -> Vec<i32> {
        self.0.accel.iter().map(|&elem| i32::from(elem)).collect()
    }

    field gyro() -> Vec<i32> {
        self.0.gyro.iter().map(|&elem| i32::from(elem)).collect()
    }

    field gyro_temp() -> i32 {
        i32::from(self.0.gyro_temp)
    }
});
