#!/usr/bin/env python3
"""
Post-processing script to autogenerate Rust integration tests from openapi.yaml.
- Parses OpenAPI spec
- Generates #[tokio::test] functions for each operation
- Writes to tests/integration.rs
"""

import os
import sys
import yaml
from jinja2 import Template

OPENAPI_PATH = "openapi.yaml"
OUTPUT_PATH = "tests/integration.rs"

TEST_TEMPLATE = """
//! Autogenerated integration tests for the Cerebras Rust SDK
//! DO NOT EDIT MANUALLY. This file is generated by scripts/generate_integration_tests.py

use cerebras_rs::prelude::*;

{% for op in operations %}
#[tokio::test]
async fn test_{{ op.operation_id }}() {
    let client = Client::from_env().expect("API key not set");
    // TODO: Fill in example data for parameters
    {%- for param in op.parameters %}
    // let {{ param.name }}: {{ param.rust_type }} = unimplemented!();
    {%- endfor %}
    let result = client.{{ op.operation_id }}(
        {%- for param in op.parameters %}{{ param.name }}, {% endfor -%}
    ).await;
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
    // Optionally, assert on response fields
}
{% endfor %}
"""

def rust_type_from_schema(schema):
    """Map OpenAPI schema types to Rust types (very basic)."""
    t = schema.get("type", "string")
    if t == "integer":
        return "i64"
    if t == "number":
        return "f64"
    if t == "boolean":
        return "bool"
    if t == "array":
        return f"Vec<{rust_type_from_schema(schema.get('items', {}))}>"
    return "String"

def main():
    with open(OPENAPI_PATH, "r") as f:
        spec = yaml.safe_load(f)

    operations = []
    for path, methods in spec.get("paths", {}).items():
        for method, op in methods.items():
            operation_id = op.get("operationId")
            if not operation_id:
                continue
            parameters = []
            for param in op.get("parameters", []):
                name = param["name"]
                schema = param.get("schema", {})
                rust_type = rust_type_from_schema(schema)
                parameters.append({"name": name, "rust_type": rust_type})
            # TODO: handle requestBody parameters if present
            operations.append({
                "operation_id": operation_id,
                "parameters": parameters,
            })

    template = Template(TEST_TEMPLATE)
    rendered = template.render(operations=operations)

    os.makedirs(os.path.dirname(OUTPUT_PATH), exist_ok=True)
    with open(OUTPUT_PATH, "w") as f:
        f.write(rendered.strip() + "\n")

    print(f"Generated {OUTPUT_PATH} with {len(operations)} test(s).")

if __name__ == "__main__":
    main()
