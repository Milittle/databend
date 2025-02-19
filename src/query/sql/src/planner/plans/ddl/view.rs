// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::sync::Arc;

use common_expression::DataSchema;
use common_expression::DataSchemaRef;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateViewPlan {
    pub if_not_exists: bool,
    pub tenant: String,
    pub catalog: String,
    pub database: String,
    pub viewname: String,
    pub column_names: Vec<String>,
    pub subquery: String,
}

impl CreateViewPlan {
    pub fn schema(&self) -> DataSchemaRef {
        Arc::new(DataSchema::empty())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlterViewPlan {
    pub tenant: String,
    pub catalog: String,
    pub database: String,
    pub viewname: String,
    pub column_names: Vec<String>,
    pub subquery: String,
}

impl AlterViewPlan {
    pub fn schema(&self) -> DataSchemaRef {
        Arc::new(DataSchema::empty())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropViewPlan {
    pub if_exists: bool,
    pub tenant: String,
    pub catalog: String,
    pub database: String,
    pub viewname: String,
}

impl DropViewPlan {
    pub fn schema(&self) -> DataSchemaRef {
        Arc::new(DataSchema::empty())
    }
}
