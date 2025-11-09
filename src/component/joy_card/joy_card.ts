import {Component} from "../component";
import {computed} from "@engine/signals";

export class JoyCard extends Component {
    protected signals = {
        joy: '',
        created: 0,
        createdDate: computed(() => {
            return new Date(this.signals.created);
        }),
        createdFormatted: computed(() => {
            return this.signals.createdDate.toLocaleString();
        }),
    }
}
window.customElements.define('app-joy-card', JoyCard);
