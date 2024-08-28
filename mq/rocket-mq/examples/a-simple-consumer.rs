use rocketmq::conf::{ClientOption, SimpleConsumerOption};
use rocketmq::model::common::{FilterExpression, FilterType};
use rocketmq::SimpleConsumer;

#[tokio::main]
async fn main() {
    // recommend to specify which topic(s) you would like to send message to
    // simple consumer will prefetch topic route when start and failed fast if topic not exist
    let mut consumer_option = SimpleConsumerOption::default();
    consumer_option.set_topics(vec!["test_topic"]);
    consumer_option.set_consumer_group("SimpleConsumerGroup");

    // set which rocketmq proxy to connect
    let mut client_option = ClientOption::default();
    client_option.set_access_url("127.0.0.1:8080");

    // build and start simple consumer
    let mut consumer = SimpleConsumer::new(consumer_option, client_option).unwrap();
    consumer.start().await.unwrap();

    // pop message from rocketmq proxy
    let receive_result = consumer
        .receive(
            "test_topic".to_string(),
            &FilterExpression::new(FilterType::Tag, "test_tag"),
        )
        .await;
    debug_assert!(
        receive_result.is_ok(),
        "receive message failed: {:?}",
        receive_result.unwrap_err()
    );

    let messages = receive_result.unwrap();
    for message in messages {
        println!("receive message: {:?}", message);
        // ack message to rocketmq proxy
        let ack_result = consumer.ack(&message).await;
        debug_assert!(
            ack_result.is_ok(),
            "ack message failed: {:?}",
            ack_result.unwrap_err()
        );
    }
}
