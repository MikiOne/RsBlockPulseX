///
// rocketmq-client = "0.1.0"
// rocketmq-common = "0.2.0"
// rocketmq-rust = "0.2.0"

// use rocketmq_client::producer::default_mq_producer::DefaultMQProducer;
// use rocketmq_client::producer::mq_producer::MQProducer;
// use rocketmq_client::Result;
// use rocketmq_common::common::message::message_single::Message;
// use rocketmq_rust::rocketmq;
//
// pub const MESSAGE_COUNT: usize = 1;
// pub const PRODUCER_GROUP: &str = "please_rename_unique_group_name";
// pub const DEFAULT_NAMESRVADDR: &str = "127.0.0.1:9876";
// pub const TOPIC: &str = "TopicTest";
// pub const TAG: &str = "TagA";
//
// #[rocketmq::main]
// pub async fn main() -> Result<()> {
//     //init logger
//     rocketmq_common::log::init_logger();
//
//     // create a producer builder with default configuration
//     let builder = DefaultMQProducer::builder();
//
//     let mut producer = builder
//         .producer_group(PRODUCER_GROUP.to_string())
//         .name_server_addr(DEFAULT_NAMESRVADDR.to_string())
//         .build();
//
//     producer.start().await?;
//
//     for _ in 0..10 {
//         let message = Message::with_tags(TOPIC, TAG, "Hello RocketMQ".as_bytes());
//
//         let send_result = producer.send_with_timeout(message, 2000).await?;
//         println!("send result: {}", send_result);
//     }
//     producer.shutdown().await;
//
//     Ok(())
// }
fn main() {}