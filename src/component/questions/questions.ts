import {Component} from "../component";

export class Questions extends Component {
    connectedCallback() {
        super.connectedCallback();
        console.log('questions connected');
    }
}
window.customElements.define('app-questions', Questions);