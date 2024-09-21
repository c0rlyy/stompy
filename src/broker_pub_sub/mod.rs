use std::{collections::HashMap, future::Future, pin::Pin};

use lapin::{
    message::Delivery,
    options::{
        BasicConsumeOptions, BasicPublishOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions, QueueDeleteOptions,
    },
    protocol::channel,
    publisher_confirm::PublisherConfirm,
    types::FieldTable,
    BasicProperties, Channel, Consumer, ConsumerDelegate, ExchangeKind, Queue,
};

// pub struct TopicsHanlder {
//                 should create custom types to provide more sematnic value
//                 i guess python habits die hard
//     pub topics: Vec<Mutex<HashMap<String, Vec<String>>>>,
// }

pub async fn create_exchange(
    chan: &Channel,
    exchange_name: &str,
    kind: ExchangeKind,
) -> Result<(), lapin::Error> {
    chan.exchange_declare(
        exchange_name,
        kind,
        ExchangeDeclareOptions::default(),
        FieldTable::default(),
    )
    .await
}

/// This function essantialy allows for creating topic
/// for queues to allow pub/sub architecture
/// routing key is the parameter which will decide where the massages will end up
/// example: rounting_key = "animals" then only the que which has binding patter matching animals
/// will get this message
pub async fn bind_que_to_exchange(
    chan: &Channel,
    que_name: &str,
    exchange_name: &str,
    routing_key: &str,
) -> Result<(), lapin::Error> {
    chan.queue_bind(
        que_name,
        exchange_name,
        routing_key,
        QueueBindOptions::default(),
        FieldTable::default(),
    )
    .await
}

/// creates que, without binding it to exchange make it rather ussless in most scenarious without
/// binding it with exchange
pub async fn create_que(chan: &Channel, que_name: &str) -> Result<lapin::Queue, lapin::Error> {
    chan.queue_declare(
        que_name,
        QueueDeclareOptions::default(),
        FieldTable::default(),
    )
    .await
}

/// sends message to exchange which routes it to correct que using routing_key
///
pub async fn publish_message_to_exchange(
    chan: &Channel,
    exchange_name: &str,
    routing_key: &str,
    data: &[u8],
) -> Result<PublisherConfirm, lapin::Error> {
    chan.basic_publish(
        exchange_name,
        routing_key,
        BasicPublishOptions::default(),
        data,
        BasicProperties::default(),
    )
    .await
}

/// creates a consumer for a named que
/// it allows for "consuming messages" and creating custom behaviour on consumption
pub async fn create_consumer(
    chan: &Channel,
    que: &str,
    consumer_name: &str,
) -> Result<Consumer, lapin::Error> {
    chan.basic_consume(
        que,
        consumer_name,
        BasicConsumeOptions::default(),
        FieldTable::default(),
    )
    .await
}

/// creates que and binds it to exchange at the same time
pub async fn create_que_bind_exchange(
    chan: &Channel,
    que_name: &str,
    exchange_name: &str,
    routing_key: &str,
) -> Result<Queue, lapin::Error> {
    let que = create_que(chan, que_name).await?;
    bind_que_to_exchange(chan, que_name, exchange_name, routing_key).await?;
    Ok(que)
}

// pub async fn create_consumer_behaviour(
//     chan: &Channel,
//     consumer: Consumer,
//     f: impl Fn(
//             Result<Option<lapin::message::Delivery>, lapin::Error>,
//         ) -> Pin<Box<dyn Future<Output = ()> + Send>>
//         + Send
//         + 'static,
// ) -> Result<(), lapin::Error> {
//     consumer.set_delegate(move |delivery| {
//         let future = f(delivery);
//         Box::pin(async move {
//             future.await;
//         })
//     });

//     Ok(())
// }
