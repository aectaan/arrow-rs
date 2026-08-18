// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Generates the Rust bindings for the Arrow Flight protobuf definitions.

use std::path::Path;

use buffa_codegen::BytesRepr;
use connectrpc_build::CodeGenConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = Path::new("../format");
    let proto_path = Path::new("../format/Flight.proto");

    let mut buffa_config = CodeGenConfig::default();
    buffa_config.file_per_package = true;
    buffa_config.preserve_unknown_fields = false;
    buffa_config.idiomatic_imports = false;
    buffa_config.generate_json = true;
    buffa_config.bytes_fields = vec![(".arrow".to_string(), BytesRepr::Bytes)];
    buffa_config.enum_attributes = vec![(
        ".".to_string(),
        "#[allow(non_camel_case_types)]".to_string(),
    )];

    connectrpc_build::Config::new()
        .out_dir("src")
        .file_per_package(true)
        .generate_json(true)
        .includes(&[proto_dir])
        .files(&[proto_path])
        .buffa_config(buffa_config)
        .compile()?;

    let proto_dir = Path::new("../format");
    let proto_path = Path::new("../format/FlightSql.proto");

    let mut buffa_config = CodeGenConfig::default();
    buffa_config.file_per_package = true;
    buffa_config.generate_json = true;
    buffa_config.preserve_unknown_fields = false;
    buffa_config.enum_attributes = vec![(
        ".".to_string(),
        "#[allow(non_camel_case_types)]".to_string(),
    )];

    connectrpc_build::Config::new()
        .out_dir("src/sql")
        .file_per_package(true)
        .generate_json(true)
        .includes(&[proto_dir])
        .files(&[proto_path])
        .buffa_config(buffa_config)
        .compile()?;

    // As the proto file is checked in, the build should not fail if the file is not found
    Ok(())
}
