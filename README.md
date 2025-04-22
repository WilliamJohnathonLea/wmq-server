# wmq-server
Will's Message Queue Server

## Setting up a consumer
1. Connect to the server via TCP
```sh
> nc localhost 42069
```
2. Send the `AssignConsumer` command to assign the connection as a consumer
```json
{"type": "AssignConsumer", "id": "my_id"}
```
3. Send the `AssignQueue` command to assign a queue to the consumer
```json
{"type": "AssignQueue", "consumer_id": "my_id", "queue": "test"}
```
4. Send the `StartConsumer` command to start the consumer
```json
{"type": "StartConsumer", "id": "my_id"}
```

## Setting up a producer
1. Connect to the server via TCP
```sh
> nc localhost 42069
```
2. Send the `AssignProducer` command to assign the connection as a producer
```json
{"type": "AssignProducer", "id": "my_id"}
```

## Declaring a queue
After connecting to the server, send the `DeclareQueue` command to declare a queue.
```json
{"type": "DeclareQueue", "name": "test", "size": 100}
```
A queue can have a maximum size of 5000 messages.

It is recommended to declare a queue from a consumer with the intention of consuming it.
This because the queues are in-memory channels and
if the channel has no consumers then writing to the queue will fail.
However, it is possible to declare a queue from a producer.

## Commands
Commands are sent from the client to set up consumers and producers and to send messages to a queue.

### AssignConsumer
Assigns a connection as a consumer.
```json
{"type": "AssignConsumer", "id": "my_id"}
```
### StartConsumer
Starts a consumer.
```json
{"type": "StartConsumer", "id": "my_id"}
```
### AssignProducer
Assigns a connection as a producer.
```json
{"type": "AssignProducer", "id": "my_id"}
```
### DeclareQueue
Declares a queue. This can be done by either a consumer or a producer.
```json
{"type": "DeclareQueue", "name": "test", "size": 5000}
```
### AssignQueue
Assigns a queue to a consumer.
```json
{"type": "AssignQueue", "consumer_id": "my_id", "queue": "test"}
```
### SendMessage
Send a message to a queue.
```json
{"type": "SendMessage", "queue": "test", "producer_id": "my_id", "msg": {"sender": "producer1", "body": "hello"}}
```
