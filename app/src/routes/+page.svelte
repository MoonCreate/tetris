<script lang="ts">
    import { TetrisGame, Tetromino } from "core_heart";
    let game = new TetrisGame();
    const bgm = new Audio("./BGM.mp3");
    const sfx = new Audio("https://www.joshwcomeau.com/sounds/switch-on.mp3");
    let board = $state(game.print());
    let tetromino = 0;

    const getTetromino = (num: number) => {
        switch (num) {
            case 0: return Tetromino.I;
            case 1: return Tetromino.J;
            case 2: return Tetromino.L;
            case 3: return Tetromino.O;
            case 4: return Tetromino.S;
            case 5: return Tetromino.T;
            default: return Tetromino.Z;
        }
    }

    const changeTetromino = () => {
        tetromino++;
        if (tetromino > 6) tetromino = 0;
        game.change(getTetromino(tetromino));
    }

    $effect(() => {
        bgm.loop = true;
        function listenKeyboard(this: Window, ev: KeyboardEvent) {
            switch (ev.key) {
                case "ArrowLeft": game.move_left(); break;
                case "ArrowRight": game.move_right(); break;
                case "ArrowDown": game.move_down(); break;
                case "c": changeTetromino(); break;
                case "ArrowUp": game.rotate_clockwise(); break;
                case "t": game.rotate_counterclockwise(); break;
                case " ": while(game.move_down()) {}; game.place(); break;
                case "r":
                    game.free();
                    game = new TetrisGame();
            }
            board = game.print();
            sfx.play();
            if (!bgm.played.length) bgm.play();
        } 

        const interval = setInterval(() => {
            const success = game.move_down();
            board = game.print();
            if (!success) game.place();
        }, 1000);

        window.addEventListener("keydown", listenKeyboard);
        return () => {
            clearInterval(interval);
            window.removeEventListener("keydown", listenKeyboard);
            game.free();
        }
    });
</script>

<div class="board">
{#each JSON.parse(board).flat() as item}
    <div class="item" class:active={item === 1} class:ghost={item === 2}></div>
{/each}
</div>

<style>

    :global(body) {
        margin: 0;
        padding: 0.5rem;
    }
    .board {
        display: grid;
        grid-template-columns: repeat(10, min-content);
        grid-template-rows: repeat(20, min-content);
        gap: 0.5rem;

        .item {
            border: 1px solid black;
            background-color: white;
            transition: background-color 300ms ease-out;
            width: 1.5rem;
            height: 1.5rem;
            box-sizing: border-box;
        }

        .item.active {
            background-color: hotpink;
            box-shadow: 0 0 0.5rem hotpink;
            border-color: transparent;
        }

        .item.ghost {
            background-color: cyan;
        }
    }
</style>