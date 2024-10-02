<script lang="ts">
	import { io } from "$lib/socket/connection";

    const messages = $state([] as {
        message: string;
        id: string;
        from: string;
        fromMe: boolean;
    }[]);

    let textToSend = $state("");
    let box: HTMLDivElement;

    $effect(() => {
        io.on("message", message => {
            messages.push({
                ...message,
                fromMe: message.id === io.id
            });
            scroll();
        });

        return () => {
            io.off("message");
        }
    });

    function scroll () {
        box.scrollTo({
            top: box.scrollHeight,
            behavior: "smooth"
        });
    }

    function sendMessage() {
        io.emit("message", textToSend);
        textToSend = "";
    }
</script>

<div class="message" bind:this={box}>
    <div class="item">
        <h1>GLOBAL CHAT</h1>
    </div>
    {#each messages as message }
        <div class="item" class:me={message.fromMe}>
            <img
            src="https://api.dicebear.com/9.x/open-peeps/svg?seed=Adrian"
            alt="avatar" />
            <p>
                <span class="from">{message.from}</span>
                <span class="message">{message.message}</span>
            </p>       
        </div>
    {/each}
    <div class="boxtext">
        <textarea bind:value={textToSend}></textarea>
        <button onclick={sendMessage}>send</button>
    </div>
</div>

<style>
    div.message {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        width: 30vw;
        height: auto;
        min-width: 200px;
        overflow:auto;
        position: fixed;
        top: 0;
        bottom: 0;
        left: 0;
        padding: 1rem;
        box-sizing: border-box;
        margin: 1rem;
        backdrop-filter: blur(0.5rem);

        .item:nth-child(1) {
            position: sticky;
            top: 0;
            color: white;
            backdrop-filter: blur(0.5rem);
        }

        .item {
            display: flex;
            gap: 0.5rem;
            align-items: center;
            animation: appear 100ms ease;
            img {
                flex: 0;
                width: 3rem;
                height: 3rem;
                aspect-ratio: 1;
                border-radius: 50%;
                object-fit: cover;
                background-color: aquamarine;
            }
            p {
                background-color: rgba(255, 255, 255, 0.074);
                padding: 0.5rem;
                margin: 0;
                border-radius: 0.5rem;
                display: flex;
                flex-direction: column;
                .from {
                    font-weight: bold;
                    color: aqua;
                }
                span.message {
                    color: white;
                    text-wrap: wrap;
                }
            }

            h1 {
                text-align: center;
                display: block;
                margin: 0;
                width: 100%;
            }
        }

        .item.me {
            flex-direction: row-reverse;
            img {
                background-color: hotpink;
            }
            p {
                align-items: self-end;
                .from {
                    color: hotpink;
                }
            }
        }
        .boxtext {
            position: sticky;
            bottom: 0;
            display: flex;
            height: max-content;
            gap: 0.5rem;
            margin-top: auto;
            textarea {
                border-radius: 0.5rem;
                background-color: rgba(255, 255, 255, 0.074);
                margin: 0;
                padding: 1rem;
                color: white;
                border: none;
                outline: none;
                resize: none;
                flex: 1;
            }
            button {
                border-radius: 0.5rem;
                flex: 0;
            }
        }
    }

    div.message::-webkit-scrollbar {
        width: 0rem;
    }

    @keyframes appear {
        0% { opacity: 0; transform: translateY(-1rem); }
        100% { opacity: 1; transform: translateY(0); }
    }
</style>