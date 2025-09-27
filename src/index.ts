import { McpServer } from "@modelcontextprotocol/sdk/server/mcp.js";
import { z } from "zod";

// Config schema users can provide via Smithery session config
export const configSchema = z.object({
  apiKey: z.string().describe("Raworc API token").optional(),
  apiUrl: z
    .string()
    .url()
    .default("https://api.remoteagent.com/api/v0")
    .describe("Raworc API base URL"),
  defaultSpace: z.string().optional().describe("Default Raworc space"),
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
    apiUrl: config.apiUrl || process.env.RAWORC_API_URL || "https://api.remoteagent.com/api/v0",
    defaultSpace: config.defaultSpace || process.env.RAWORC_DEFAULT_SPACE,
    timeoutSeconds: config.timeoutSeconds || (process.env.RAWORC_TIMEOUT ? Number(process.env.RAWORC_TIMEOUT) : 30),
  };

  const server = new McpServer({ name: "raworc-mcp", version: "0.1.0" });

  // Version
  server.tool("version", {
    description: "Get API version",
    parameters: z.object({}).strict(),
    execute: async () => asTextContent(await http(cfg, "GET", "/api/v0/version")),
  });

  // Agents - list
  server.tool("agents_list", {
    description: "List/search agents",
    parameters: z
      .object({ q: z.string().optional(), tags: z.string().optional(), state: z.string().optional(), limit: z.number().int().optional(), page: z.number().int().optional(), offset: z.number().int().optional() })
      .strict(),
    execute: async ({ q, tags, state, limit, page, offset }) =>
      asTextContent(
        await http(cfg, "GET", "/api/v0/agents", { query: { q, tags, state, limit, page, offset } })
      ),
  });

  // Agents - create
  server.tool("agents_create", {
    description: "Create agent",
    parameters: z
      .object({
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
      })
      .strict(),
    execute: async (args) => asTextContent(await http(cfg, "POST", "/api/v0/agents", { body: args })),
  });

  // Agents - get
  server.tool("agents_get", {
    description: "Get agent by name",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}`)),
  });

  // Agents - update
  server.tool("agents_update", {
    description: "Update agent",
    parameters: z
      .object({
        name: z.string(),
        metadata: z.record(z.any()).nullable().optional(),
        description: z.string().nullable().optional(),
        tags: z.array(z.string()).nullable().optional(),
        idle_timeout_seconds: z.number().int().nullable().optional(),
        busy_timeout_seconds: z.number().int().nullable().optional(),
      })
      .strict(),
    execute: async ({ name, ...rest }) =>
      asTextContent(await http(cfg, "PUT", `/api/v0/agents/${encodeURIComponent(name)}`, { body: rest })),
  });

  // Agents - state
  server.tool("agents_update_state", {
    description: "Update agent state",
    parameters: z.object({ name: z.string(), state: z.string() }).strict(),
    execute: async ({ name, state }) =>
      asTextContent(await http(cfg, "PUT", `/api/v0/agents/${encodeURIComponent(name)}/state`, { body: { state } })),
  });

  server.tool("agents_busy", {
    description: "Set agent busy",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/busy`)),
  });

  server.tool("agents_idle", {
    description: "Set agent idle",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/idle`)),
  });

  server.tool("agents_sleep", {
    description: "Schedule agent sleep",
    parameters: z.object({ name: z.string(), delay_seconds: z.number().int().nullable().optional(), note: z.string().nullable().optional() }).strict(),
    execute: async ({ name, delay_seconds, note }) =>
      asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/sleep`, { body: { delay_seconds, note } })),
  });

  server.tool("agents_cancel", {
    description: "Cancel most recent pending/processing",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/cancel`)),
  });

  server.tool("agents_wake", {
    description: "Wake agent (optional prompt)",
    parameters: z.object({ name: z.string(), prompt: z.string().nullable().optional() }).strict(),
    execute: async ({ name, prompt }) =>
      asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/wake`, { body: { prompt } })),
  });

  server.tool("agents_runtime", {
    description: "Get total runtime across sessions",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/runtime`)),
  });

  server.tool("agents_remix", {
    description: "Remix agent",
    parameters: z
      .object({ name: z.string(), new_name: z.string(), metadata: z.record(z.any()).nullable().optional(), code: z.boolean().optional(), secrets: z.boolean().optional(), content: z.boolean().optional(), prompt: z.string().nullable().optional() })
      .strict(),
    execute: async ({ name, new_name, ...rest }) =>
      asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/remix`, { body: { name: new_name, ...rest } })),
  });

  server.tool("agents_publish", {
    description: "Publish agent",
    parameters: z.object({ name: z.string(), code: z.boolean().optional(), secrets: z.boolean().optional(), content: z.boolean().optional() }).strict(),
    execute: async ({ name, ...rest }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/publish`, { body: rest })),
  });

  server.tool("agents_unpublish", {
    description: "Unpublish agent",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/unpublish`)),
  });

  server.tool("agents_delete", {
    description: "Delete agent",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "DELETE", `/api/v0/agents/${encodeURIComponent(name)}`)),
  });

  // Responses
  server.tool("responses_list", {
    description: "List responses for agent",
    parameters: z.object({ name: z.string(), limit: z.number().int().optional(), offset: z.number().int().optional() }).strict(),
    execute: async ({ name, limit, offset }) =>
      asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/responses`, { query: { limit, offset } })),
  });

  server.tool("responses_create", {
    description: "Create a response (user input)",
    parameters: z
      .object({ name: z.string(), input: z.any(), background: z.boolean().optional() })
      .strict(),
    execute: async ({ name, ...body }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/responses`, { body })),
  });

  server.tool("responses_get", {
    description: "Get response by id",
    parameters: z.object({ name: z.string(), id: z.string() }).strict(),
    execute: async ({ name, id }) => asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/responses/${encodeURIComponent(id)}`)),
  });

  server.tool("responses_update", {
    description: "Update response",
    parameters: z.object({ name: z.string(), id: z.string(), status: z.string().optional(), input: z.any().optional(), output: z.any().optional() }).strict(),
    execute: async ({ name, id, ...rest }) =>
      asTextContent(await http(cfg, "PUT", `/api/v0/agents/${encodeURIComponent(name)}/responses/${encodeURIComponent(id)}`, { body: rest })),
  });

  server.tool("responses_count", {
    description: "Get response count",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/responses/count`)),
  });

  // Files
  server.tool("files_list_root", {
    description: "List /agent root",
    parameters: z.object({ name: z.string(), offset: z.number().int().optional(), limit: z.number().int().optional() }).strict(),
    execute: async ({ name, offset, limit }) =>
      asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/files/list`, { query: { offset, limit } })),
  });

  server.tool("files_list_path", {
    description: "List children under relative path",
    parameters: z.object({ name: z.string(), path: z.string(), offset: z.number().int().optional(), limit: z.number().int().optional() }).strict(),
    execute: async ({ name, path, offset, limit }) =>
      asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/files/list/${encodeURIComponent(path)}`, { query: { offset, limit } })),
  });

  server.tool("files_metadata", {
    description: "Get file/directory metadata",
    parameters: z.object({ name: z.string(), path: z.string() }).strict(),
    execute: async ({ name, path }) =>
      asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/files/metadata/${encodeURIComponent(path)}`)),
  });

  server.tool("files_read", {
    description: "Read file bytes (base64)",
    parameters: z.object({ name: z.string(), path: z.string() }).strict(),
    execute: async ({ name, path }) => {
      const bytes = (await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/files/read/${encodeURIComponent(path)}`, { raw: true })) as Uint8Array;
      const b64 = Buffer.from(bytes).toString("base64");
      return asTextContent({ base64: b64 });
    },
  });

  server.tool("files_delete", {
    description: "Delete file or empty directory",
    parameters: z.object({ name: z.string(), path: z.string() }).strict(),
    execute: async ({ name, path }) =>
      asTextContent(await http(cfg, "DELETE", `/api/v0/agents/${encodeURIComponent(name)}/files/delete/${encodeURIComponent(path)}`)),
  });

  // Context
  server.tool("context_get", {
    description: "Get context usage",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "GET", `/api/v0/agents/${encodeURIComponent(name)}/context`)),
  });

  server.tool("context_clear", {
    description: "Clear context (set new cutoff)",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/context/clear`)),
  });

  server.tool("context_compact", {
    description: "Compact context via LLM",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "POST", `/api/v0/agents/${encodeURIComponent(name)}/context/compact`)),
  });

  // Published agents (public)
  server.tool("published_agents_list", {
    description: "List all published agents",
    parameters: z.object({}).strict(),
    execute: async () => asTextContent(await http(cfg, "GET", "/api/v0/published/agents")),
  });

  server.tool("published_agents_get", {
    description: "Get published agent by name",
    parameters: z.object({ name: z.string() }).strict(),
    execute: async ({ name }) => asTextContent(await http(cfg, "GET", `/api/v0/published/agents/${encodeURIComponent(name)}`)),
  });

  return server.server;
}


