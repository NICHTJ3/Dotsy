import * as z from 'zod';

export const profileCommandOptions = z.object({
  list: z.boolean().optional(),
  install: z.string().array().optional(),
  uninstall: z.string().array().optional(),
});

export type ProfileCommandOptionsType = z.infer<typeof profileCommandOptions>;
