import {Component} from "../component";

export class Questions extends Component {
    protected getData() {
        return {
            currentStep: -1,
            answers: ['', '', '']
        };
    }

    public checkLength() {
    }
}
window.customElements.define('app-questions', Questions);