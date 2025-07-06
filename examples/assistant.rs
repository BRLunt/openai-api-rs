use openai_api_rs::v1::api::OpenAIClient;
use openai_api_rs::v1::assistant::AssistantRequest;
use openai_api_rs::v1::common::GPT4_O;
use openai_api_rs::v1::message::{CreateMessageRequest, MessageRole};
use openai_api_rs::v1::run::CreateRunRequest;
use openai_api_rs::v1::thread::CreateThreadRequest;
use openai_api_rs::v1::types::{Function, JSONSchemaType, Tool, Tools, ToolsType};
use serde_json::json;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_API_KEY").unwrap().to_string();
    let mut client = OpenAIClient::builder().with_api_key(api_key).build()?;

    // Define the properties for the JSON schema
    let p_concept_id_schema = openai_api_rs::v1::types::JSONSchemaDefine {
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
        parameters: openai_api_rs::v1::types::FunctionParameters {
            schema_type: JSONSchemaType::Object,
            properties: Some(props),
            required: None,
        },
    };

    let tools = Tool {
        r#type: ToolsType::Function,
        function: Some(f),
        name: None, // must be none, if we have a function
    };

    let req = AssistantRequest::new(GPT4_O.to_string()).tools(vec![tools]);

    let req = req
        .clone()
        .description("this is a test assistant".to_string());

    let req = req.clone().instructions("You are a personal math tutor. When asked a question, answer with a function call, or with an explanation what to do next.".to_string());
    println!(
        "AssistantRequest: {:?}",
        serde_json::to_string(&req).unwrap()
    );

    let result = client.create_assistant(req).await?;
    println!("Create Assistant Result ID: {:?}", result.id);

    let thread_req = CreateThreadRequest::new();
    let thread_result = client.create_thread(thread_req).await?;
    println!("Create Thread Result ID: {:?}", thread_result.id.clone());

    let message_req = CreateMessageRequest::new(
        MessageRole::user,
        "I need to learn concept 1 and concept 2. I already know concept one. What items should I learn?".to_string(),
    );

    let message_result = client
        .create_message(thread_result.id.clone(), message_req)
        .await?;
    println!("Create Message Result: {:?}", message_result);

    let run_req = CreateRunRequest::new(result.id);
    let run_result = client.create_run(thread_result.id.clone(), run_req).await?;
    println!("Create Run Result ID: {:?}", run_result.id.clone());

    loop {
        let run_result = client
            .retrieve_run(thread_result.id.clone(), run_result.id.clone())
            .await
            .unwrap();
        if run_result.status == "completed" {
            break;
        } else if run_result.status == "requires_action" {
            println!("Run requires action, waiting for next iteration...");
            println!("Run result: {:?}", run_result);
            // check which function was requested and do the action
            if let Tools::Function (t) = run_result.tools.first().unwrap() {
                if t.function.name == "recommend_learning_item" {
                    println!("Tool call: {:?}", t);
                    // Here you would implement the logic to handle the function call,
                    // For example, you could call a function to recommend learning items
                    // based on the provided concept_ids.

                    let uuid_1 = "3e8f8c0-1b2c-4d3e-8f8c-0a1b2c3d4e5f";
                    let uuid_2 = "4d3e8f8c-0a1b-2c3d-4e5f-6a7b8c9d0e1f";
                    let uuid_3 = "5f6a7b8c-9d0e-1f2a-3b4c-5d6e7f8g9h0i";

                    let item_meta_1 = json!({"uuid": &uuid_1, "concept_id": 1, "difficulty": "easy", "interactive": true, "language": "en"});
                    let item_meta_2 = json!({"concept_id": &uuid_2, "difficulty": "medium", "interactive": true, "language": "en"});
                    let item_meta_3 = json!({"concept_id": &uuid_3, "difficulty": "hard", "interactive": true, "language": "en"});

                    let list_of_items = vec![item_meta_1, item_meta_2, item_meta_3];

                    // Simulate a response to the function call
                    let response = json!({
                        "concept_ids": list_of_items
                    });

                    // Send the response back to the assistant and run
                    let response_message = CreateMessageRequest::new(
                            MessageRole::function,
                            format!(
                                "I recommend the following learning items: {:?}\nPlease select one item for the learner and return the uuid to finish the run.",
                                response
                            ),
                        );
                    loop {
                        let run_result = client
                            .retrieve_run(thread_result.id.clone(), run_result.id.clone())
                            .await
                            .unwrap();
                        if run_result.status == "completed" {
                            println!("Run completed: {:?}", run_result);
                            println!("Run result: {:?}", run_result);
                            break;
                        } else if run_result.status == "requires_action" {
                            println!("Run requires action, waiting for next iteration...");
                            println!("Run result: {:?}", run_result);
                            break;
                        } else if run_result.status == "failed" {
                            println!("Run failed: {:?}", run_result);
                            break;
                        } else {
                            println!("Run status: {:?}", run_result.status);
                        }
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    }

                    let _ = client
                        .create_message(thread_result.id.clone(), response_message)
                        .await
                        .unwrap();
                }
            }
            break;
        } else if run_result.status == "failed" {
            println!("Run failed: {:?}", run_result);
            break;
        } else {
            println!("Run status: {:?}", run_result.status);
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }

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
    Ok(())
}

// OPENAI_API_KEY=xxxx cargo run --package openai-api-rs --example assistant
