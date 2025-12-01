require('./styles.scss');
require('@bundles/datastar');
require('./component');

import {LocationService} from "./service/location";
document.addEventListener('DOMContentLoaded', () => {
    const location = new LocationService();
    global.service = {
        location,
        watch: location.start(),
    };
});
