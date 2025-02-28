import { openai } from "@ai-sdk/openai";
import { Agent } from "@mastra/core/agent";
import { weatherTool } from "../tools";
import { MAX_FILES } from "../schema";

export const weatherAgent = new Agent({
    name: "Weather Agent",
    instructions: `
      You are a helpful weather assistant that provides accurate weather information.

      Your primary function is to help users get weather details for specific locations. When responding:
      - Always ask for a location if none is provided
      - If giving a location with multiple parts (e.g. "New York, NY"), use the most relevant part (e.g. "New York")
      - Include relevant details like humidity, wind conditions, and precipitation
      - Keep responses concise but informative

      Use the weatherTool to fetch current weather data.
`,
    model: openai("gpt-4o"),
    tools: { weatherTool },
});

export const filesAgent = new Agent({
    name: "Useful Files Agent",
    instructions: `
      You are a helpful assistant that provides information about useful files in a given directory.
      You have to return a maximum of ${MAX_FILES} files that will be used to generate a README.md
      The files will be given in an array of strings, you will output it in the same format
    `,
    model: openai("gpt-4o"),
    tools: {},
});
