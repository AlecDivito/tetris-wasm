import Page from './Page';
import StateManager from "../StateManager";
import { GetElementById } from '../util';

export default class PauseModal extends Page {
    private resumeBtn: HTMLButtonElement;
    private restartBtn: HTMLButtonElement;
    private controlsBtn: HTMLButtonElement;
    private exitGameBtn: HTMLButtonElement;

    constructor() {
        super('pause-game-modal');

        this.resumeBtn = GetElementById('pause-game-resume') as HTMLButtonElement;
        this.restartBtn = GetElementById('pause-game-restart') as HTMLButtonElement;
        this.controlsBtn = GetElementById('pause-game-control') as HTMLButtonElement;
        this.exitGameBtn = GetElementById('pause-game-exit') as HTMLButtonElement;

        this.resumeBtn.addEventListener('click', this.resumeGame);
        this.restartBtn.addEventListener('click', this.restartGame);
        this.controlsBtn.addEventListener('click', this.controlPage);
        this.exitGameBtn.addEventListener('click', this.exitGame);
    }

    private resumeGame = () => {
        StateManager.GetInstance().GoToGameAndResumeGame();
    }

    private restartGame = () => {
        StateManager.GetInstance().GoToGameAndRestart();
    }

    private controlPage = () => {
        StateManager.GetInstance().PushToHowToPlayModal();
    }
    
    private exitGame = () => {
        StateManager.GetInstance().GoToMainMenu();
    }
}
