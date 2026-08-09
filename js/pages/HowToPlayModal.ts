import StateManager from "../StateManager";
import { GetElementById } from "../util";
import Page from "./Page";


export default class HowToPlayModal extends Page {

    private backButton: HTMLButtonElement

    constructor() {
        super("how-to-play-modal");

        this.backButton = GetElementById('how-to-play-back') as HTMLButtonElement

        this.backButton.addEventListener('click', this.goBack)
    }

    goBack = () => this.hide()
}