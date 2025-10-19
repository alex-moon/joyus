import {Component} from "../component";

export class Questions extends Component {
    public name = 'Test';

    static get observedAttributes() {
        return ['name'];
    }
}
window.customElements.define('app-questions', Questions);