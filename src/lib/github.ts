import fetch, { Response } from "node-fetch";

type Repository = {
    language: string | undefined;
};

/**
 * Extracts the language data for a list of repositories.
 *
 * @param repos - List of repositories from GitHub API.
 * @returns - A list of languages used. Undefined language projects are
 * discarded.
 */
function extract_repository_languages(repos: Repository[]): string[] {
    return repos
        .map((repo: Repository) => {
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
 * @param repos - List of repositories from GitHub API.
 * @returns Key value pair with amount of projects using each language.
 */
function get_language_breakdown(repos: Repository[]): Record<string, number> {
    const language_breakdown: Record<string, number> = {};
    const languages = extract_repository_languages(repos);

    for (const language of languages) {
        if (!Object.keys(language_breakdown).includes(language)) {
            language_breakdown[language] = 0;
        }
        language_breakdown[language]++;
    }

    return language_breakdown;
}

/**
 * Fetches the repository language data for a GitHub user using the GitHub API.
 *
 * @param user - Username to get language data for.
 * @returns - Key value pair with amount of projects using each language.
 */
async function get_user_repository_data(user: string): Promise<Repository[]> {
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
): Promise<Record<string, number>> {
    return get_user_repository_data(user).then((repositories: Repository[]) => {
        return get_language_breakdown(repositories);
    });
}
