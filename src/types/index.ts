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
    description?: string;
    position?: string;
    technologies?: string[];
    links?: Record<string, string | undefined>;
};

/**
 * Used for storing information about the language used in github repositories.
 */
export type LanguageRecord = {
    language: string;
    hits: number;
};

/** Stores the optional languageof a GitHub repository. */
export type GitHubRepository = {
    language: string | undefined;
};

/** Format of the language_colors.json configuration file. */
export type LanguageColors = Record<
    string,
    { color?: string | null; url: string }
>;
