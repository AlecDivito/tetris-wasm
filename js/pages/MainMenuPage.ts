import Page from "./Page";
import StateManager from "../StateManager";
import { GetElementById } from "../util";

export default class MainMenuPage extends Page {

    private playBtn: HTMLButtonElement;
    private customGameBtn: HTMLButtonElement;
    private howToPlayBtn: HTMLButtonElement;

    constructor() {
        super("main-menu-page");
        this.playBtn = GetElementById('main-menu-play') as HTMLButtonElement;
        this.customGameBtn = GetElementById('main-menu-custom-game') as HTMLButtonElement;
        this.howToPlayBtn = GetElementById('main-menu-how-to-play') as HTMLButtonElement;

        this.playBtn.addEventListener('click', this.playGame);
        this.customGameBtn.addEventListener('click', this.customGame);
        this.howToPlayBtn.addEventListener('click', this.howToPlay);
    }

    playGame = () => {
        StateManager.GetInstance().GoToGameAndStartGame();
    }

    customGame = () => {
        StateManager.GetInstance().GoToCustomGame();
    }

    howToPlay = () => {
        StateManager.GetInstance().PushToHowToPlayModal();
    }
}
