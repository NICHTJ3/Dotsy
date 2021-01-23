import * as z from 'zod';

export const globalConfig = z.object({
  packages_install_cmd: z.string().default('sudo apt install'),
  packages_uninstall_cmd: z.string().default('sudo apt remove'),
});

export type GlobalConfigType = z.infer<typeof globalConfig>;
