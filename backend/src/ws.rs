use actix_web::{web, HttpRequest, HttpResponse};
use actix_ws::create_ws;
use futures::stream::StreamExt;
use log::info;

pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_web::error::Error> {
    let (res, session, msg_stream) = create_ws(req, stream)?;

    actix_rt::spawn(async move {
        let mut msg_stream = msg_stream;
        
        // Send welcome message
        let _ = session.text("🚀 Connected to OpenDiff Real-time Sync");

        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                actix_ws::Message::Text(text) => {
                    info!("📨 Message: {}", text);
                    // Broadcast to all connected clients
                    let response = format!("✅ Message received: {}", text);
                    let _ = session.text(response);
                }
                actix_ws::Message::Binary(_) => {
                    let _ = session.text("Binary messages not supported");
                }
                actix_ws::Message::Close(reason) => {
                    info!("❌ WebSocket closed: {:?}", reason);
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(res)
}
