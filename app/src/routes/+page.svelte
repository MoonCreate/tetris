<script lang="ts">
	import { goto } from "$app/navigation";
    import MessageContainer from "$components/MessageContainer.svelte";
    import { io } from "$lib/socket/connection";

    let username = $state("");

    let errorUsername = $state("not login");

    $effect(() => {
        io.on("adduser", function adduser(data) {
            errorUsername = data.success ? "" : data.message;
            if (data.success) return io.off("adduser", adduser); 
        });
    });

    $effect(() => {
       if (!errorUsername.length) goto("/play");
    })
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
    {/if}
</div>

<MessageContainer />