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

use super::model::*;
use juniper::*;
use kubos_service;

type Context = kubos_service::Context<Subsystem>;

pub struct QueryRoot;

// Base GraphQL query model
graphql_object!(QueryRoot: Context as "Query" |&self| {

    // Test query to verify service is running without attempting
    // to communicate with the underlying subsystem
    //
    // {
    //     ping: "pong"
    // }
    field ping() -> FieldResult<String>
    {
        Ok(String::from("pong"))
    }

    //----- Generic Queries -----//

    // Get the last run mutation
    //
    // {
    //     ack: AckCommand
    // }
    field ack(&executor) -> FieldResult<AckCommand>
    {
        let last_cmd = executor.context().subsystem().last_cmd.read()?;
        Ok(*last_cmd)
    }

    // Get all errors encountered since the last time this field was queried
    //
    // {
    //     errors: [String]
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
    {
        executor.context().subsystem().get_read_health();

        match executor.context().subsystem().errors.write() {
            Ok(mut master_vec) => {
                let current = master_vec.clone();
                master_vec.clear();
                master_vec.shrink_to_fit();
                Ok(current)
            },
            _ => Ok(vec!["Error: Failed to borrow master errors vector".to_owned()])
        }
    }

    // Get the current configuration of the system
    //
    // {
    //     config: "Not Implemented"
    // }
    field config(&executor) -> FieldResult<String>
    {
        Ok(String::from("Not Implemented"))
    }


    // Get the test results of the last run test
    //
    // Note: For this service, this actually just fetches the nominal
    // and debug telemetry of the system, since there is no actual
    // built-in test
    //
    // {
    //     testResults{
    //         success,
    //         telemetryNominal{...},
    //         telemetryDebug{...}
    //     }
    // }
    field test_results(&executor) -> FieldResult<IntegrationTestResults> {
        Ok(executor.context().subsystem().get_test_results()?)
    }

    // Get the current mode of the system
    //
    // {
    //     mode: Mode
    // }
    field mode(&executor) -> FieldResult<Mode> {
        Ok(executor.context().subsystem().get_mode()?)
    }

    // Get the last reported orientation of the system
    //
    // {
    //     orientation: "Not Implemented"
    // }
    field orientation(&executor) -> FieldResult<String> {
        Ok(String::from("Not Implemented"))
    }

    // Get the last reported spin values of the system
    // Note: The spin values are automatically updated every six seconds
    //
    // {
    //     spin{
    //         x: f64,
    //         y: f64,
    //         z: f64
    //     }
    // }
    field spin(&executor) -> FieldResult<Spin> {
        Ok(executor.context().subsystem().get_spin()?)
    }
});

pub struct MutationRoot;

// Base GraphQL mutation model
graphql_object!(MutationRoot: Context as "Mutation" |&self| {

    // Get all errors encountered while processing this GraphQL request
    //
    // Note: This will only return errors thrown by fields which have
    // already been processed, so it is recommended that this field be specified last.
    //
    // mutation {
    //     errors: [String]
    // }
    field errors(&executor) -> FieldResult<Vec<String>>
    {
        match executor.context().subsystem().errors.read() {
            Ok(master_vec) => Ok(master_vec.clone()),
            _ => Ok(vec!["Error: Failed to borrow master errors vector".to_owned()])
        }
    }

    // Execute a trivial command against the system
    //
    // mutation {
    //     noop {
    //         errors: String,
    //         success: Boolean
    //    }
    // }
    field noop(&executor) -> FieldResult<GenericResponse>
    {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::Noop;
        Ok(executor.context().subsystem().noop()?)
    }


    // Update system values
    //
    // gpsTime: Optional. If specified, updates the system's ADACS clock
    // rv: Optional. If specified, updates the orbital position and velocity at epoch.
    //     The argument has the following sub-fields:
    //         - eciPos: Vector containing the new X, Y, and Z ECI position values
    //         - eciVel: Vector containing the new X, Y, and Z ECI velocity values
    //         - timeEpoch: GPS time at which the eciPos and eciVel values will go into effect
    //
    // mutation {
    //     update(gps_time: Option<i32>,
    //         rv: Option<{eciPos: [f64; 3], eciVel: [f64; 3], timeEpoch: i32}>) {
    //         errors: String,
    //         success: Boolean,
    //     }
    // }
    field update(&executor, gps_time: Option<i32>, rv: Option<RVInput>)
    -> FieldResult<GenericResponse> {
        let mut last_cmd = executor.context().subsystem().last_cmd.write()?;
        *last_cmd = AckCommand::Update;
        Ok(executor.context().subsystem().update(gps_time, rv)?)
    }

});
