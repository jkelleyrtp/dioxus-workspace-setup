use dioxus::prelude::*;

#[server]
pub async fn read_user() -> Result<(), ServerFnError> {
    todo!()
}

#[server]
pub async fn change_username(new_username: String) -> Result<(), ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(())
}
