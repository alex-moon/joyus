import * as Datastar from '../datastar/src/index';

export class Component extends HTMLElement {
    connectedCallback() {
        if (!this.shadowRoot) {
            console.error('Expected shadowRoot to be present');
        }
        try {
            Datastar.apply(this);
        } catch (e) {
            console.error('Failed to apply datastar', e);
        }
    }
}
