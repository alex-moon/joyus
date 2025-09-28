import {apply, load} from '../datastar/src';
import {
    Attr,
    Bind,
    Class,
    Computed,
    DELETE, Effect,
    GET, Indicator, JsonSignals, On, OnIntersect, OnInterval, OnLoad, OnSignalPatch,
    PATCH,
    PatchElements,
    PatchSignals, Peek,
    POST,
    PUT, Ref, SetAll, Show, Signals, Style, Text, ToggleAll
} from "../datastar/src/plugins";

export class Component extends HTMLElement {
    protected getData() {
        return {};
    }

    connectedCallback() {
        if (!this.shadowRoot) {
            throw new Error('Expected shadowRoot to be present');
        }
        this.bind();
    }

    bind() {
        const wrapper = document.createElement('div');
        while (this.shadowRoot.firstChild) {
            wrapper.appendChild(this.shadowRoot.firstChild);
        }

        this.shadowRoot.appendChild(wrapper);
        wrapper.dataset.signals = JSON.stringify(this.getData());
        apply(wrapper as HTMLElement);
        (window as any).ctrl[this.id] = this;
    }
}
