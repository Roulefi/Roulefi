import getConfig from "../config.js";
import * as connection from "./connection.js";

const nearConfig = getConfig('testnet');

const getTopicName = (topic) => {
  return `com.nearprotocol.${nearConfig.networkId}.explorer.${topic}`;
};

const getProcedureName = (
  procedure
) => {
  return `com.nearprotocol.${nearConfig.networkId}.explorer.${procedure}`;
};

let subscriptions = {};

function subscribe(
  topic,
  handler
) {
  if (!subscriptions[topic]) {
    subscriptions[topic] = [];
  }
  subscriptions[topic].push(handler);
  void connection.subscribeTopic(
    // That's unfair as we actually change topic name
    // But the types match so we'll keep it
    getTopicName(topic),
    (data) => subscriptions[topic].forEach((handler) => handler(data))
  );
  const lastValue = connection.getLastValue(topic);
  if (lastValue) {
    handler(lastValue);
  }
  return () => {
    subscriptions[topic] = subscriptions[topic].filter(
      (lookupHandler) => lookupHandler !== handler
    );
    void connection.unsubscribeTopic(topic);
  };
}

function getCall() {
  return (procedure, args) =>
    connection.call(getProcedureName(procedure), args);
}

const wampApi = { subscribe, getCall };

export default wampApi;