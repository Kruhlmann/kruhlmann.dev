import { ContainerInfo } from "dockerode";

import config from "../../config/config.json";

/* eslint-disable @typescript-eslint/no-var-requires */
/* eslint-disable node/no-unpublished-require */
const Docker = require("dockerode");

const docker_connection = new Docker({ socketPath: "/var/run/docker.sock" });

/**
 * Retrieves a list of registred containers.
 *
 * @returns - List of container information objects.
 */
export async function get_containers(): Promise<ContainerInfo[]> {
    return new Promise((resolve, reject) => {
        docker_connection.listContainers(
            (error: unknown, containers: ContainerInfo[]) => {
                if (error) {
                    reject(error);
                }
                const public_containers = containers.filter((container) => {
                    const container_name = container.Names[0].slice(1);
                    return config.public_containers.includes(container_name);
                });
                resolve(public_containers);
            },
        );
    });
}
