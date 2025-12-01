export class DistanceHelper {
    private static readonly ONE_MILE = 1609.344;
    public static format(value: number | null) {
        if (value > this.ONE_MILE) {
            const miles = value / this.ONE_MILE;
            if (miles > 1000) {
                return `${(miles / 1000).toFixed(1)}k miles`;
            }
            return `${miles.toFixed(1)} miles`;
        }
        if (value > 1000) {
            return '1 mile';
        }
        if (value < 100) {
            return 'Right here';
        }
        return `${value.toFixed(0)}m`;
    }
}
