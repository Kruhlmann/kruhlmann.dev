WIP BattleNet NodeJS API.

## Usage

    import { BattleNetClient, BattleNetOauthService } from "battlenetjs";

    async function main(): Promise<void> {
        const service = new BattleNetOauthService("client_id", "client_secret");
        const namespace = new BattleNetNamespace(
            BattleNetBaseNamespace.WOW_CLASSIC_PROGRESSION,
            BattleNetRegion.NORTH_AMERICA,
        );
        const client = new BattleNetClient(service, namespace);
        const item = await client.get_item_by_id(19_019);
        console.log(item.name);
        const item2 = await client.get_item_by_name("Thunderfury");
        console.log(item2.name);
    }

Outputs

    Thunderfury, Blessed Blade of the Windseeker
    Thunderfury, Blessed Blade of the Windseeker
