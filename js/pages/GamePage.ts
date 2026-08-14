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
        const viewportWidth = window.innerWidth;
        const viewportHeight = window.innerHeight;
    
        // -------------------------
        // Main game board
        // -------------------------
        const availableHeight = viewportHeight - 100;
        const availableWidth = viewportWidth - 40;
    
        const maxCellSizeByHeight = Math.floor(availableHeight / 20);
        const maxCellSizeByWidth = Math.floor(availableWidth / 10);
        const cellSize = Math.max(15, Math.min( maxCellSizeByHeight, maxCellSizeByWidth, 35));
    
        // -------------------------
        // Preview
        // -------------------------
        //
        // Desktop:
        //   ~200px wide
        //
        // Mobile:
        //   ~10vw wide
        //
        const previewWidth = viewportWidth >= 768 ? 160 : viewportWidth * 0.15;
        const previewHeight = viewportHeight * 0.8;
    
        // A Tetris piece is at most 4 cells wide/high.
        const previewCellSize = Math.max(5, Math.floor(Math.min(previewWidth / 4, previewHeight / 4)));
    
        // -------------------------
        // Hold
        // -------------------------
        //
        // Keep this a small square.
        //
        const holdContainerSize = viewportWidth >= 768 ? 100 : 40;
        const holdCellSize = Math.max(2, Math.floor(holdContainerSize / 4));
    
        return {
            gridColor: "#1e2130",
            cellSize,
            previewCellSize,
            holdCellSize
        };
    }

    private onResize = () => {
        StateManager.GetInstance().UpdateTetrisConfig(this.CalculateTetrisConfig())
    }
}
