import { mastra } from "./mastra";
import dotenv from "dotenv";

dotenv.config();

async function main() {
    const agent = mastra.getAgent("weatherAgent");

    const result = await agent.generate("What is the weather in London?");

    console.log("Agent response:", result.text);
}

main();
