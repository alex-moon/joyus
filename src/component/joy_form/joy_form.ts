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
        showFrustrationNudge: false,
        showContextNudge: false,
        showJoyNudge: false,
        frustrationProgress: 0,
        contextProgress: 0,
        joyProgress: 0,
    };

    private nextTimeout: number | undefined;
    private lastKeystroke: {[key: string]: number} = {
        frustration: 0,
        context: 0,
        joy: 0,
    };
    private nudgeTimers: {[key: string]: number | undefined} = {
        frustration: undefined,
        context: undefined,
        joy: undefined,
    };

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
        
        // Update progress percentage
        if (step === 'frustration') {
            this.signals.frustrationProgress = Math.min(100, (len / this.signals.MAX_LENGTH) * 100);
        } else if (step === 'context') {
            this.signals.contextProgress = Math.min(100, (len / this.signals.MAX_LENGTH) * 100);
        } else if (step === 'joy') {
            this.signals.joyProgress = Math.min(100, (len / this.signals.MAX_LENGTH) * 100);
        }
        
        // Track last keystroke time
        this.lastKeystroke[step] = Date.now();
        
        // Clear existing nudge timer for this step
        if (this.nudgeTimers[step]) {
            clearTimeout(this.nudgeTimers[step]);
        }
        
        // Hide nudge immediately on keystroke
        if (step === 'frustration') {
            this.signals.showFrustrationNudge = false;
        } else if (step === 'context') {
            this.signals.showContextNudge = false;
        } else if (step === 'joy') {
            this.signals.showJoyNudge = false;
        }
        
        // If input is less than minimum, set timer to show nudge after 1 second
        if (len < this.signals.MAX_LENGTH) {
            this.nudgeTimers[step] = window.setTimeout(() => {
                if (step === 'frustration') {
                    this.signals.showFrustrationNudge = true;
                } else if (step === 'context') {
                    this.signals.showContextNudge = true;
                } else if (step === 'joy') {
                    this.signals.showJoyNudge = true;
                }
            }, 1000);
        }
        
        // Auto-advance if minimum reached
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
        
        // Clear nudge timers for the current step
        const step = this.currentStep;
        if (step && this.nudgeTimers[step]) {
            clearTimeout(this.nudgeTimers[step]);
            this.nudgeTimers[step] = undefined;
        }

        if (this.signals.currentStep < STEPS.length - 1) {
            this.signals.currentStep++;
        }

        const nextStep = this.currentStep;
        if (nextStep) {
            const el = this.shadowRoot?.getElementById(nextStep);
            el?.focus();
        }
    }
}
window.customElements.define('app-joy-form', JoyForm);
