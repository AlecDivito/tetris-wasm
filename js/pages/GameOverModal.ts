import Page from "./Page";
import { GetElementById } from "../util";
import StateManager from "../StateManager";
import { topScores, submitGameScore } from "../services/pocketbase";
import { ListResult, RecordModel } from "pocketbase";

export default class GameOverModal extends Page {

    private restartBtn: HTMLButtonElement;
    private exitGameBtn: HTMLButtonElement;
    private scoreText: HTMLElement;
    private levelText: HTMLElement;
    private rowText: HTMLElement;

    private loading: HTMLElement;
    private failText: HTMLElement;
    private scoreList: HTMLElement;
    private highScoreRowTemplate: HTMLTemplateElement;

    private gameOverForm: HTMLElement;
    private saveScoreForm: HTMLFormElement;
    private gameOverSavedGameText: HTMLElement;

    private playerName: string | null

    constructor() {
        super("game-over-modal");

        this.playerName = window.localStorage.getItem('playerName')

        this.scoreText = GetElementById('game-over-score') as HTMLElement;
        this.levelText = GetElementById('game-over-level') as HTMLElement;
        this.rowText = GetElementById('game-over-row') as HTMLElement;

        this.exitGameBtn = GetElementById('game-over-exit') as HTMLButtonElement;
        this.restartBtn = GetElementById('game-over-restart') as HTMLButtonElement;

        this.loading = GetElementById('game-over-score-list-loading') as HTMLElement;
        this.failText = GetElementById('game-over-score-list-fail') as HTMLElement;
        this.scoreList = GetElementById('game-over-score-list') as HTMLElement;
        this.highScoreRowTemplate = GetElementById('high-score-row') as HTMLTemplateElement;

        this.saveScoreForm = GetElementById('game-over-save-score-form') as HTMLFormElement
        this.gameOverForm = GetElementById('game-over-form') as HTMLElement
        this.gameOverSavedGameText = GetElementById('game-over-saved-score-text') as HTMLElement

        this.exitGameBtn.addEventListener('click', this.exitGame);
        this.restartBtn.addEventListener('click', this.restartGame);
        this.saveScoreForm.addEventListener('submit', this.submitScore);

        if (this.playerName) {
            this.gameOverForm.style.display = 'none'
        }

        this.hideHighScoreList()
    }

    show = () => {
        this.showSavePlayerScore()
        const score = StateManager.GetInstance().getCurrentGameScore();
        const level = StateManager.GetInstance().getCurrentGameLevel();
        const row = StateManager.GetInstance().getCurrentGameRowCompleted();
        this.scoreText.textContent = score.toLocaleString()
        this.levelText.textContent = level.toString()
        this.rowText.textContent = row.toString()
        this.parentElement.style.display = "block";
        topScores().then(data => this.render(data)).catch(this.renderFail)
    }

    hide = () => {
        this.parentElement.style.display = "none";
        this.hideHighScoreList()
    }

    private exitGame = () => {
        console.log('exit current game');
        StateManager.GetInstance().GoToMainMenu();
    }

    private restartGame = () => {
        StateManager.GetInstance().GoToGameAndRestartGame();
    }

    private submitScore = (event: SubmitEvent) => {
        event.preventDefault()
        event.stopPropagation()
        const formData = new FormData(this.saveScoreForm);
        const data = Object.fromEntries(formData.entries());
        submitGameScore(data['player'] as string, StateManager.GetInstance().getCurrentGameScore(), {
            score: StateManager.GetInstance().getCurrentGameScore(),
            level: StateManager.GetInstance().getCurrentGameLevel(),
            row: StateManager.GetInstance().getCurrentGameRowCompleted(),
        }).finally(() => {
            // window.localStorage.setItem('playerName', data['player'] as string)
            this.savedPlayerScore()
            topScores().then(data => this.render(data)).catch(this.renderFail)
        })
    }

    private render = (scores: ListResult<RecordModel>) => {
        while (this.scoreList.firstChild) {
            this.scoreList.removeChild(this.scoreList.firstChild);
        }
        for (let i = 0; i < scores.items.length; i++) {
            const fragment = document.createRange().createContextualFragment(this.highScoreRowTemplate.innerHTML);
            const row = fragment.querySelector('li')!;

            // Update values
            row.querySelector('[data-label="number"]')!.textContent = (i + 1).toString();
            row.querySelector('[data-label="name"]')!.textContent = scores.items[i].player;
            row.querySelector('[data-label="score"]')!.textContent = scores.items[i].score;

            // Append to your list
            this.scoreList.appendChild(row);
        }
        this.showHighScoreList()

    }

    private savedPlayerScore() {
        this.saveScoreForm.style.display = 'none'
        this.gameOverSavedGameText.style.display = 'block'
    }

    private showSavePlayerScore() {
        this.saveScoreForm.style.display = 'flex'
        this.gameOverSavedGameText.style.display = 'none'
    }

    private renderFail() {
        this.scoreList.style.display = "none";
        this.failText.style.display = 'block';
        this.loading.style.display = 'none';
    }

    private hideHighScoreList() {
        this.scoreList.style.display = "none";
        this.failText.style.display = 'none';
        this.loading.style.display = 'block';
    }

    private showHighScoreList() {
        this.scoreList.style.display = "flex";
        this.failText.style.display = 'none';
        this.loading.style.display = 'none';
    }
}
