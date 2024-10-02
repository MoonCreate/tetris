import client from "socket.io-client";

const socket = client("ws://localhost:3000");

export const io = socket;