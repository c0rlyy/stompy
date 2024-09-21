pub mod broker_pub_sub;

// use lapin::{
//     message::DeliveryResult,
//     options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
//     publisher_confirm::PublisherConfirm,
//     types::FieldTable,
//     BasicProperties, Channel, Connection, ConnectionProperties, Consumer, Queue,
//  };

// pub async fn create_subscriber(
//     chan: &Channel,
//     que_name: &str,
//     tag: &str,
// ) -> Result<Consumer, lapin::Error> {
//     chan.basic_consume(
//         que_name,
//         tag,
//         BasicConsumeOptions::default(),
//         FieldTable::default(),
//     )
//     .await
// }

// pub async fn create_que(chan: &Channel, que_name: &str) -> Result<Queue, lapin::Error> {
//     chan.queue_declare(
//         que_name,
//         QueueDeclareOptions::default(),
//         FieldTable::default(),
//     )
//     .await
// }

// pub async fn send_msg_que(
//     chan: &Channel,
//     que_name: &str,
//     data: &[u8],
// ) -> Result<PublisherConfirm, lapin::Error> {
//     chan.basic_publish(
//         "",
//         que_name,
//         BasicPublishOptions::default(),
//         data,
//         BasicProperties::default(),
//     )
//     .await
// }
