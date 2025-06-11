require('dotenv').config();
const { PubSub } = require('@google-cloud/pubsub');

const express = require('express');

require('dotenv').config();

const app = express();
const port = process.env.PORT || 8080;
const projectId = process.env.GOOGLE_CLOUD_PROJECT_ID;
const topicName = process.env.PUBSUB_TOPIC_NAME;
const subscriptionName = process.env.PUBSUB_SUBSCRIPTION_NAME;

let messages = [];

async function listenForMessages() {
  const pubsub = new PubSub({ projectId });
  const subscription = pubsub.subscription(subscriptionName);

  subscription.on('message', message => {
    const logMessage = `Received message ${message.id}: ${message.data}`;
    console.log(logMessage);
    messages.push(logMessage);
    message.ack();
  });

  subscription.on('error', error => {
    console.error(`Received error: ${error}`);
  });
}

app.get('/', (req, res) => {
  res.send(`
    <h1>Pub/Sub Messages</h1>
    <ul>
      ${messages.map(message => `<li>${message}</li>`).join('')}
    </ul>
  `);
});

app.listen(port, () => {
  console.log(`App listening on port ${port}`);
  listenForMessages();
});