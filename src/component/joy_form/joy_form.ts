import {Component} from "../component";

type StepName = 'start' | 'frustration' | 'context' | 'joy';

const STEPS: string[] = [
    'start',
    'frustration',
    'context',
    'joy',
];

export class JoyForm extends Component {
    protected signals = {
        MAX_LENGTH: 100,
        currentStep: 0,
        frustration: '',
        context: '',
        joy: '',
    };

    private nextTimeout: number | undefined;

    public get currentStep() {
        return STEPS[this.signals.currentStep];
    }

    public is(name: StepName) {
        return this.currentStep === name;
    }

    public checkLength() {
        const step = this.currentStep;
        if (!(step in this.signals)) {
            return;
        }

        const len = (this.signals[step] as string).length;
        if (len >= this.signals.MAX_LENGTH) {
            if (!this.nextTimeout) {
                this.nextTimeout = window.setTimeout(() => this.next(), 1000);
            }
        }
    }

    public next() {
        if (this.nextTimeout) {
            clearTimeout(this.nextTimeout);
            this.nextTimeout = undefined;
        }

        if (this.signals.currentStep < STEPS.length - 1) {
            this.signals.currentStep++;
        }

        const step = this.currentStep;
        if (step) {
            const el = this.shadowRoot?.getElementById(step);
            el?.focus();
        }
    }
}
window.customElements.define('app-joy-form', JoyForm);
