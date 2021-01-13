import * as z from 'zod';

export const config = z.object({
  description: z.string().optional(),
  links: z
    .array(
      z.object({
        from: z.string(),
        to: z.string(),
      }),
    )
    .optional(),
  directories: z.array(z.string()).optional(),
  packages: z.array(z.string()).optional(),
  shell: z.array(z.string()).optional(),
  'revert-shell': z.array(z.string()).optional(),
});

export type ConfigType = z.infer<typeof config>;
