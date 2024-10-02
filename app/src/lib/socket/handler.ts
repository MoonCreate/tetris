import { Server, type ServerOptions } from "socket.io";
import type { HttpServer } from "vite";

const users = new Map<string, {
    rawboard: string;
}>();
export default function injectSocketIO(server: Partial<ServerOptions> | HttpServer) {
    const io = new Server(server, {
        cors: {
            origin: "*"
        }
    });

    io.on("connection", socket => {
        let username = "";
        socket.on("adduser", data => {
            if (typeof data !== "string") return socket.emit("adduser", {
                success: false,
                message: "Invalid Data"
            });

            if (users.has(data)) return socket.emit("adduser", {
                success: false,
                message: "User Already Exists"
            });

            username = data;
            users.set(data, { rawboard: "" });

            return socket.emit("adduser", {
                success: true,
                message: "User Added"
            });
        }).on("message", data => {
            if (!username.length) return;
            io.emit("message", {
                id: socket.id,
                from: username,
                message: data
            });
        }).on("disconnect", () => {
            if (username.length) users.delete(username);
        })
    });
}