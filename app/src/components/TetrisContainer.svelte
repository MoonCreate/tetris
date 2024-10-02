<script lang="ts">
	import { loadSFX } from "$lib/util";
    import { TetrisGame } from "core_heart";
    /* svelte-ignore non_reactive_update */
    let game = new TetrisGame();

    let rawprint = $state(game.print());
    let boardprint = $derived(JSON.parse(rawprint).flat() as number[]);
    let isOver = $state(false);
    let isPaused = $state(false);

    const sfx = loadSFX();

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
        if (eliminated) {
            triggerEffects("shake");
            sfx.line.play();
        } 
    }


    let gameRun: Timer;

    const createTimeout = () => gameRun = setTimeout(() => {
        const success = game.move_down();
        if (!success) {
            place();
            isOver = game.is_over(); 
        }
        rawprint = game.print();
        if (!isOver && !isPaused) createTimeout();
    }, 500);

    const listenKeyboard = (ev: KeyboardEvent) => {
        const keys = (...keys: string[]) => {
            const result = keys.includes(ev.key);
            return result;
        }

        let success = false;

        switch(true) {
            case keys("ArrowLeft", "h") && !isOver && !isPaused: success = game.move_left(); triggerEffects("jitterLeft"); sfx.move.play(); break;
            case keys("ArrowRight", "l") && !isOver && !isPaused: success = game.move_right(); triggerEffects("jitterRight"); sfx.move.play(); break;
            case keys("ArrowDown", "j") && !isOver && !isPaused: success = game.move_down(); sfx.move.play(); break;
            case keys("ArrowUp", "k") && !isOver && !isPaused: success = game.rotate_clockwise(); sfx.move.play(); break;
            case keys("z") && !isOver && !isPaused: success = game.rotate_counterclockwise(); sfx.move.play(); break;
            case keys("Escape"): game.free(); game = new TetrisGame(); success = true; rawprint = game.print(); isOver = false; break;
            case keys(" ") && !isOver && !isPaused: success = true; while(game.move_down()) {}; place(); isOver = game.is_over(); break;
            case keys("p") && !isOver: isPaused = !isPaused; break;
        }

        if (isPaused) return;

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
            --tetromino-color: {getTetrominoColor(item === 8 ? (game.piece ?? 0) + 1: item)};
            --tetromino-opacity: {item === 8 ? 0.2 : 1}
        "
        ></div>
{/each}
{#if isOver}
    <div class="announce" style="--announce-color: hsl(351deg, 74%, 73%);">
        <h1>Game Over</h1>
    </div>
{/if}
{#if isPaused}
    <div class="announce" style="--announce-color: hsl(40deg, 70%, 78%);">
        <h1>Paused</h1>
    </div>
{/if}
</div>

<style>
    .board {
        display: grid;
        grid-template-columns: repeat(10, min-content);
        grid-template-rows: repeat(20, min-content);
        gap: 0.5rem;
        position: relative;

        .item {
            --tetromino-opacity: 100%;
            --tetromino-color: 0deg, 0%, 100%;
            border: 1px hsl(230deg, 19%, 26%) dashed;
            background-color: hsla(var(--tetromino-color), var(--tetromino-opacity));
            transition: background-color 100 ease;
            width: 1.5rem;
            height: 1.5rem;
            box-sizing: border-box;
            border-radius: 10%;
        }

        .item.active {
            border-color: transparent;
            color: hsla(var(--tetromino-color), 50%);
            box-shadow: 0 0 0.5rem;
        }

        .announce {
            position: absolute;
            inset: 0;
            width: fit-content;
            height: fit-content;
            margin: auto;
            background-color: hsla(236deg, 23%, 12%, 85%);
            border-radius: .5rem;
            padding: 1rem;
            border: 1px hsl(230deg, 19%, 26%) dashed;
            backdrop-filter: blur(1rem);
            * {
                padding: 0;
                margin: 0;
                box-sizing: border-box;
                font-family: monospace;
                color: var(--announce-color);
            }
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