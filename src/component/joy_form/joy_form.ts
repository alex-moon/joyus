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
        // per-step nudge text (can be "Keep writing!" or "Finish up!")
        frustrationNudgeText: 'Keep writing!',
        contextNudgeText: 'Keep writing!',
        joyNudgeText: 'Keep writing!',
        // per-step fill color for the progress bar (changes when full)
        frustrationFillColor: '#06b6d4',
        contextFillColor: '#06b6d4',
        joyFillColor: '#06b6d4',
        showFrustrationNudge: false,
        showContextNudge: false,
        showJoyNudge: false,
        frustrationProgress: 0,
        contextProgress: 0,
        joyProgress: 0,
    };

    private nextTimeout: number | undefined;
    // timer used to show the "Keep writing!" nudge when below the minimum
    private nudgeTimers: {[key: string]: number | undefined} = {
        frustration: undefined,
        context: undefined,
        joy: undefined,
    };
    // remember the last observed length per step so we can detect crossing the threshold
    private lastLen: {[key: string]: number} = {
        frustration: 0,
        context: 0,
        joy: 0,
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
            // change fill color when full
            this.signals.frustrationFillColor = progress >= 100 ? '#f97316' : '#06b6d4';
        } else if (step === 'context') {
            this.signals.contextProgress = progress;
            this.signals.contextFillColor = progress >= 100 ? '#f97316' : '#06b6d4';
        } else if (step === 'joy') {
            this.signals.joyProgress = progress;
            this.signals.joyFillColor = progress >= 100 ? '#f97316' : '#06b6d4';
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
        // Update progress percentage and fill color
        const pct = Math.min(100, (len / this.signals.MAX_LENGTH) * 100);
        this.updateStepProgress(step, pct);

        // Clear any "Keep writing!" nudge timer for this step (we'll re-schedule below if needed)
        if (this.nudgeTimers[step]) {
            clearTimeout(this.nudgeTimers[step]);
            this.nudgeTimers[step] = undefined;
        }

        // Hide nudge immediately on keystroke; we'll show the appropriate one below
        this.updateStepNudge(step, false);

        const prevLen = this.lastLen[step] ?? 0;
        // Case A: still below required length => schedule "Keep writing!" nudge after 1s
        if (len < this.signals.MAX_LENGTH) {
            // reset the next-step timeout if any, because we're below threshold now
            if (this.nextTimeout) {
                clearTimeout(this.nextTimeout);
                this.nextTimeout = undefined;
            }

            // schedule the "Keep writing!" nudge to appear after 1s of idle
            this.nudgeTimers[step] = window.setTimeout(() => {
                if (step === 'frustration') {
                    this.signals.frustrationNudgeText = 'Keep writing!';
                } else if (step === 'context') {
                    this.signals.contextNudgeText = 'Keep writing!';
                } else if (step === 'joy') {
                    this.signals.joyNudgeText = 'Keep writing!';
                }
                this.updateStepNudge(step, true);
            }, 1000);
        } else {
            // Case B: at-or-above required length
            // We only start/refresh the 1s auto-advance timer if the last keystroke put us at/over the threshold.
            // If prevLen < MAX_LENGTH and len >= MAX_LENGTH -> crossing event, start the countdown.
            // If prevLen >= MAX_LENGTH and user types more, refresh the countdown so it is 1s after last keystroke.
            // If a backspace reduced us below, the countdown will have been cleared above.
            // Show the 'Finish up!' nudge during this countdown.
            if (step === 'frustration') {
                this.signals.frustrationNudgeText = 'Finish up!';
            } else if (step === 'context') {
                this.signals.contextNudgeText = 'Finish up!';
            } else if (step === 'joy') {
                this.signals.joyNudgeText = 'Finish up!';
            }

            // refresh the nextTimeout so countdown is always 1s after the most recent keystroke
            if (this.nextTimeout) {
                clearTimeout(this.nextTimeout);
            }
            this.updateStepNudge(step, true);
            this.nextTimeout = window.setTimeout(() => {
                // If input is still >= MAX_LENGTH after the 1s idle, advance
                const currentLen = (this.signals[step] as string).length;
                if (currentLen >= this.signals.MAX_LENGTH) {
                    this.next();
                } else {
                    // if the user removed characters during the wait, cancel timer
                    if (this.nextTimeout) {
                        clearTimeout(this.nextTimeout);
                        this.nextTimeout = undefined;
                    }
                }
            }, 1000);
        }

        // remember this length for the next keystroke
        this.lastLen[step] = len;
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
        // Reset per-step nudge text and ensure nudge is hidden when advancing
        if (currentStep === 'frustration') {
            this.signals.frustrationNudgeText = 'Keep writing!';
            this.updateStepNudge('frustration', false);
        } else if (currentStep === 'context') {
            this.signals.contextNudgeText = 'Keep writing!';
            this.updateStepNudge('context', false);
        } else if (currentStep === 'joy') {
            this.signals.joyNudgeText = 'Keep writing!';
            this.updateStepNudge('joy', false);
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