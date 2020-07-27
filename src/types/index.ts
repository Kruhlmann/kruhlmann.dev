/** Items contained in the technologies section of the config file. */
export interface Technology {
    name: string;
    link: string;
    show_on_index: boolean;
}

/** Svelte custom event abbreviation. */
export type SvelteEvent<T> = Event & { detail?: T };

/** Config activities item. */
export type TimelineItem = {
    date: {
        year: number;
        month: number;
        day?: number;
    };
    type: string;
    title: string;
    description: string;
    position: string;
    technologies: string[];
};
