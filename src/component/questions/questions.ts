import {Component} from "../component";

export class Questions extends Component {
    protected getData() {
        return {
            currentStep: -1,
            answers: ['', '', '']
        };
    }

    public checkLength() {
        console.log('check length called yay');
    }
}
window.customElements.define('app-questions', Questions);