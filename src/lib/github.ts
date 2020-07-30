import fetch, { Response } from "node-fetch";

import { GitHubRepository, LanguageRecord } from "../types";

/**
 * Extracts the language data for a list of repositories.
 *
 * @param repositories - List of repositories from GitHub API.
 * @returns - A list of languages used. Undefined language projects are
 * discarded.
 */
function extract_repository_languages(
    repositories: GitHubRepository[],
): string[] {
    return repositories
        .map((repo: GitHubRepository) => {
            return repo.language || "";
        })
        .filter((language: string) => {
            return language !== "";
        });
}

/**
 * Retrieves the language breakdown from a list of repositories gathered from
 * the GitHub API.
 *
 * @param repositories - List of repositories from GitHub API.
 * @returns Key value pair with amount of projects using each language.
 */
function get_language_breakdown(
    repositories: GitHubRepository[],
): Record<string, number> {
    const language_breakdown: Record<string, number> = {};
    const languages = extract_repository_languages(repositories);

    for (const language of languages) {
        if (!Object.keys(language_breakdown).includes(language)) {
            language_breakdown[language] = 0;
        }
        language_breakdown[language]++;
    }

    return language_breakdown;
}

/**
 * Converts an accumulated Record of languages into an array and sorts it
 * descending by hit count.
 *
 * @param languages - Accumulated Record of languages.
 * @returns - Sorted list of language stats.
 */
function sort_languages_as_array(
    languages: Record<string, number>,
): LanguageRecord[] {
    const language_records: LanguageRecord[] = [];
    for (const [language, hits] of Object.entries(languages)) {
        language_records.push({ language, hits });
    }
    return language_records.sort((language_a, language_b) => {
        return language_b.hits - language_a.hits;
    });
}

/**
 * Fetches the repository language data for a GitHub user using the GitHub API.
 *
 * @param user - Username to get language data for.
 * @returns - Key value pair with amount of projects using each language.
 */
async function get_user_repository_data(
    user: string,
): Promise<GitHubRepository[]> {
    return fetch(`https://api.github.com/users/${user}/repos?per_page=100`)
        .then((response: Response) => {
            return response.json();
        })
        .catch((error: Error) => {
            console.warn(`Error while retirving language stats: ${error}`);
            return [];
        });
}

export async function get_user_languages(
    user: string,
): Promise<LanguageRecord[]> {
    return get_user_repository_data(user).then(
        (repositories: GitHubRepository[]) => {
            const language_breakdown = get_language_breakdown(repositories);
            return sort_languages_as_array(language_breakdown);
        },
    );
}
