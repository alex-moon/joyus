import {Component} from "../component";
import {computed} from "@engine/signals";
import {DateHelper} from "../../service/date";
import {DistanceHelper} from "../../service/distance";

export class JoyCard extends Component {
    protected signals = {
        joy: '',
        created: 0,
        distance: null,
        createdDate: computed(() => {
            return new Date(this.signals.created);
        }),
        createdFormatted: DateHelper.computedFormat(
            () => this.signals.createdDate,
        ),
        distanceFormatted: computed(() => {
            return DistanceHelper.format(this.signals.distance);
        })
    }
}
window.customElements.define('app-joy-card', JoyCard);
