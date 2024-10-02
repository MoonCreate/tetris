<script lang="ts">
    import { io } from "$lib/socket/connection";

    let username = $state("");
    let message = $state("");

    let errorUsername = $state("not login");

    const messages = $state([] as {
        message: string;
        from: string;
        time: string;
    }[]);

    $effect(() => {
        io.on("adduser", function adduser(data) {
            errorUsername = data.success ? "" : data.message;
            if (data.success) return io.off("adduser", adduser); 
        });

        io.on("message", (message) => {
            console.log(message);
            messages.push(message);
        });

    });

    function sendMessage() {
        const m = message.trim();
        if (!m) return;

        message = "";
        io.emit("message", m);
    }

</script>

<div>
    {errorUsername}
    <h1>Tetris</h1>
    <h2>Enter your username</h2>
    {#if errorUsername.length}
        <input type="text" bind:value={username} />
        <button onclick={() => io.emit("adduser", username)}>enter</button>
    {:else}
        Hello {username}
        <textarea bind:value={message} onkeydown={(e) => e.key === "Enter" && sendMessage()}></textarea>
    {/if}
</div>

<div class="chats">
    {#each messages as message}
        <div>
            <p>{message.from}: {message.message}</p>
            <p>{message.time}</p>
        </div>
    {/each}
</div>

<style>
    .chats {
        display: flex;
        flex-direction: column;
    }
</style>