import {Component} from "../component";

export class App extends Component {
    public signals = {
        example: 'lol',
    };
}
window.customElements.define('app-app', App);