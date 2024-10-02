import type { PluginOption } from "vite";
import injectSocketIO from "./handler";

export const websocketServer: PluginOption = {
    name: "webSocketServer",
    configureServer(server) {
        injectSocketIO(server.httpServer!);
    }
}