import {apply} from '@engine';

export class Component extends HTMLElement {
    protected signals = {};

    connectedCallback() {
        if (!this.shadowRoot) {
            throw new Error('Expected shadowRoot to be present');
        }
        apply(this);
    }
}
