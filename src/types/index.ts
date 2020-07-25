/** Items contained in the technologies section of the config file. */
export interface Technology {
    name: string;
    link: string;
    show_on_index: boolean;
}

/** Svelte custom event abbreviation. */
export type SvelteEvent<T> = Event & { detail?: T };
