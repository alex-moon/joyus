import {computed, effect, signal} from '@engine/signals'
import type {Effect} from "@engine/types";

export class DateHelper {
    private static readonly INTERVAL = 1000
    private static _now = signal<Date>(new Date())
    private static _timeUpdater: Effect;

    private static ensureTicker() {
        if (!this._timeUpdater) {
            this._timeUpdater = effect(() => {
                setInterval(() => {
                    this._now(new Date())
                }, this.INTERVAL);
            })
        }
    }

    public static computedFormat(dateGetter: () => Date) {
        this.ensureTicker()
        return computed(() => {
            return DateHelper.format(dateGetter(), this._now())
        })
    }

    public static format(date: Date, now: Date): string {
        if (!date) {
            return '';
        }

        const diffMs = now.getTime() - date.getTime()
        const diffMinutes = Math.floor(diffMs / (1000 * 60))

        // Helper to format time (e.g., "11:08")
        const formatTime = (d: Date): string => {
            const h = String(d.getHours()).padStart(2, '0')
            const m = String(d.getMinutes()).padStart(2, '0')
            return `${h}:${m}`
        }

        // Helper to format short date (e.g., "Sat 29 Nov 2025")
        const formatShortDate = (d: Date, includeYear: boolean = false): string => {
            const options: Intl.DateTimeFormatOptions = {
                weekday: 'short',
                day: '2-digit',
                month: 'short',
                year: includeYear ? 'numeric' : undefined,
            }
            return d.toLocaleString('en-US', options).replace(/,/g, '')
        }

        // 1. Relative Time (Up to 30 minutes)
        if (diffMinutes < 1) {
            const diffSeconds = Math.floor(diffMs / 1000)
            const s = diffSeconds === 1 ? '' : 's'
            return diffSeconds < 5 ? 'Just now' : `${diffSeconds} second${s} ago`
        }
        if (diffMinutes < 30) {
            const s = diffMinutes === 1 ? '' : 's'
            return `${diffMinutes} minute${s} ago`
        }

        // Date comparison for Today/Yesterday
        const dateDay = new Date(date.getFullYear(), date.getMonth(), date.getDate())
        const todayDay = new Date(now.getFullYear(), now.getMonth(), now.getDate())
        const diffDaysExact = (todayDay.getTime() - dateDay.getTime()) / (1000 * 60 * 60 * 24)

        // 2. Yesterday's Time ("16:49 Yesterday")
        if (diffDaysExact >= 1 && diffDaysExact < 2) {
            return `${formatTime(date)} Yesterday`
        }

        // 3. Current Year, Older than Yesterday ("16:49 Mon 23 Jan")
        if (date.getFullYear() === now.getFullYear()) {
            return `${formatTime(date)} ${formatShortDate(date, false)}`
        }

        // 4. Older than 1 Year ("16:49 Mon 23 Jan 2025")
        return `${formatTime(date)} ${formatShortDate(date, true)}`
    }
}