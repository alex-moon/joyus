import {apply} from '@engine';
import {DATASTAR_ELEMENT_PATCH_EVENT} from "@engine/consts";
import type {DatastarElementPatchEvent} from "@engine/types";

const scripts = new WeakSet<HTMLScriptElement>()
for (const script of document.querySelectorAll('script')) {
    scripts.add(script)
}

const execute = (target: Element): void => {
    const elScripts =
        target instanceof HTMLScriptElement
            ? [target]
            : target.querySelectorAll('script')
    for (const old of elScripts) {
        if (!scripts.has(old)) {
            const script = document.createElement('script')
            for (const { name, value } of old.attributes) {
                script.setAttribute(name, value)
            }
            script.text = old.text
            old.replaceWith(script)
            scripts.add(script)
        }
    }
}

export class Component extends HTMLElement {
    protected signals = {};

    connectedCallback() {
        if (!this.shadowRoot) {
            const root = this.attachShadow({ mode: 'open' });
            const template = this.querySelector('template[shadowrootmode="open"]');
            if (template) {
                root.appendChild((template as any).content.cloneNode(true));
            }
        }
        apply(this);
        document.addEventListener(DATASTAR_ELEMENT_PATCH_EVENT, this.handlePatchEvent.bind(this));
    }

    disconnectedCallback() {
        document.removeEventListener(DATASTAR_ELEMENT_PATCH_EVENT, this.handlePatchEvent.bind(this));
    }

    handlePatchEvent(event: CustomEvent<DatastarElementPatchEvent>) {
        if (event.detail.id === this.id) {
            this.applyPatch(event.detail.element);
        }
    }

    applyPatch(element: Element) {
        const cloned = element.cloneNode(true) as Element
        execute(cloned)
        this.replaceWith(cloned);
    }
}
