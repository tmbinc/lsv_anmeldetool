use crate::api::auth::LoggedUser;
use actix_web::{
    rt::time::interval,
    web::{self, Path},
    Responder,
};
use actix_web_lab::sse;
use apistos::{api_operation, ApiComponent};
use futures_util::future;
use parking_lot::Mutex;
use schemars::JsonSchema;
use serde::Serialize;
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

pub struct Broadcaster {
    inner: Mutex<BroadcasterInner>,
}

#[derive(Debug, Clone, Default)]
struct BroadcasterInner {
    clients: Vec<tokio::sync::mpsc::Sender<sse::Event>>,
}

impl Broadcaster {
    /// Constructs new broadcaster and spawns ping loop.
    pub fn create() -> Arc<Self> {
        let this = Arc::new(Broadcaster {
            inner: Mutex::new(BroadcasterInner::default()),
        });
        Broadcaster::spawn_ping(Arc::clone(&this));
        // println!("created success");

        this
    }

    /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast list if not.
    fn spawn_ping(this: Arc<Self>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                this.remove_stale_clients().await;
            }
        });
    }

    /// Removes all non-responsive clients from broadcast list.
    async fn remove_stale_clients(&self) {
        let clients = self.inner.lock().clients.clone();
        println!("active client {:?}", clients);

        let mut ok_clients = Vec::new();

        println!("okay active client {:?}", ok_clients);

        for client in clients {
            if client
                .send(sse::Event::Comment("ping".into()))
                .await
                .is_ok()
            {
                ok_clients.push(client.clone());
            }
        }

        self.inner.lock().clients = ok_clients;
    }

    /// Registers client with broadcaster, returning an SSE response body.
    pub async fn new_client(&self) -> impl Responder {
        println!("starting creation");
        let (tx, rx) = tokio::sync::mpsc::channel(10);

        tx.send(sse::Event::Data(sse::Data::new("connected")))
            .await
            .unwrap();
        println!("creating new clients success {:?}", tx);
        self.inner.lock().clients.push(tx);
        sse::Sse::from_infallible_receiver(rx).with_retry_duration(Duration::from_secs(10))
    }

    /// Broadcasts `msg` to all clients.
    pub async fn broadcast(&self, msg: &str) {
        let clients = self.inner.lock().clients.clone();

        let send_futures = clients
            .iter()
            .map(|client| client.send(sse::Event::Data(sse::Data::new(msg))));

        // try to send to all clients, ignoring failures
        // disconnected clients will get swept up by `remove_stale_clients`
        let _ = future::join_all(send_futures).await;
    }
}

pub struct SseState {
    pub broadcaster: Arc<Broadcaster>,
}

#[derive(Serialize, ApiComponent, JsonSchema)]
struct ChangedMessage {
    kind: &'static str,
    event: Uuid,
    group: Option<Uuid>,
    round: Option<i32>,
}

impl SseState {
    pub async fn timetable_changed(&self, event: &Uuid) {
        let msg = ChangedMessage {
            kind: "timetable",
            event: event.clone(),
            group: None,
            round: None,
        };
        self.broadcaster
            .broadcast(&serde_json::to_string(&msg).unwrap_or("invalid".into()))
            .await;
    }
    pub async fn pairing_updated(&self, event: &Uuid, group: &Uuid, round: i32) {
        let msg = ChangedMessage {
            kind: "pairing",
            event: event.clone(),
            group: Some(group.clone()),
            round: Some(round),
        };
        self.broadcaster
            .broadcast(&serde_json::to_string(&msg).unwrap_or("invalid".into()))
            .await;
    }
}

#[api_operation(summary = "read SSE", skip_args = "_user")]
pub async fn sse(
    state: web::Data<SseState>,
    _user: LoggedUser,
    _event_uid: Path<Uuid>,
) -> impl Responder {
    println!("in api");
    state.broadcaster.new_client().await
}
