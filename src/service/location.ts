// Define the structure for the payload we send to the backend
interface LocationPayload {
    latitude: number;
    longitude: number;
}

export class LocationService {
    readonly ENDPOINT = '/user';
    readonly WATCH_OPTIONS: PositionOptions = {
        enableHighAccuracy: true, // Use GPS/more precise methods if available
        timeout: 5000,            // Maximum time to wait for a location retrieval
        maximumAge: 5000          // Accept a cached position if it's less than 5 seconds old
    };

    stop(watchId: number | undefined): void {
        if (watchId !== undefined && 'geolocation' in navigator) {
            navigator.geolocation.clearWatch(watchId);
        }
    }

    start(): number | undefined {
        if (!('geolocation' in navigator)) {
            console.error('Geolocation is not supported by this browser.');
            return undefined;
        }

        return navigator.geolocation.watchPosition(
            this.success.bind(this),
            this.error.bind(this),
            this.WATCH_OPTIONS
        );
    }

    private error(error: GeolocationPositionError): void {
        console.error('Failed to locate:', error);
    }

    private success(position: GeolocationPosition): void {
        const { latitude, longitude } = position.coords;
        this.ping(latitude, longitude);
    }

    private async ping(latitude: number, longitude: number): Promise<void> {
        const payload: LocationPayload = { latitude, longitude };

        try {
            const response = await fetch(this.ENDPOINT, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                // IMPORTANT: Ensures the browser sends the 'session_id' cookie
                credentials: 'include',
                body: JSON.stringify(payload),
            });

            if (!response.ok) {
                console.error(
                    'Error response from server',
                    response.status,
                    latitude,
                    longitude
                );
            }
        } catch (error) {
            console.error('Ping failed', error);
        }
    }
}
