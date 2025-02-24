import {invoke} from "@tauri-apps/api/core";
import {z} from 'zod';

export const zodConsulClient = z.object({
    name: z.string(),
    host: z.string(),
    port: z.number(),
    scheme: z.literal('HTTP').or(z.literal('HTTPS')),
});
export type ConsulClient = z.infer<typeof zodConsulClient>;

export class TauriBride {
    async getConsulValues(client: ConsulClient): Promise<object> {
        const rawValue = await invoke('get_consul_values', {
            consulClient: client,
        });

        return z.object({}).passthrough().parse(rawValue);
    }

    async saveConsulValues(
        client: ConsulClient,
        initialValues: object,
        newValues: object,
    ): Promise<object> {
        const rawValue = await invoke('save_consul_values', {
            consulClient: client,
            initialValues,
            newValues,
        });

        return z.object({}).passthrough().parse(rawValue);
    }

    async listServers(): Promise<ConsulClient[]> {
        const rawValue = await invoke('get_servers');

        return z.array(zodConsulClient).parse(rawValue);
    }
}