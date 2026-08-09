import Page from "./Page";
import { GetElementById } from "../util";
import StateManager from "../StateManager";
import { TetrisConfig } from "Tetris";

export default class GamePage extends Page {
    
    private pauseBtn: HTMLButtonElement;

    private rightContentBar: HTMLElement;
    private mainContentBar: HTMLElement
    private leftContentBar: HTMLElement;

    constructor() {
        super('game-page');
        this.pauseBtn = GetElementById('game-pause') as HTMLButtonElement;
        this.pauseBtn.addEventListener('click', this.pauseGame);

        this.rightContentBar = GetElementById('game__board__item--right');
        this.mainContentBar = GetElementById('game__board__item--main');
        this.leftContentBar = GetElementById('game__board__item--left');
    }

    show() {
        this.BeforePageLoad();
        super.show();
    }

    hide() {
        super.hide();
    }

    BeforePageLoad() {
        let padding = 8 * 4;
        let width = document.documentElement.clientWidth - padding;
        
        let mainWidth = .465 * width;
        let sideWidth = .15 * width;

        // this.rightContentBar.style.width = `${sideWidth}px`;
        // this.leftContentBar.style.width = `${sideWidth}px`;
        // this.mainContentBar.style.width = `${mainWidth}px`;
    }

    pauseGame = () => {
        StateManager.GetInstance().GoToPauseModalAndPauseGame();
    }

    CalculateTetrisConfig(): TetrisConfig {
        let width = document.documentElement.clientWidth - 8 * 4;
        
        let cellSize = (.70 * width) / 10;
        let previewCellSize = (.15 * width) / 4;

        if (cellSize > 35) {
            cellSize = 35;
        }
        if (previewCellSize > 15) {
            previewCellSize = 15;
        }

        return {
            gridColor: "#1e2130",
            cellSize,
            previewCellSize
        };
    }

}
