import { get_containers } from "../../lib/docker";

export async function get(
    _request: unknown,
    responder: {
        end: (json: unknown) => void;
        setHeader: (s1: string, s2: string) => void;
    },
): Promise<void> {
    const container_names = await get_containers();
    responder.setHeader("Content-Type", "application/json");
    responder.end(JSON.stringify(container_names));
}
