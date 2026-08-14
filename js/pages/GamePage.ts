import Page from "./Page";
import { GetElementById } from "../util";
import StateManager from "../StateManager";
import { TetrisConfig } from "../Tetris";

export default class GamePage extends Page {

    private pauseBtn: HTMLButtonElement;

    private rightContentBar: HTMLElement;
    private mainContentBar: HTMLElement

    constructor() {
        super('game-page');
        this.pauseBtn = GetElementById('game-pause') as HTMLButtonElement;
        this.pauseBtn.addEventListener('click', this.pauseGame);

        this.rightContentBar = GetElementById('preview');
        this.mainContentBar = GetElementById('tetris');

        window.addEventListener('resize', this.onResize)
    }

    show() {
        super.show();
    }

    hide() {
        super.hide();
    }

    pauseGame = () => {
        StateManager.GetInstance().GoToPauseModalAndPauseGame();
    }

    CalculateTetrisConfig(): TetrisConfig {
        // The goal:
        // Make sure the tetris board always fits on the screen.
        // It must take the height and width into account when
        // rendering that board

        // 1. Base sizing on viewport dimension limits instead of the canvas width
        let availableHeight = window.innerHeight - 100; // Leave space for headers/footers
        let maxCellSizeByHeight = Math.floor((availableHeight - 1) / 20) - 1; // Assuming 20 rows standard

        // 2. Set bounds (Minimum: 15px, Maximum: 35px)
        let cellSize = Math.max(15, Math.min(maxCellSizeByHeight, 35));

        // 3. Scale preview size proportional to main game grid cell size
        let previewCellSize = Math.max(10, Math.min(Math.floor(cellSize * 0.25), 15));

        return {
            gridColor: "#1e2130",
            cellSize,
            previewCellSize
        };
    }

    private onResize = () => {
        StateManager.GetInstance().UpdateTetrisConfig(this.CalculateTetrisConfig())
    }
}
