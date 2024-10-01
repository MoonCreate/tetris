<script lang="ts">
    import { TetrisGame, Tetromino } from "core_heart";
    const game = new TetrisGame();
    const sfx = new Audio("https://www.joshwcomeau.com/sounds/switch-on.mp3");
    let board = $state(game.print());

    $effect(() => {
        function listenKeyboard(this: Window, ev: KeyboardEvent) {
            switch (ev.key) {
                case "ArrowLeft": game.move_left(); break;
                case "ArrowRight": game.move_right(); break;
                case "ArrowUp": game.move_up(); break;
                case "ArrowDown": game.move_down(); break;
                case "c": game.change(Tetromino.I); break;
                case "r": game.rotate(); break;
            }
            board = game.print();
            sfx.play();
        } 
        window.addEventListener("keydown", listenKeyboard);

        return () => {
            window.removeEventListener("keydown", listenKeyboard);
            game.free();
        }
    });
</script>

<div class="board">
{#each JSON.parse(board).flat() as item}
    <div class="item" class:active={item}></div>
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
            transition: background-color 0.5s ease-in-out;
            width: 2rem;
            height: 2rem;
            box-sizing: border-box;
        }

        .item.active {
            background-color: hotpink;
            box-shadow: 0 0 0.5rem hotpink;
            border-color: transparent;
        }
    }
</style>