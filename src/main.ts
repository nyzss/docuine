import { mastra } from "./mastra";
import dotenv from "dotenv";
import walk from "ignore-walk";
import { filesSchema } from "./mastra/schema";
dotenv.config();

export const FOLDERS_TO_IGNORE = [
    "node_modules",
    "dist",
    "build",
    "test",
    "tests",
    "tests",
    "CMakeFiles",
];

async function main({ path }: { path?: string } = {}) {
    // const workflow = mastra.getWorkflow("weatherWorkflow");
    // const { start } = workflow.createRun();
    // const { results } = await start({
    //     triggerData: {
    //         city: "Izmir",
    //     },
    // });
    // console.log(results);

    const rawFiles = await walk({
        path: path || "./",
        ignoreFiles: [".gitignore"],
        includeEmpty: false,
        follow: false,
    });

    const files = rawFiles.filter((file) => {
        if (file.startsWith(".")) return false;

        for (const folder of FOLDERS_TO_IGNORE) {
            if (file.startsWith(folder)) return false;
        }

        return true;
    });

    const agent = mastra.getAgent("filesAgent");
    const result = await agent.generate(
        [
            {
                role: "user",
                content: files.join("\n"),
            },
        ],
        {
            output: filesSchema,
        }
    );
    console.log("test3", result.object);
}

main({
    path: "./",
});
