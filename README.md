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
3. Send the `StartConsumer` command to start the consumer
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
{"type": "DeclareQueue", "name": "test"}
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
