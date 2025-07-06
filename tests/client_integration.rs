use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::assistant::AssistantRequest;
use openai_api_rs::v1::common::GPT4_O;
use openai_api_rs::v1::completion::CompletionResponse;
use openai_api_rs::v1::message::{CreateMessageRequest, MessageRole};
use openai_api_rs::v1::run::CreateRunRequest;
use openai_api_rs::v1::thread::CreateThreadRequest;
use openai_api_rs::v1::types::{Function, JSONSchemaType, Tool,  ToolsType};
use serde_json::json;
use std::collections::HashMap;
use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};
use openai_api_rs::v1::types;


#[tokio::test]
async fn test_completion_returns_expected_text() {
    // Start a mock server
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "cmpl-12345",
            "object": "text_completion",
            "created": 1633072800,
            "model": "text-davinci-003",
            "choices": [
                {
                    "text": "Bitcoin is a decentralized digital currency.",
                    "index": 0,
                    "logprobs": null,
                    "finish_reason": "stop"
                }
            ],
            "usage": {
                "prompt_tokens": 10,
                "completion_tokens": 20,
                "total_tokens": 30
            }
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client: OpenAIClient = OpenAIClient::builder()
        .with_endpoint(mock_server.uri())
        .with_api_key("test_api_key")
        .build()
        .unwrap();

    let response = client
        .completion(
            openai_api_rs::v1::completion::CompletionRequest::new(
                openai_api_rs::v1::completion::GPT3_TEXT_DAVINCI_003.to_string(),
                "What is Bitcoin?".to_string(),
            )
            .max_tokens(100),
        )
        .await;
    assert!(response.is_ok());

    let result: CompletionResponse = response.unwrap();
    assert_eq!(result.id, "cmpl-12345");
    assert_eq!(result.model, "text-davinci-003");
    assert_eq!(
        result.choices[0].text.trim(),
        "Bitcoin is a decentralized digital currency."
    );
}

#[tokio::test]
async fn test_assistant_creation() {
    // Start a mock server
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/assistants"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!(
            {
              "id": "asst_abc123",
              "object": "assistant",
              "created_at": 1698984975,
              "name": "Math Tutor",
              "description": null,
              "model": "gpt-4o",
              "instructions": "You are a personal math tutor. When asked a question, write and run Python code to answer the question.",
              "tools": [
                {
                  "type": "code_interpreter"
                }
              ],
              "metadata": {},
              "top_p": 1.0,
              "temperature": 1.0,
              "response_format": "auto"
            }
        )))
        .expect(1)
        .mount(&mock_server)
        .await;

    let mut client = OpenAIClient::builder()
        .with_endpoint(mock_server.uri())
        .with_api_key("test_api_key")
        .build()
        .unwrap();

    let f = Function {
        name: "recommend_learning_item".to_string(),
        description: Some("Recommend a learning item based on the user's input.".to_string()),
        parameters: types::FunctionParameters {
            schema_type: JSONSchemaType::Object,
            properties: None,
            required: None,
        },
    };

    let t = Tool {
        r#type: ToolsType::Function,
        function: Some(f.clone()),
        name: None,
    };

    let expected_obj = json!({
        "type": "function",
        "function": {
            "name": f.name,
            "description": f.description,
            "parameters": {
                "type": f.parameters.schema_type,
            }
        }
    });

    let actual_json: serde_json::Value =
        serde_json::to_value(&t).expect("Failed to serialize Function tool to Value");
    assert_eq!(
        actual_json, expected_obj,
        "Serialized tool does not match expected API format"
    );

    let assistant_request =
        AssistantRequest::new("gpt-3.5-turbo".to_string())
            .name("Test Assistant".to_string())
            .description("An assistant for testing purposes.".to_string())
            .tools(vec![t.clone()]);

    client
        .create_assistant(assistant_request)
        .await
        .expect("Failed to create assistant");
}

fn get_assistant_response() -> serde_json::Value {
    json!({
      "id": "run_OwFdkkd1doRRmAwsMnNeQ7v9",
      "object": "thread.run",
      "created_at": 1750600575,
      "assistant_id": "asst_PaLpDNhIqYE6gIUe91HxyqSv",
      "thread_id": "thread_itUn0qiqmeYIx67iKEQZgDbY",
      "status": "requires_action",
      "started_at": 1750600576,
      "expires_at": 1750601175,
      "cancelled_at": null,
      "failed_at": null,
      "completed_at": null,
      "required_action": {
        "type": "submit_tool_outputs",
        "submit_tool_outputs": {
          "tool_calls": [
            {
              "id": "call_DlFYouRdPHX6T2fYDbXVeYOJ",
              "type": "function",
              "function": {
                "name": "recommend_learning_item",
                "arguments": "{\"first_prop\":\"concept 2\"}"
              }
            }
          ]
        }
      },
      "last_error": null,
      "model": "gpt-4o",
      "instructions": "You are a personal math tutor. When asked a question, answer with a function call, or with an explanation what to do next.",
      "tools": [
        {
          "type": "function",
          "function": {
            "name": "recommend_learning_item",
            "description": "Recommend a learning item based on the user's input.",
            "parameters": {
              "type": "object",
              "properties": {
                "first_prop": {
                  "type": "string",
                  "description": "The concept_id of the recommended learning item."
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
    })
}

#[tokio::test]
pub async fn test_assistant_creation_w_function() {
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/assistants"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_assistant_response()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = OpenAIClient::builder()
        .with_endpoint(mock_server.uri())
        .with_api_key("test_api_key")
        .build();

    assert!(
        client.is_ok(),
        "Failed to create OpenAIClient: {:?}",
        client.err()
    );

    let mut client = client.unwrap();
    let f = Function {
        name: "recommend_learning_item".to_string(),
        description: Some("Recommend a learning item based on the user's input.".to_string()),
        parameters: types::FunctionParameters {
            schema_type: JSONSchemaType::Object,
            properties: None,
            required: None,
        },
    };

    let t = Tool {
        r#type: ToolsType::Function,
        function: Some(f.clone()),
        name: None,
    };

    let assistant_request =
        AssistantRequest::new("gpt-4o".to_string())
            .name("Test Assistant with Function".to_string())
            .description("An assistant for testing purposes with function.".to_string())
            .tools(vec![t.clone()]);

    let response = client.create_assistant(assistant_request).await;
    assert!(
        response.is_ok(),
        "Failed to create assistant: {:?}",
        response.err()
    );
    let result = response.unwrap();

    assert_eq!(
        result.id, "run_OwFdkkd1doRRmAwsMnNeQ7v9",
        "Unexpected assistant ID"
    );
    assert_eq!(
        result.model, "gpt-4o",
        "Unexpected model in assistant response"
    );
    assert_eq!(result.instructions,
               Some("You are a personal math tutor. When asked a question, answer with a function call, or with an explanation what to do next.".to_string()),
               "Unexpected instructions in assistant response");

    assert!(
        !result.tools.is_empty(),
        "Expected at least one tool in assistant response"
    );
    let tool_obj = &result.tools.first().unwrap().clone();

    // check if the tool is of type Function
    if let types::Tools::Function (function , .. ) = tool_obj {

        assert_eq!(
            function.function.name, "recommend_learning_item".to_string(),
            "Unexpected function name in tool"
        );
        assert_eq!(
            function.function.description.as_deref(),
            Some("Recommend a learning item based on the user's input."),
            "Unexpected function description in tool"
        );
        assert_eq!(
            function.function.parameters.schema_type,
            JSONSchemaType::Object,
            "Unexpected schema type in function parameters"
        );
        // assert!(function.parameters.properties.is_none(), "Expected no properties in function parameters");
        assert!(
            function.function.parameters.required.is_none(),
            "Expected no required fields in function parameters"
        );
    } else {
        panic!(
            "Expected tool to be of type Function, but got {:?}",
            tool_obj
        );
    }
}

pub async fn test_assistant_run() {
    // mock assistant creation
    let mock_server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/assistants"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_assistant_response()))
        .expect(1)
        .mount(&mock_server)
        .await;

    //mock the thread creation
    Mock::given(method("POST"))
        .and(path("/threads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": "thread_itUn0qiqmeYIx67iKEQZgDbY",
            "object": "thread",
            "created_at": 1750600575,
            "assistant_id": "asst_PaLpDNhIqYE6gIUe91HxyqSv",
            "name": null,
            "description": null,
            "metadata": {},
            "model": "gpt-4o",
            "instructions": "You are a personal math tutor. When asked a question, answer with a function call, or with an explanation what to do next.",
            "tools": [
                {
                    "type": "function",
                    "function": {
                        "name": "recommend_learning_item",
                        "description": "Recommend a learning item based on the user's input.",
                        "parameters": {
                            "type": "object",
                            "properties": {
                                "first_prop": {
                                    "type": "string",
                                    "description": "The concept_id of the recommended learning item."
                                }
                            }
                        },
                        "strict": false
                    }
                }
            ],
            "tool_resources": {},
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
        })))
        .expect(1)
        .mount(&mock_server)
        .await;

    // mock the run creation
    Mock::given(method("POST"))
        .and(path("/assistants/run"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_assistant_response()))
        .expect(1)
        .mount(&mock_server)
        .await;

    // Mock the run status
    Mock::given(method("GET"))
        .and(path("/assistants/run/run_OwFdkkd1doRRmAwsMnNeQ7v9"))
        .respond_with(ResponseTemplate::new(200).set_body_json(get_assistant_response()))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = OpenAIClient::builder()
        .with_endpoint(mock_server.uri())
        .with_api_key("test_api_key")
        .build();

    assert!(
        client.is_ok(),
        "Failed to create OpenAIClient: {:?}",
        client.err()
    );
    let mut client = client.unwrap();

    // Now build up the assistant request
    let p_concept_id_schema = types::JSONSchemaDefine {
        schema_type: Some(JSONSchemaType::String),
        description: Some("The concept_id of the recommended learning item.".to_string()),
        ..Default::default()
    };

    let props = HashMap::from([(
        "concept_ids".to_string(),
        Box::new(p_concept_id_schema.clone()),
    )]);

    let f = Function {
        name: "recommend_learning_item".to_string(),
        description: Some("Recommend a learning item based on the user's input.".to_string()),
        parameters: types::FunctionParameters {
            schema_type: JSONSchemaType::Object,
            properties: Some(props),
            required: None,
        },
    };

    let tools = Tool {
        r#type: ToolsType::Function,
        function: Some(f),
        name: None, // must be none if we have a function
    };

    let req = AssistantRequest::new(GPT4_O.to_string()).tools(vec![tools]);

    let req = req
        .clone()
        .description("this is a test assistant".to_string());

    let req = req.clone().instructions("You are a personal math tutor. When asked a question, answer with a function call, or with an explanation what to do next.".to_string());
    assert!(
        serde_json::to_string(&req).is_ok(),
        "Failed to serialize AssistantRequest to JSON"
    );

    let result = client
        .create_assistant(req)
        .await
        .expect("Failed to create assistant");
    assert_eq!(
        result.id, "run_OwFdkkd1doRRmAwsMnNeQ7v9",
        "Unexpected assistant ID"
    );

    let thread_req = CreateThreadRequest::new();
    let thread_result = client
        .create_thread(thread_req)
        .await
        .expect("Failed to create thread");
    assert_eq!(
        thread_result.id, "thread_itUn0qiqmeYIx67iKEQZgDbY",
        "Unexpected thread ID"
    );

    let message_req = CreateMessageRequest::new(
        MessageRole::user,
        "I need to learn concept 1 and concept 2. I already know concept one. What items should I learn?".to_string(),
    );

    let message_result = client
        .create_message(thread_result.id.clone(), message_req)
        .await
        .expect("Failed to create message");
    assert_eq!(
        message_result.id, "call_DlFYouRdPHX6T2fYDbXVeYOJ",
        "Unexpected message ID"
    );

    let run_req = CreateRunRequest::new(result.id);
    let run_result = client
        .create_run(thread_result.id.clone(), run_req)
        .await
        .expect("Failed to create run");
    assert_eq!(
        run_result.id, "run_OwFdkkd1doRRmAwsMnNeQ7v9",
        "Unexpected run ID"
    );

    let mut is_received = false;
    for _ in 0..2 {
        let run_result = client
            .retrieve_run(thread_result.id.clone(), run_result.id.clone())
            .await
            .unwrap();
        if run_result.status == "completed" {
            is_received = true;
            break;
        } else {
            println!("waiting...");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }

    assert!(is_received, "Run did not complete in time");

    let list_message_result = client
        .list_messages(thread_result.id.clone())
        .await
        .unwrap();
    for data in list_message_result.data {
        for content in data.content {
            println!(
                "{:?}: {:?} {:?}",
                data.role, content.text.value, content.text.annotations
            );
        }
    }
}
