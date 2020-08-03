declare module "dockerode" {
    interface ContainerInfo {
        Id: string;
        Names: string[];
        Image: string;
        ImageID: string;
        Command: string;
        Created: number;
        Ports: Port[];
        Labels: { [label: string]: string };
        State: string;
        Status: string;
        HostConfig: {
            NetworkMode: string;
        };
        NetworkSettings: {
            Networks: { [networkType: string]: NetworkInfo };
        };
        Mounts: Array<{
            Name?: string;
            Type: string;
            Source: string;
            Destination: string;
            Driver?: string;
            Mode: string;
            RW: boolean;
            Propagation: string;
        }>;
    }
}
