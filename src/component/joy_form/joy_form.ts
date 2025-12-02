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

    private updateStepProgress(step: string, progress: number) {
        if (step === 'frustration') {
            this.signals.frustrationProgress = progress;
        } else if (step === 'context') {
            this.signals.contextProgress = progress;
        } else if (step === 'joy') {
            this.signals.joyProgress = progress;
        }
    }

    private updateStepNudge(step: string, show: boolean) {
        if (step === 'frustration') {
            this.signals.showFrustrationNudge = show;
        } else if (step === 'context') {
            this.signals.showContextNudge = show;
        } else if (step === 'joy') {
            this.signals.showJoyNudge = show;
        }
    }

    public checkLength() {
        const step = this.currentStep;
        if (!(step in this.signals)) {
            return;
        }

        const len = (this.signals[step] as string).length;
        
        // Update progress percentage
        this.updateStepProgress(step, Math.min(100, (len / this.signals.MAX_LENGTH) * 100));
        
        // Clear existing nudge timer for this step
        if (this.nudgeTimers[step]) {
            clearTimeout(this.nudgeTimers[step]);
        }
        
        // Hide nudge immediately on keystroke
        this.updateStepNudge(step, false);
        
        // If input is less than minimum, set timer to show nudge after 1 second
        if (len < this.signals.MAX_LENGTH) {
            this.nudgeTimers[step] = window.setTimeout(() => {
                this.updateStepNudge(step, true);
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
        const currentStep = this.currentStep;
        if (currentStep && this.nudgeTimers[currentStep]) {
            clearTimeout(this.nudgeTimers[currentStep]);
            this.nudgeTimers[currentStep] = undefined;
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
