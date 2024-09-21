use std::{error::Error, sync::Arc};

use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    publisher_confirm::PublisherConfirm,
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, Consumer, ExchangeKind, Queue,
};
use stompy::broker_pub_sub::{
    bind_que_to_exchange, create_consumer, create_exchange, create_que, publish_message_to_exchange,
};
// use stompy::{create_que, create_subscriber, send_msg_que};

#[tokio::main]
async fn main() {
    let uri = "amqp://localhost:5672";
    let options = ConnectionProperties::default()
        .with_executor(tokio_executor_trait::Tokio::current())
        .with_reactor(tokio_reactor_trait::Tokio)
        .with_connection_name("pub_sub_stuff".into());

    let connection = Connection::connect(uri, options).await.unwrap();
    let channel = connection.create_channel().await.unwrap();
    let chan_container = Arc::new(channel);

    let chan_ref = chan_container.clone();

    // Capture consumer tag
    //     consumer.set_delegate(move |delivery: DeliveryResult| {
    //         let consumer_tag = consumer_tag.clone(); // Clone tag if needed inside async block

    //         async move {
    //             println!("Starting async closure execution");

    //             let delivery = match delivery {
    //                 // Carries the delivery alongside its channel
    //                 Ok(Some(delivery)) => delivery,
    //                 // The consumer got canceled
    //                 Ok(None) => {
    //                     dbg!("Consumer canceled");
    //                     return;
    //                 }
    //                 // Carries the error and is always followed by Ok(None)
    //                 Err(error) => {
    //                     dbg!("Failed to consume queue message", error);
    //                     return;
    //                 }
    //             };

    //             dbg!("Processing message...");
    //             // Do something with the delivery data (The message payload)
    //             println!("Consumer tag: {}", consumer_tag);
    //             dbg!("{}", &delivery.data);
    //             delivery
    //                 .ack(BasicAckOptions::default())
    //                 .await
    //                 .expect("Failed to acknowledge message");
    //         }
    //     });
    //     dbg!("finsihed");
    // }
    std::future::pending::<()>().await;
}
