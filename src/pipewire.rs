use ashpd::{
    desktop::screencast::{Screencast, CursorMode, SourceType, PersistMode}, 
    WindowIdentifier,
};

pub async fn get_capture_node_id() -> ashpd::Result<u32> {
    let proxy = Screencast::new().await?;
    let session = proxy.create_session().await?;
    proxy
        .select_sources(
            &session, 
            CursorMode::Embedded, 
            SourceType::Monitor | SourceType::Window, 
            false, 
            None, 
            PersistMode::Application
        )
        .await?;

    let response = proxy
        .start(&session, &WindowIdentifier::default())
        .await?
        .response()?;

    let node_id = response.streams().first().unwrap().pipe_wire_node_id();
    Ok(node_id)
}
