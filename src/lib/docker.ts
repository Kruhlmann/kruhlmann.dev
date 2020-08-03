import { ContainerInfo } from "dockerode";

// eslint-disable-line typescript-eslint/no-var-requires //
const Docker = require("dockerode");

const docker_connection = new Docker({ socketPath: "/var/run/docker.sock" });

export async function get_containers(): Promise<string[]> {
    return new Promise((resolve, reject) => {
        docker_connection.listContainers(
            (error: unknown, containers: ContainerInfo[]) => {
                if (error) {
                    reject(error);
                }
                const container_names = containers.map((container) => {
                    return container.Names[0].slice(1);
                });
                resolve(container_names);
            },
        );
    });
}
