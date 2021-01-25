import * as z from 'zod';

export const configCommandOptions = z.object({
  list: z.boolean().default(false),
  install: z.string().array().optional(),
  uninstall: z.string().array().optional(),
});

export type ConfigCommandOptionsType = z.infer<typeof configCommandOptions>;
