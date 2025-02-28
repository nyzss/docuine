import { z } from "zod";

export const MAX_FILES = 10;

export const filesSchema = z.object({
    files: z
        .array(z.string())
        .max(MAX_FILES)
        .describe(
            "List of files in the project that will be used to generate a README.md"
        ),
});
