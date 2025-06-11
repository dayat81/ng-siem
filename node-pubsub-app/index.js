const express = require('express');
const { PubSub } = require('@google-cloud/pubsub');

require('dotenv').config();

const app = express();
const port = process.env.PORT || 8080;
const projectId = process.env.GOOGLE_CLOUD_PROJECT_ID;
const topicName = process.env.PUBSUB_TOPIC_NAME;
const subscriptionName = process.env.PUBSUB_SUBSCRIPTION_NAME;

const messages = [];

async function listenForMessages() {
  const pubsub = new PubSub({ projectId });
  const subscription = pubsub.subscription(subscriptionName);

  subscription.on('message', message => {
    const logMessage = `Received message ${message.id}: ${message.data}`;
    console.log(logMessage);
    messages.push(logMessage);
    wss.clients.forEach(client => {
      console.log(`Sending message to client: ${client.readyState}`);
      if (client.readyState === 1) {
        client.send(logMessage);
      } else {
        console.log('Client not connected, readyState:', client.readyState);
      }
    });
    message.ack();
  });

  subscription.on('error', error => {
    console.error(`Received error from subscription: ${error}`);
  });

  subscription.on('close', () => {
    console.log('Subscription closed');
  });
}

app.use(express.static('public'));

app.get('/', (req, res) => {
  res.sendFile(__dirname + '/public/index.html');
});

const server = app.listen(port, () => {
  console.log(`App listening on port ${port}`);
  listenForMessages();
});

const { WebSocketServer } = require('ws');
const wss = new WebSocketServer({ server });

wss.on('connection', ws => {
  console.log('Client connected');

  ws.on('close', () => {
    console.log('Client disconnected');
  });

  ws.on('error', error => {
    console.error(`WebSocket error: ${error}`);
  });

  ws.on('message', message => {
    console.log(`Received message from client: ${message}`);
  });
});