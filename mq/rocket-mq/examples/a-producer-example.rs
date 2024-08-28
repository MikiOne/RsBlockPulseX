use rocketmq::conf::{ClientOption, ProducerOption};
use rocketmq::model::message::MessageBuilder;
use rocketmq::Producer;

#[tokio::main]
async fn main() {
    // recommend to specify which topic(s) you would like to send message to
    // producer will prefetch topic route when start and failed fast if topic not exist
    let mut producer_option = ProducerOption::default();
    producer_option.set_topics(vec!["test_topic"]);

    // set which rocketmq proxy to connect
    let mut client_option = ClientOption::default();
    client_option.set_access_url("127.0.0.1:8080");

    // build and start producer
    let mut  producer = Producer::new(producer_option, client_option).unwrap();
    producer.start().await.unwrap();

    // build message
    let message = MessageBuilder::builder()
        .set_topic("test_topic")
        .set_tag("test_tag")
        .set_body("hello world".as_bytes().to_vec())
        .build()
        .unwrap();

    // send message to rocketmq proxy
    let result = producer.send(message).await;
    debug_assert!(result.is_ok(), "send message failed: {:?}", result);
    println!(
        "send message success, message_id={}",
        result.unwrap().message_id()
    );
}
