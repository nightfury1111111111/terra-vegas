use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use terra_vegas_ico::msg::{
    ConfigResponse, DepositResponse, ExecuteMsg, InstantiateMsg, LaunchConfig, QueryMsg,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(LaunchConfig), &out_dir);
    export_schema(&schema_for!(DepositResponse), &out_dir);
    export_schema(&schema_for!(ConfigResponse), &out_dir);
}
