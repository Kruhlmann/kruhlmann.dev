import fs from "fs";
import fetch, { Response } from "node-fetch";

import { GitHubRepository, LanguageRecord } from "../types";

const github_cache_file = "github_api_data.json";

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

async function make_file_if_not_exists(filepath: string): Promise<void> {
    return new Promise((resolve, reject) => {
        fs.access(filepath, fs.constants.F_OK | fs.constants.W_OK, (error) => {
            if (error) {
                if (error.code === "ENOENT") {
                    resolve();
                } else {
                    reject(`File ${filepath} is read-only`);
                }
            }
            resolve();
        });
    });
}

async function get_file_timestamp(filepath: string): Promise<Date> {
    await make_file_if_not_exists(filepath);
    const stats = fs.statSync(filepath);
    return stats.mtime;
}

async function get_github_data_local(): Promise<GitHubRepository[]> {
    return new Promise((resolve, reject) => {
        fs.readFile(github_cache_file, (error, filedata) => {
            if (error) {
                reject(error);
            }
            resolve(JSON.parse(filedata.toString()) as GitHubRepository[]);
        });
    });
}

function write_github_cache(repositories: GitHubRepository[]): void {
    console.log("Wrote GitHub cache");
    fs.writeFileSync(github_cache_file, JSON.stringify(repositories));
}

async function get_github_data_remote(
    user: string,
): Promise<GitHubRepository[]> {
    return fetch(`https://api.github.com/users/${user}/repos?per_page=100`)
        .then((response: Response) => {
            return response.json();
        })
        .then((repositories: GitHubRepository[]) => {
            write_github_cache(repositories);
            return repositories;
        })
        .catch((error: Error) => {
            console.warn(`Error while retirving language stats: ${error}`);
            return [];
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
    const last_time_modified = await get_file_timestamp(github_cache_file);
    const ms_delta = last_time_modified.getTime() - new Date().getTime();
    const hour_delta = Math.abs(ms_delta) / 36e5;

    if (hour_delta < 24) {
        console.log("Using locally cached GitHub response");
        return get_github_data_local();
    } else {
        console.log("Cache expiring, fetching remote data.");
        return get_github_data_remote(user);
    }
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
