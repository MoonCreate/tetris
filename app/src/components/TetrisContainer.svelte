<script lang="ts">
    import initWasm, { TetrisGame } from "core_heart";

    type TetrisContainerProps = {
        readonly loadWasm?: boolean;
    };
    const { loadWasm = false }: TetrisContainerProps = $props();

    /* svelte-ignore non_reactive_update */
    let game = new TetrisGame();

    let rawprint = $state(game.print());
    let boardprint = $derived(JSON.parse(rawprint).flat() as number[]);

    // effects
    const effects = $state({
        shake: false,
        jitterRight: false,
        jitterLeft: false
    });

    const triggerEffects = (key: keyof typeof effects) => {
        effects[key] = true;
        setTimeout(() => effects[key] = false, 500);
    }

    const getTetrominoColor = (tetromino: number) => {
        switch (tetromino) {
            case 1: return "227deg, 68%, 88%";
            case 2: return "40deg, 70%, 78%";
            case 3: return "105deg, 48%, 72%";
            case 4: return "189deg, 59%, 73%";
            case 5: return "267deg, 83%, 80%";
            case 6: return "351deg, 74%, 73%";
            case 7: return "220deg, 83%, 75%";
        }
    }

    const place = () => {
        const eliminated = game.place();
        if (eliminated) triggerEffects("shake");
    }


    let gameRun: Timer;

    const createTimeout = () => gameRun = setTimeout(() => {
        const success = game.move_down();
        if (!success) place();
        rawprint = game.print();
        createTimeout();
    }, 500);

    const listenKeyboard = (ev: KeyboardEvent) => {
        const keys = (...keys: string[]) => {
            const result = keys.includes(ev.key);
            return result;
        }

        let success = false;

        switch(true) {
            case keys("ArrowLeft", "h"): success = game.move_left(); triggerEffects("jitterLeft"); break;
            case keys("ArrowRight", "l"): success = game.move_right(); triggerEffects("jitterRight"); break;
            case keys("ArrowDown", "j"): success = game.move_down(); break;
            case keys("ArrowUp", "k"): success = game.rotate_clockwise(); break;
            case keys("z"): success = game.rotate_counterclockwise(); break;
            case keys("Escape"): game.free(); game = new TetrisGame(); success = true; rawprint = game.print(); break;
            case keys(" "): success = true; while(game.move_down()) {}; place(); break;
        }

        if (success) {
            rawprint = game.print();
            clearTimeout(gameRun);
            createTimeout();
        }
    }

    $effect(() => {
        window.addEventListener("keydown", listenKeyboard);
        createTimeout();
        return () => {
            window.removeEventListener("keydown", listenKeyboard);
            clearTimeout(gameRun);
            game.free();
        }
    });
</script>

{#snippet board()}
    <div
        class="board"
        class:shake={effects.shake}
        class:jitter-right={effects.jitterRight}
        class:jitter-left={effects.jitterLeft}>
    {#each boardprint as item}
        <div
            class="item"
            class:active={item > 0 && item < 8}
            style="
                --tetromino-color: {getTetrominoColor(item === 8 ? game.current_tetromino + 1: item)};
                --tetromino-opacity: {item === 8 ? 0.5 : 1}
            "
            ></div>
    {/each}
    </div>
{/snippet}

{#snippet init()}
    <div>Loading wasm....</div>
{/snippet}

{#if loadWasm}
    {#await initWasm()}
        {@render init()}
    {:then} 
        {@render board()}
    {/await}
{:else}
    {@render board()}
{/if}

<style>
    :global(body) {
        margin: 0;
        padding: 0.5rem;
        display: flex;
        justify-content: space-around;
        align-items: center;
        width: 100vw;
        height: 100vh;
        max-width: 100%;
        max-height: 100%;
        box-sizing: border-box;
        background-color: hsl(236deg, 23%, 12%);
        gap: 0.5rem;
    }
    .board {
        display: grid;
        grid-template-columns: repeat(10, min-content);
        grid-template-rows: repeat(20, min-content);
        gap: 0.5rem;

        .item {
            --tetromino-opacity: 100%;
            --tetromino-color: 0deg, 0%, 100%;
            border: 1px hsl(230deg, 19%, 26%) dashed;
            background-color: hsla(var(--tetromino-color), var(--tetromino-opacity));
            transition: background-color 100 ease;
            width: 1.5rem;
            height: 1.5rem;
            box-sizing: border-box;
        }

        .item.active {
            border-color: transparent;
        }
    }

    @keyframes shake {
        0% { transform: translate(1px, 1px) rotate(0deg); }
        10% { transform: translate(-1px, -2px) rotate(-1deg); }
        20% { transform: translate(-3px, 0px) rotate(1deg); }
        30% { transform: translate(3px, 2px) rotate(0deg); }
        40% { transform: translate(1px, -1px) rotate(1deg); }
        50% { transform: translate(-1px, 2px) rotate(-1deg); }
        60% { transform: translate(-3px, 1px) rotate(0deg); }
        70% { transform: translate(3px, 1px) rotate(-1deg); }
        80% { transform: translate(-1px, -1px) rotate(1deg); }
        90% { transform: translate(1px, 2px) rotate(0deg); }
        100% { transform: translate(1px, -2px) rotate(-1deg); }
    }

    @keyframes jitter-right {
        0% { transform: translateX(0) rotate(0deg); }
        50% { transform: translateX(0.5rem) rotate(1deg); }
        100% { transform: translateX(0) rotate(0deg); }
    }

    @keyframes jitter-left {
        0% { transform: translateX(0) rotate(0deg); }
        50% { transform: translateX(-0.5rem) rotate(-1deg); }
        100% { transform: translateX(0) rotate(0deg); }
    }

    .board.shake {
        animation: shake 0.5s;
    }

    .board.jitter-right {
        animation: jitter-right 0.5s;
    }

    .board.jitter-left {
        animation: jitter-left 0.5s;
    }
</style>