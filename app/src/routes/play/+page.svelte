<script lang="ts">
	import TetrisContainer from "$components/TetrisContainer.svelte";
	import { loadBGM } from "$lib/util";
    import initWasm from "core_heart";
    let wasmInited = $state(false);
    let bgmName = $state("");
    $effect(() => {
        const bgm = loadBGM();
        const timeout = setTimeout(() => {
            bgm.play();
            bgmName = bgm.innerText;
            setTimeout(() => {
                bgmName = "";
            }, 1000 * 10);
        }, 5000);

        initWasm().then(() => {
            wasmInited = true;
        });

        return () => {
            clearTimeout(timeout);
        }
    })
</script>

{#if bgmName.length}
    <div class="bgm-anounce">Now playing {bgmName}</div>
{/if}

{#if wasmInited}
    <TetrisContainer />
    <TetrisContainer />
{:else}
    <div>Loading Wasm please wait....</div>
{/if}

<style>
    @keyframes bounce {
        0% { transform: translateY(0); }
        50% { transform: translateY(-0.5rem); }
        100% { transform: translateY(0); }
    }

    .bgm-anounce {
        position: fixed;
        left: 0;
        bottom: 0;
        color: white;
        padding: 1rem;
        animation: bounce 1s ease infinite;
    }
</style>