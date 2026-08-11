import StateManager from "../StateManager";
import { GetElementById } from "../util";
import Page from "./Page";


export default class CustomGamePage extends Page {

    private form: HTMLFormElement
    private back: HTMLButtonElement

    constructor() {
        super("custom-game-page");

        this.back = GetElementById('custom-game-back') as HTMLButtonElement
        this.form = GetElementById('custom-game-form') as HTMLFormElement

        this.form.addEventListener('submit', this.submit)
        this.back.addEventListener('click', this.goBack)
    }

    goBack = () => StateManager.GetInstance().GoToMainMenu()

    submit = (event: SubmitEvent) => {
        event.preventDefault(); 
        const formData = new FormData(this.form);
        console.log(formData)
    }
}