use super::thread::CreateThreadRequest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::impl_builder_methods;
use crate::v1::types::Tools;

#[derive(Debug, Serialize, Clone)]
pub struct CreateRunRequest {
    assistant_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<HashMap<String, String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>, // 1: json!("auto"), 2: json!({"type": "json_object"})
}

impl CreateRunRequest {
    pub fn new(assistant_id: String) -> Self {
        Self {
            assistant_id,
            model: None,
            instructions: None,
            tools: None,
            metadata: None,
            response_format: None,
        }
    }
}

impl_builder_methods!(
    CreateRunRequest,
    model: String,
    instructions: String,
    tools: Vec<HashMap<String, String>>,
    metadata: HashMap<String, String>,
    response_format: Value
);

#[derive(Debug, Serialize, Clone)]
pub struct ModifyRunRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl ModifyRunRequest {
    pub fn new() -> Self {
        Self { metadata: None }
    }
}

impl Default for ModifyRunRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl_builder_methods!(
    ModifyRunRequest,
    metadata: HashMap<String, String>
);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LastError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FunctionCall {
    pub name: String,
    // The arguments are a string that contains JSON, so String is the correct type.
    pub arguments: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ToolCall {
    pub id: String,
    pub r#type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubmitToolOutputs {
    pub tool_calls: Vec<ToolCall>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequiredAction {
    pub r#type: String,
    pub submit_tool_outputs: SubmitToolOutputs,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RunObject {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub thread_id: String,
    pub assistant_id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_action: Option<RequiredAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<LastError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    pub model: String,
    pub instructions: Option<String>,

    // Use the new Tool struct
    pub tools: Vec<Tools>, //

    pub metadata: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_prompt_tokens: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncation_strategy: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<Value>,
    #[serde(default)]
    pub parallel_tool_calls: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListRun {
    pub object: String,
    pub data: Vec<RunObject>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
}

#[derive(Debug, Serialize, Clone)]
pub struct CreateThreadAndRunRequest {
    pub assistant_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread: Option<CreateThreadRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<HashMap<String, String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RunStepObject {
    pub id: String,
    pub object: String,
    pub created_at: i64,
    pub assistant_id: String,
    pub thread_id: String,
    pub run_id: String,
    #[serde(rename = "type")]
    pub run_step_type: String,
    pub status: String,
    pub step_details: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<LastError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ListRunStep {
    pub object: String,
    pub data: Vec<RunStepObject>,
    pub first_id: String,
    pub last_id: String,
    pub has_more: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::v1::types::JSONSchemaType;
    use serde_json::json;

    #[test]
    fn test_parse_complex_entity_payload() {
        let input_payload = json!({
           "id": "run_abc123def456",
           "object": "entity.process",
           "created_at": 1609459200,
           "assistant_id": "asst_xyz789uvw123",
           "thread_id": "thread_ghi456jkl789",
           "status": "pending_input",
           "started_at": 1609459201,
           "expires_at": 1609459800,
           "cancelled_at": null,
           "failed_at": null,
           "completed_at": null,
           "required_action": {
             "type": "submit_tool_outputs",
             "submit_tool_outputs": {
               "tool_calls": [
                 {
                   "id": "call_pqr321stu654",
                   "type": "function",
                   "function": {
                     "name": "process_data_item",
                     "arguments": "{\"property_alpha\":\"value_beta\"}"
                   }
                 }
               ]
             }
           },
           "last_error": null,
           "model": "model-vNext",
           "instructions": "This is a generic instruction set for the processing agent.",
           "tools": [
             {
               "type": "function",
               "function": {
                 "name": "process_data_item",
                 "description": "A generic description of what this tool does.",
                 "parameters": {
                   "type": "object",
                   "properties": {
                     "property_alpha": {
                       "type": "string",
                       "description": "A description for property alpha."
                     }
                   }
                 },
                 "strict": false
               }
             }
           ],
           "tool_resources": {},
           "metadata": {},
           "temperature": 1.0,
           "top_p": 1.0,
           "reasoning_effort": null,
           "max_completion_tokens": null,
           "max_prompt_tokens": null,
           "truncation_strategy": {
             "type": "auto",
             "last_messages": null
           },
           "incomplete_details": null,
           "usage": null,
           "response_format": "auto",
           "tool_choice": "auto",
           "parallel_tool_calls": true
    });

        let parsed_entity: RunObject = serde_json::from_value(input_payload).unwrap();

        // top-level
        assert_eq!(parsed_entity.id, "run_abc123def456");
        assert_eq!(parsed_entity.object, "entity.process");
        assert_eq!(parsed_entity.status, "pending_input");
        assert_eq!(parsed_entity.assistant_id, "asst_xyz789uvw123");
        assert_eq!(parsed_entity.thread_id, "thread_ghi456jkl789");
        assert_eq!(parsed_entity.model, "model-vNext");
        assert_eq!(parsed_entity.instructions, Some("This is a generic instruction set for the processing agent.".to_string()));

        // tools
        assert_eq!(parsed_entity.tools.len(), 1);
        assert!(matches!(parsed_entity.tools[0], Tools::Function { .. }));
        if let Tools::Function (function ) = &parsed_entity.tools[0] {
            assert_eq!(function.function.name, "process_data_item");
            assert_eq!(function.function.description, Some("A generic description of what this tool does.".to_string()));
            assert_eq!(function.function.parameters.schema_type, JSONSchemaType::Object);
            assert!(function.function.parameters.properties.is_some());
            if let Some(properties) = &function.function.parameters.properties {
                assert!(properties.contains_key("property_alpha"));
                if let Some(prop) = properties.get("property_alpha") {
                    assert_eq!(prop.description, Some("A description for property alpha.".to_string()));
                    assert_eq!(prop.schema_type, Some(JSONSchemaType::String));
                }
            }
        }

        // required action
        assert!(parsed_entity.required_action.is_some());
        if let Some(required_action) = &parsed_entity.required_action {
            assert_eq!(required_action.r#type, "submit_tool_outputs");
            assert_eq!(required_action.submit_tool_outputs.tool_calls.len(), 1);
            let tool_call = &required_action.submit_tool_outputs.tool_calls[0];
            assert_eq!(tool_call.id, "call_pqr321stu654");
            assert_eq!(tool_call.r#type, "function");
            assert_eq!(tool_call.function.name, "process_data_item");
            assert_eq!(tool_call.function.arguments, "{\"property_alpha\":\"value_beta\"}");
        }

        // metadata and other misc fields
        assert!(parsed_entity.metadata.is_empty());
        assert_eq!(parsed_entity.temperature, Some(1.0));
        assert_eq!(parsed_entity.top_p, Some(1.0));
        assert_eq!(parsed_entity.parallel_tool_calls, true);
        assert_eq!(parsed_entity.response_format, Some(Value::String("auto".to_string())));
        assert_eq!(parsed_entity.tool_choice, Some(Value::String("auto".to_string())));
        assert_eq!(parsed_entity.truncation_strategy, Some(json!({"type": "auto", "last_messages": null})));

        // some misc nullable fields
        assert!(parsed_entity.incomplete_details.is_none());
        assert!(parsed_entity.usage.is_none());
        assert!(parsed_entity.reasoning_effort.is_none());
        assert!(parsed_entity.max_completion_tokens.is_none());
        assert!(parsed_entity.max_prompt_tokens.is_none());
        assert!(parsed_entity.expires_at.is_some());
        assert!(parsed_entity.started_at.is_some());
        assert!(parsed_entity.cancelled_at.is_none());
        assert!(parsed_entity.failed_at.is_none());
        assert!(parsed_entity.completed_at.is_none());
        assert!(parsed_entity.last_error.is_none());
    }
}