import { Cell, Universe } from "wasm-game-of-life";
import { memory} from "wasm-game-of-life/wasm_game_of_life_bg.wasm";

const UNIVERSE_HEIGHT = 64;
const UNIVERSE_WIDTH = 64;

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.generate_predefined_universe_random(UNIVERSE_HEIGHT, UNIVERSE_WIDTH);
const height = universe.height();
const width = universe.width();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext('2d');
const tick_btn = document.getElementById("tick-the-world-btn");
const epoch_counter = document.getElementById("epoch-counter");

const renderLoop = () => {
    universe.tick();
    drawGrid();
    drawCells();
    incrementEpoch();
};

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, col) => {
    return row * width + col;
};

const isBitSet = (idx, bit_array) => {
    const byte_idx = Math.floor(idx / 8);
    const bit_mask = 1 << (idx % 8);
    return (bit_array[byte_idx] & bit_mask) === bit_mask;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);
    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            ctx.fillStyle = isBitSet(idx, cells)
                ? ALIVE_COLOR
                : DEAD_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    ctx.stroke();
};

const resetEpochCounter = () => {
    epoch_counter.value = 0;
};

const incrementEpoch = () => {
    epoch_counter.value++;
};

drawGrid();
drawCells();
resetEpochCounter();

tick_btn.onclick = () => {
    renderLoop();
};
