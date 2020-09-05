import { readable, writable } from "svelte/store";

import { date_to_timestamp } from "./date";

export const show_contact_modal = writable(false);
export const show_theme_modal = writable(false);
export const time = readable(date_to_timestamp(new Date()), (set) => {
    const interval = setInterval(() => {
        set(date_to_timestamp(new Date()));
    }, 1000);

    return function stop() {
        clearInterval(interval);
    };
});
