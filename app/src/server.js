import http from "node:http";
import express from "express";
import injectSocketIO from "./lib/socket/handler";
import { handler } from "../build/handler";

const app = express();
const server = http.createServer(app);

injectSocketIO(server);

// app.use(handler);

server.listen(3000, () => {
    console.log("Running on http://localhost:3000");
});