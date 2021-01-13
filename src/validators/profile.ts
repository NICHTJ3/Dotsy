import * as z from 'zod';

export const profile = z.object({
  description: z.string().optional(),
  configs: z.array(z.string()).optional(),
  directories: z.array(z.string()).optional(),
  packages: z.array(z.string()).optional(),
  shell: z.array(z.string()).optional(),
  'revert-shell': z.array(z.string()).optional(),
});

export type ProfileType = z.infer<typeof profile>;
