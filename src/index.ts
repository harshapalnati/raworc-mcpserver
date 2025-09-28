import "dotenv/config";
import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";

// Config schema users can provide via Smithery session config
export const configSchema = z.object({
  apiKey: z.string().describe("Raworc API token").optional(),
  apiUrl: z
    .string()
    .url()
    .default("https://ra-hyp-1.raworc.com")
    .describe("Raworc API base URL"),
  timeoutSeconds: z
    .number()
    .int()
    .positive()
    .default(30)
    .describe("Request timeout in seconds"),
});

type Cfg = z.infer<typeof configSchema>;

function buildHeaders(cfg: Cfg): HeadersInit {
  const token = cfg.apiKey || process.env.RAWORC_AUTH_TOKEN;
  const headers: Record<string, string> = { "Content-Type": "application/json" };
  if (token) headers["Authorization"] = `Bearer ${token}`;
  return headers;
}

function buildUrl(base: string, path: string, query?: Record<string, unknown>): string {
  const url = new URL(path.replace(/\n/g, "").trim().replace(/\s+/g, ""), base);
  if (query) {
    Object.entries(query).forEach(([k, v]) => {
      if (v === undefined || v === null || v === "") return;
      url.searchParams.set(k, String(v));
    });
  }
  return url.toString();
}

async function http<T = unknown>(
  cfg: Cfg,
  method: string,
  path: string,
  opts?: { query?: Record<string, unknown>; body?: unknown; raw?: boolean }
): Promise<T | string | Uint8Array> {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), (cfg.timeoutSeconds ?? 30) * 1000);
  try {
    const url = buildUrl(cfg.apiUrl, path, opts?.query);
    const res = await fetch(url, {
      method,
      headers: buildHeaders(cfg),
      body: opts?.body !== undefined ? JSON.stringify(opts.body) : undefined,
      signal: controller.signal,
    });
    if (!res.ok) {
      const text = await res.text().catch(() => "");
      throw new Error(`${res.status} ${res.statusText}: ${text}`);
    }
    if (opts?.raw) {
      const buf = new Uint8Array(await res.arrayBuffer());
      return buf;
    }
    const ct = res.headers.get("content-type") || "";
    if (ct.includes("application/json")) {
      return (await res.json()) as T;
    }
    return (await res.text()) as unknown as T;
  } finally {
    clearTimeout(timeout);
  }
}

function asTextContent(data: unknown): { content: { type: "text"; text: string }[] } {
  const text = typeof data === "string" ? data : JSON.stringify(data, null, 2);
  return { content: [{ type: "text", text }] };
}

export default function createServer({ config }: { config: Cfg }) {
  const cfg: Cfg = {
    apiKey: config.apiKey || process.env.RAWORC_AUTH_TOKEN,
    apiUrl: config.apiUrl || process.env.RAWORC_API_URL || "https://ra-hyp-1.raworc.com",
    timeoutSeconds: config.timeoutSeconds || (process.env.RAWORC_TIMEOUT ? Number(process.env.RAWORC_TIMEOUT) : 30),
  };

  const server = new McpServer({ name: "raworc-mcp", version: "0.1.0" });

  // Version
  server.registerTool(
    "version",
    { description: "Get API version", inputSchema: {} },
    async () => asTextContent(await http(cfg, "GET", "/api/v0/version"))
  );

  // Agents - list
  server.registerTool(
    "agents_list",
    {
      description: "List/search agents",
      inputSchema: {
        q: z.string().optional(),
        tags: z.string().optional(),
        state: z.string().optional(),
        limit: z.number().int().optional(),
        page: z.number().int().optional(),
        offset: z.number().int().optional(),
      },
    },
    async ({ q, tags, state, limit, page, offset }) =>
      asTextContent(
        await http(cfg, "GET", "/api/v0/agents", { query: { q, tags, state, limit, page, offset } })
      )
  );

  // Agents - create
  server.registerTool(
    "agents_create",
    {
      description: "Create agent",
      inputSchema: {
        name: z.string(),
        description: z.string().nullable().optional(),
        metadata: z.record(z.any()).optional(),
        tags: z.array(z.string()).optional(),
        secrets: z.record(z.string()).optional(),
        instructions: z.string().nullable().optional(),
        setup: z.string().nullable().optional(),
        prompt: z.string().nullable().optional(),
        idle_timeout_seconds: z.number().int().nullable().optional(),
        busy_timeout_seconds: z.number().int().nullable().optional(),
      },
    },
    async (args) => asTextContent(await http(cfg, "POST", "/api/v0/agents", { body: args }))
  );

  // Agents - get
  server.registerTool(
    "agents_get",
    { description: "Get agent by name", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}`))
  );

  // Agents - update
  server.registerTool(
    "agents_update",
    {
      description: "Update agent",
      inputSchema: {
        name: z.string(),
        metadata: z.record(z.any()).nullable().optional(),
        description: z.string().nullable().optional(),
        tags: z.array(z.string()).nullable().optional(),
        idle_timeout_seconds: z.number().int().nullable().optional(),
        busy_timeout_seconds: z.number().int().nullable().optional(),
      },
    },
    async ({ name, ...rest }) =>
      asTextContent(await http(cfg, "PUT", `/api/v0/agents/${encodeURIComponent(name)}`, { body: rest }))
  );

  // Agents - state
  server.registerTool(
    "agents_update_state",
    { description: "Update agent state", inputSchema: { name: z.string(), state: z.string() } },
    async ({ name, state }) =>
      asTextContent(await http(cfg, "PUT", `/api/v0/agents/${encodeURIComponent(name)}/state`, { body: { state } }))
  );

  server.registerTool(
    "agents_busy",
    { description: "Set agent busy", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/busy`))
  );

  server.registerTool(
    "agents_idle",
    { description: "Set agent idle", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/idle`))
  );

  server.registerTool(
    "agents_sleep",
    {
      description: "Schedule agent sleep",
      inputSchema: { name: z.string(), delay_seconds: z.number().int().nullable().optional(), note: z.string().nullable().optional() },
    },
    async ({ name, delay_seconds, note }) =>
      asTextContent(
        await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/sleep`, { body: { delay_seconds, note } })
      )
  );

  server.registerTool(
    "agents_cancel",
    { description: "Cancel most recent pending/processing", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/cancel`))
  );

  server.registerTool(
    "agents_wake",
    { description: "Wake agent (optional prompt)", inputSchema: { name: z.string(), prompt: z.string().nullable().optional() } },
    async ({ name, prompt }) =>
      asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/wake`, { body: { prompt } }))
  );

  server.registerTool(
    "agents_runtime",
    { description: "Get total runtime across sessions", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/runtime`))
  );

  server.registerTool(
    "agents_remix",
    {
      description: "Remix agent",
      inputSchema: { name: z.string(), new_name: z.string(), metadata: z.record(z.any()).nullable().optional(), code: z.boolean().optional(), secrets: z.boolean().optional(), content: z.boolean().optional(), prompt: z.string().nullable().optional() },
    },
    async ({ name, new_name, ...rest }) =>
      asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/remix`, { body: { name: new_name, ...rest } }))
  );

  server.registerTool(
    "agents_publish",
    { description: "Publish agent", inputSchema: { name: z.string(), code: z.boolean().optional(), secrets: z.boolean().optional(), content: z.boolean().optional() } },
    async ({ name, ...rest }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/publish`, { body: rest }))
  );

  server.registerTool(
    "agents_unpublish",
    { description: "Unpublish agent", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/unpublish`))
  );

  server.registerTool(
    "agents_delete",
    { description: "Delete agent", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "DELETE", `/api/v0/agents/${encodeURIComponent(name)}`))
  );

  // Responses
  server.registerTool(
    "responses_list",
    { description: "List responses for agent", inputSchema: { name: z.string(), limit: z.number().int().optional(), offset: z.number().int().optional() } },
    async ({ name, limit, offset }) =>
      asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/responses`, { query: { limit, offset } }))
  );

  server.registerTool(
    "responses_create",
    { description: "Create a response (user input)", inputSchema: { name: z.string(), input: z.any(), background: z.boolean().optional() } },
    async ({ name, ...body }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/responses`, { body }))
  );

  server.registerTool(
    "responses_get",
    { description: "Get response by id", inputSchema: { name: z.string(), id: z.string() } },
    async ({ name, id }) => asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/responses/${encodeURIComponent(id)}`))
  );

  server.registerTool(
    "responses_update",
    { description: "Update response", inputSchema: { name: z.string(), id: z.string(), status: z.string().optional(), input: z.any().optional(), output: z.any().optional() } },
    async ({ name, id, ...rest }) =>
      asTextContent(await http(cfg, "PUT", `/api/v0/agents/${encodeURIComponent(name)}/responses/${encodeURIComponent(id)}`, { body: rest }))
  );

  server.registerTool(
    "responses_count",
    { description: "Get response count", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/responses/count`))
  );

  // Files
  server.registerTool(
    "files_list_root",
    { description: "List /agent root", inputSchema: { name: z.string(), offset: z.number().int().optional(), limit: z.number().int().optional() } },
    async ({ name, offset, limit }) =>
      asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/files/list`, { query: { offset, limit } }))
  );

  server.registerTool(
    "files_list_path",
    { description: "List children under relative path", inputSchema: { name: z.string(), path: z.string(), offset: z.number().int().optional(), limit: z.number().int().optional() } },
    async ({ name, path, offset, limit }) =>
      asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/files/list/${encodeURIComponent(path)}`, { query: { offset, limit } }))
  );

  server.registerTool(
    "files_metadata",
    { description: "Get file/directory metadata", inputSchema: { name: z.string(), path: z.string() } },
    async ({ name, path }) =>
      asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/files/metadata/${encodeURIComponent(path)}`))
  );

  server.registerTool(
    "files_read",
    { description: "Read file bytes (base64)", inputSchema: { name: z.string(), path: z.string() } },
    async ({ name, path }) => {
      const bytes = (await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/files/read/${encodeURIComponent(path)}`, { raw: true })) as Uint8Array;
      const b64 = Buffer.from(bytes).toString("base64");
      return asTextContent({ base64: b64 });
    }
  );

  server.registerTool(
    "files_delete",
    { description: "Delete file or empty directory", inputSchema: { name: z.string(), path: z.string() } },
    async ({ name, path }) =>
      asTextContent(await http(cfg, "DELETE", `/api/v0/agents/${encodeURIComponent(name)}/files/delete/${encodeURIComponent(path)}`))
  );

  // Context
  server.registerTool(
    "context_get",
    { description: "Get context usage", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/context`))
  );

  server.registerTool(
    "context_clear",
    { description: "Clear context (set new cutoff)", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/context/clear`))
  );

  server.registerTool(
    "context_compact",
    { description: "Compact context via LLM", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/context/compact`))
  );

  // Published agents (public)
  server.registerTool(
    "published_agents_list",
    { description: "List all published agents", inputSchema: {} },
    async () => asTextContent(await http(cfg, "GET", "/api/v0/published/agents"))
  );

  server.registerTool(
    "published_agents_get",
    { description: "Get published agent by name", inputSchema: { name: z.string() } },
    async ({ name }) => asTextContent(await http(cfg, "GET", `/api/v0/published/agents/${encodeURIComponent(name)}`))
  );

  return server.server;
}


