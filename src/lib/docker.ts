import { ContainerInfo } from "dockerode";

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
                resolve(containers);
            },
        );
    });
}
