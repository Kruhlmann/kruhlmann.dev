import { get_user_languages } from "../../lib/github";

export async function get(
    _request: unknown,
    responder: {
        end: (json: unknown) => void;
        setHeader: (s1: string, s2: string) => void;
    },
): Promise<void> {
    const languages = await get_user_languages("Kruhlmann");
    const languages_response = JSON.stringify(languages);
    responder.setHeader("Content-Type", "application/json");
    responder.end(languages_response);
}
