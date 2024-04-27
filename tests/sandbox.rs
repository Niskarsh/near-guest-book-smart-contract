// use backend_rs::Message;
use near_sdk::{near, AccountId, NearToken};
use std::error::Error;
// macro allowing us to convert args into JSON bytes to be read by the contract.
use serde_json::{json, Map, Value};

async fn prepare_dev_env(
) -> Result<(Vec<near_workspaces::Account>, near_workspaces::Contract), Box<dyn Error>> {
    let worker = near_workspaces::sandbox().await?;
    let wasm = near_workspaces::compile_project("./").await?;
    let contract = worker.dev_deploy(&wasm).await?;
    // Call init method
    let _ = contract
        .call("init")
        .args_json(json!({}))
        .transact()
        .await?;

    let account = worker.dev_create_account().await?;
    let account1 = worker.dev_create_account().await?;
    let account2 = worker.dev_create_account().await?;
    Ok((vec![account, account1, account2], contract))
}

#[tokio::test]
async fn returns_all_messages() -> Result<(), Box<dyn Error>> {
    let (account, contract) = prepare_dev_env().await?;
    // println!("{:?}, {:?}", account, contract);
    let _ = account[0]
        .call(contract.id(), "add_message")
        .args_json(json!({ "message": "Hi there "}))
        .transact()
        .await?;

    let recieved_messages: serde_json::Value = account[0]
        .view(contract.id(), "get_messages")
        .args_json(json!({ "offset": 0, "limit": 10 }))
        .await?
        .json()?;
    let dummy = vec![json!({ "id": "123"})];

    assert_ne!(recieved_messages.as_array().unwrap_or(&vec![]).len(), 0);

    let messages_extracted = recieved_messages.as_array().unwrap_or(&dummy)[0].as_object();

    match messages_extracted {
        None => {}
        Some(object) => {
            assert_eq!(object["id"], account[0].id().to_string());
        }
    };

    // Check if messages are lost once retrieved
    let recieved_messages: serde_json::Value = account[0]
        .view(contract.id(), "get_messages")
        .args_json(json!({ "offset": 0, "limit": 10 }))
        .await?
        .json()?;
    let dummy = vec![json!({ "id": "123"})];

    assert_ne!(recieved_messages.as_array().unwrap_or(&vec![]).len(), 0);

    let messages_extracted = recieved_messages.as_array().unwrap_or(&dummy)[0].as_object();

    match messages_extracted {
        None => {}
        Some(object) => {
            assert_eq!(object["id"], account[0].id().to_string());
        }
    };
    Ok(())
}

#[tokio::test]
async fn verify_highest_donation() -> Result<(), Box<dyn Error>> {
    let (account, contract) = prepare_dev_env().await?;
    let _ = account[0]
        .call(contract.id(), "add_message")
        .args_json(json!({ "message": "Hi there "}))
        .transact()
        .await?;

    let _ = account[1]
        .call(contract.id(), "add_message")
        .args_json(json!({ "message": "Hi there. I am rich "}))
        .deposit(NearToken::from_near(20))
        .transact()
        .await?;

    let _ = account[2]
        .call(contract.id(), "add_message")
        .args_json(json!({ "message": "Hi there. I am richer "}))
        .deposit(NearToken::from_near(50))
        .transact()
        .await?;

    let recieved_messages: serde_json::Value = account[0]
        .view(contract.id(), "get_messages")
        .args_json(json!({ "offset": 0, "limit": 10 }))
        .await?
        .json()?;

    // println!("{:?}", recieved_messages);

    assert_ne!(recieved_messages.as_array().unwrap_or(&vec![]).len(), 0);

    let highest_donation = account[0]
        .view(contract.id(), "highest_donation")
        .args_json(json!({}))
        .await?
        .json::<NearToken>()?;

    assert_eq!(highest_donation, NearToken::from_near(50));

    let premium_messages = contract
        .view("get_premium_messages")
        .args_json(json!({ "offset": 0, "limit": 10 }))
        .await?
        // .json()?;
        .json::<Vec<Map<String, Value>>>()?;

    assert_ne!(premium_messages.len(), 0);
    assert_eq!(premium_messages.len(), 2);

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn reinitiate_contract_with_non_admin_id() -> () {
    let (account, contract) = prepare_dev_env().await.unwrap();
    let _ = account[0]
        .call(contract.id(), "init")
        .args_json(json!({}))
        .transact()
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn get_account_wise_msg() -> Result<(), Box<dyn Error>> {
    let (account, contract) = prepare_dev_env().await?;
    // println!("{:?}", contract);
    let _ = account[0]
        .call(contract.id(), "add_message")
        .args_json(json!({ "message": "Hi there "}))
        .transact()
        .await?;

    let _ = account[1]
        .call(contract.id(), "add_message")
        .args_json(json!({ "message": "Hi there. I am rich "}))
        .deposit(NearToken::from_near(20))
        .transact()
        .await?;

    let _ = account[2]
        .call(contract.id(), "add_message")
        .args_json(json!({ "message": "Hi there. I am richer "}))
        .deposit(NearToken::from_near(50))
        .transact()
        .await?;

    let recieved_messages = account[2]
        .call(contract.id(), "messages_by_signed_in_user")
        .args_json(json!({}))
        .transact()
        .await?
        .json::<Vec<Map<String, Value>>>()?;

    assert_eq!(recieved_messages[0]["id"], account[2].id().to_string());

    let recieved_messages = account[0]
        .view(contract.id(), "get_messages")
        .args_json(json!({}))
        // .transact()
        .await?
        .json::<Vec<Map<String, Value>>>()?;

    assert_eq!(recieved_messages.len(), 3);

    Ok(())
}

#[tokio::test]
async fn cmp_msg() -> Result<(), Box<dyn Error>> {
    #[near(serializers = [borsh, json])]
    #[derive(Debug, Clone, PartialEq)]
    // #[borsh(crate = "near_sdk::borsh")]
    pub struct Message {
        id: AccountId,
        premium_attached: Option<NearToken>,
        message: String,
    }
    let (account, contract) = prepare_dev_env().await?;
    let _ = account[0]
        .call(contract.id(), "add_message")
        .args_json(json!({ "message": "Hi there "}))
        .transact()
        .await?;
    let recieved_messages = account[2]
        .view(contract.id(), "get_messages")
        .args_json(json!({}))
        // .transact()
        .await?
        .json::<Vec<Message>>()?;

    assert_eq!(
        recieved_messages,
        vec![Message {
            id: account[0].id().clone(),
            message: "Hi there ".to_string(),
            premium_attached: None,
        }]
    );

    let _ = account[1]
        .call(contract.id(), "add_message")
        .args_json(json!({ "message": "Hi there. I am rich "}))
        .deposit(NearToken::from_near(20))
        .transact()
        .await?;
    let recieved_messages = account[2]
        .view(contract.id(), "get_messages")
        .args_json(json!({}))
        // .transact()
        .await?
        .json::<Vec<Message>>()?;

    assert_eq!(
        recieved_messages,
        vec![
            Message {
                id: account[0].id().clone(),
                message: "Hi there ".to_string(),
                premium_attached: None,
            },
            Message {
                id: account[1].id().clone(),
                message: "Hi there. I am rich ".to_string(),
                premium_attached: Some(NearToken::from_near(20)),
            }
        ]
    );
    let _ = account[2]
        .call(contract.id(), "add_message")
        .args_json(json!({ "message": "Hi there. I am richer "}))
        .deposit(NearToken::from_near(50))
        .transact()
        .await?;
    let recieved_messages = account[2]
        .view(contract.id(), "get_messages")
        .args_json(json!({}))
        // .transact()
        .await?
        .json::<Vec<Message>>()?;

    assert_eq!(
        recieved_messages,
        vec![
            Message {
                id: account[0].id().clone(),
                message: "Hi there ".to_string(),
                premium_attached: None,
            },
            Message {
                id: account[1].id().clone(),
                message: "Hi there. I am rich ".to_string(),
                premium_attached: Some(NearToken::from_near(20)),
            },
            Message {
                id: account[2].id().clone(),
                message: "Hi there. I am richer ".to_string(),
                premium_attached: Some(NearToken::from_near(50)),
            }
        ]
    );

    Ok(())
}
