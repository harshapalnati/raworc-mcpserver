# RemoteAgent REST API (ra-hyp-1)

Public documentation of REST endpoints. Interactive pages require login.

- Base URL: `https://ra-hyp-1.raworc.com`
- API Namespace: `v0`
- Version: `0.10.9 (v0)`

## Version

GET `/api/v0/version`

Public. Get API version and current version string.

Example:

```
curl -s https://ra-hyp-1.raworc.com/api/v0/version
```

Response 200:

```
{
  "version": "0.x.y",
  "api": "v0"
}
```

Schema: Version
- version: string — Semantic version of server
- api: string — API namespace (e.g., 'v0')

## Agents

Agent lifecycle and management endpoints (protected).

GET `/api/v0/agents`

Bearer. List/search agents with pagination.

Query parameters:
- q: string — Search substring over name and description (case-insensitive)
- tags: string (comma-separated) — Filter by tags (AND). e.g., `tags=prod,team` (case-insensitive, stored lowercase)
- state: string — Filter by state: `init|idle|busy|slept`
- limit: int — Page size (default 30, max 100)
- page: int — Page number (1-based). Ignored when `offset` is set.
- offset: int — Row offset (0-based). Takes precedence over page.

Example:

```
curl -s \
  -H "Authorization: Bearer <token>" \
  "https://ra-hyp-1.raworc.com/api/v0/agents?q=demo&tags=prod,team&state=idle&limit=30&page=1"
```

Response 200:

```
{"items":[{"name":"demo","created_by":"admin","state":"idle","description":"Demo agent","parent_agent_name":null,"created_at":"2025-01-01T12:00:00Z","last_activity_at":"2025-01-01T12:10:00Z","metadata":{},"tags":["prod","team"],"is_published":false,"published_at":null,"published_by":null,"publish_permissions":{"code":true,"secrets":true,"content":true},"idle_timeout_seconds":300,"busy_timeout_seconds":3600,"idle_from":"2025-01-01T12:10:00Z","busy_from":null}],"total":1,"limit":30,"offset":0,"page":1,"pages":1}
```

Schema: ListAgentsResult
- items: Agent[] — Array of agents for current page
- total: int — Total agents matching filters
- limit: int — Page size
- offset: int — Row offset (0-based)
- page: int — Current page number (1-based)
- pages: int — Total page count

POST `/api/v0/agents`

Bearer. Create agent.

Body fields:
- name: string (required) — Must match `^[A-Za-z][A-Za-z0-9-]{0,61}[A-Za-z0-9]$`
- description: string|null — Optional human-readable description
- metadata: object — Arbitrary JSON metadata (default `{}`)
- tags: string[] — Array of alphanumeric tags (default `[]`)
- secrets: object<string,string> — Key/value secrets map (default empty)
- instructions: string|null — Optional instructions
- setup: string|null — Optional setup script or commands
- prompt: string|null — Optional initial prompt
- idle_timeout_seconds: int|null — Idle timeout seconds (default 300)
- busy_timeout_seconds: int|null — Busy timeout seconds (default 3600)

Example:

```
curl -s \
  -X POST \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"name":"demo","description":"Demo agent"}' \
  https://ra-hyp-1.raworc.com/api/v0/agents
```

Response 200: Agent

GET `/api/v0/agents/{name}`

Bearer. Get agent by name.

Path:
- name: string (required)

Example:

```
curl -s \
  -H "Authorization: Bearer <token>" \
  https://ra-hyp-1.raworc.com/api/v0/agents/<name>
```

Response 200: Agent

PUT `/api/v0/agents/{name}`

Bearer. Update agent by name.

Body fields:
- metadata: object|null — Replace metadata (omit to keep)
- description: string|null — Update description
- tags: string[]|null — Replace tags array
- idle_timeout_seconds: int|null — Update idle timeout seconds
- busy_timeout_seconds: int|null — Update busy timeout seconds

Example:

```
curl -s \
  -X PUT \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"description":"Updated"}' \
  https://ra-hyp-1.raworc.com/api/v0/agents/<name>
```

Response 200: Agent

PUT `/api/v0/agents/{name}/state`

Bearer. Update agent state (generic).

Body:
- state: string (required) — `init|idle|busy|slept`

Example:

```
curl -s \
  -X PUT \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"state":"idle"}' \
  https://ra-hyp-1.raworc.com/api/v0/agents/<name>/state
```

Response 200: `{ "success": true, "state": "idle" }`

POST `/api/v0/agents/{name}/busy`

Bearer. Set agent busy.

Response 200: `{ "success": true, "state": "busy", "timeout_status": "paused" }`

POST `/api/v0/agents/{name}/idle`

Bearer. Set agent idle.

Response 200: `{ "success": true, "state": "idle", "timeout_status": "active" }`

POST `/api/v0/agents/{name}/sleep`

Bearer. Schedule agent to sleep after an optional delay (min/default 5s).

Body:
- delay_seconds: int|null — Delay before sleeping (min/default 5)
- note: string|null — Optional note to display in chat when sleep occurs

Example:

```
curl -s \
  -X POST \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"delay_seconds":10,"note":"User requested sleep"}' \
  https://ra-hyp-1.raworc.com/api/v0/agents/<name>/sleep
```

Response 200: Agent (state may not change immediately)

POST `/api/v0/agents/{name}/cancel`

Bearer. Cancel the most recent pending/processing response (or queued task) and set agent to idle.

Response 200: `{ "status":"ok", "agent":"demo", "cancelled":true }`

POST `/api/v0/agents/{name}/wake`

Bearer. Wake agent (optionally send a prompt).

Body:
- prompt: string|null — Optional prompt to send on wake

Response 200: Agent

GET `/api/v0/agents/{name}/runtime`

Bearer. Get total runtime across sessions (seconds). Includes current session.

Response 200:

```
{"agent_name":"demo","total_runtime_seconds":1234,"current_session_seconds":321}
```

POST `/api/v0/agents/{name}/remix`

Bearer. Remix agent (create a new agent from parent).

Body:
- name: string (required) — New agent name (same regex as create)
- metadata: object|null — Optional metadata override
- code: boolean — Copy code (default true)
- secrets: boolean — Copy secrets (default true)
- content: boolean — Copy content (always true in v0.4.0+)
- prompt: string|null — Optional initial prompt

Response 200: Agent

POST `/api/v0/agents/{name}/publish`

Bearer. Publish agent.

Body:
- code: boolean — Allow code remix (default true)
- secrets: boolean — Allow secrets remix (default true)
- content: boolean — Publish content (default true)

Response 200: Agent

POST `/api/v0/agents/{name}/unpublish`

Bearer. Unpublish agent.

Response 200: Agent

DELETE `/api/v0/agents/{name}`

Bearer. Delete agent.

Response 200: No JSON body.

## Agent Responses

Composite input→output exchanges with live items (protected).

GET `/api/v0/agents/{name}/responses`

Bearer. List responses for agent.

Query parameters:
- limit: int — Max responses (0..1000, default 100)
- offset: int — Offset for pagination (default 0)

Response 200:

```
[{"id":"uuid","agent_name":"demo","status":"completed","input_content":[{"type":"text","content":"hi"}],"output_content":[{"type":"text","content":"hello"}],"segments":[{"type":"final","channel":"final","text":"hello"}],"created_at":"2025-01-01T12:00:00Z","updated_at":"2025-01-01T12:00:10Z"}]
```

POST `/api/v0/agents/{name}/responses`

Bearer. Create a response (user input). Supports blocking when `background=false`.

Body:
- input: object (required) — Preferred shape: `{ content: [{ type: 'text', content: string }] }`. Legacy `{ text: string }` also accepted.
- background: boolean — Default true. If false, request blocks up to 15 minutes until response reaches terminal status; returns 504 on timeout.

Response 200: ResponseObject

Response 504:

```
{"message":"Timed out waiting for response to complete"}
```

GET `/api/v0/agents/{name}/responses/{id}`

Bearer. Get a single response by id.

Response 200: ResponseObject

PUT `/api/v0/agents/{name}/responses/{id}`

Bearer. Update a response (append output items and/or mark status).

Body:
- status: 'pending'|'processing'|'completed'|'failed' — Status update
- input: object — Optional input update; replaces existing input JSON
- output: object — Output update; shape `{ text?: string, items?: [] }`

Response 200: ResponseObject

GET `/api/v0/agents/{name}/responses/count`

Bearer. Get response count for agent.

Response 200:

```
{"count":123,"agent_name":"demo"}
```

## Agent Files

Read-only browsing of an agent's `/agent` workspace (protected). Paths are relative to `/agent`.

GET `/api/v0/agents/{name}/files/list`

Bearer. List immediate children at `/agent` (root). Sorted by name (case-insensitive). Supports pagination with `offset`+`limit` and returns `total` and `next_offset`.

Response 200:

```
{"entries":[{"name":"code","kind":"dir","size":0,"mode":"0755","mtime":"2025-01-01T12:00:00Z"}],"offset":0,"limit":100,"next_offset":null,"total":1}
```

GET `/api/v0/agents/{name}/files/list/{path...}`

Bearer. List immediate children under a relative path (e.g., `code/src`). Sorted by name (case-insensitive). Supports pagination. Path must be safe (no leading '/', no `..`).

Response 200:

```
{"entries":[{"name":"main.rs","kind":"file","size":1024,"mode":"0644","mtime":"2025-01-01T12:00:00Z"}],"offset":0,"limit":100,"next_offset":null,"total":1}
```

GET `/api/v0/agents/{name}/files/metadata/{path...}`

Bearer. Get metadata for a file or directory. For symlinks, includes `link_target`. Returns 409 if the agent is sleeping; 400 for invalid paths; 404 if not found.

Response 200:

```
{"kind":"file","size":1024,"mode":"0644","mtime":"2025-01-01T12:00:00Z"}
```

GET `/api/v0/agents/{name}/files/read/{path...}`

Bearer. Read a file and return its raw bytes. Sets `Content-Type` (guessed by filename) and `X-Raworc-File-Size` headers. Max size 25MB; larger files return 413. Returns 409 if sleeping; 404 if not found; 400 for invalid paths.

Response 200: No JSON body (raw bytes)

DELETE `/api/v0/agents/{name}/files/delete/{path...}`

Bearer. Delete a file or empty directory. Returns `{ deleted: true }` on success. May be disabled in some environments.

Response 200: `{ "deleted": true }`

## Agent Context

Context usage and management (protected).

GET `/api/v0/agents/{name}/context`

Bearer. Get estimated context usage since the current cutoff (if any).

Response 200:

```
{"agent":"demo","soft_limit_tokens":100000,"used_tokens_estimated":12345,"used_percent":12.3,"basis":"estimated_from_history_chars","cutoff_at":"2025-01-01T12:34:56Z","measured_at":"2025-01-01T13:00:00Z","total_messages_considered":42}
```

POST `/api/v0/agents/{name}/context/clear`

Bearer. Clear context by setting a new cutoff at now. Adds a "Context Cleared" marker response.

Response 200: AgentContextUsage (with zeroed usage)

POST `/api/v0/agents/{name}/context/compact`

Bearer. Compact context by summarizing recent conversation via LLM and setting a new cutoff. Adds a "Context Compacted" marker response with the summary in `output.text`.

Response 200: AgentContextUsage

## Published Agents (Public)

Publicly visible agents and details.

GET `/api/v0/published/agents`

Public. List all published agents.

Response 200: Agent[]

GET `/api/v0/published/agents/{name}`

Public. Get details of a published agent by name.

Response 200: Agent

## Error Format

On error, endpoints return an HTTP status and a JSON body:

```
{ "message": "Error description" }
```

## Response Object (Responses APIs)

Standard object returned by `/api/v0/agents/{name}/responses` endpoints.

- id: string — Response ID (UUID)
- agent_name: string — Agent name
- status: string — One of: `pending`, `processing`, `completed`, `failed`, `cancelled`
- input_content: array — User input content items (preferred shape uses `content` array; legacy `{ text: string }` accepted)
- output_content: array — Final content items extracted from segments (typically the tool_result payload)
- segments: array — All step-by-step segments/items: commentary, tool calls/results, system markers, final
- created_at: string (RFC3339) — Creation timestamp (UTC)
- updated_at: string (RFC3339) — Last update timestamp (UTC)

GET list is ordered by `created_at` ascending.

Update semantics (PUT `/responses/{id}`):
- `output.text` replaces
- `output.items` appends
- Other `output` keys overwrite

Preferred input uses `content` array; legacy `text` field is still accepted.

## Common Schemas

Version
- version: string — Semantic version of server
- api: string — API namespace (e.g., v0)

Auth Profile
- user: string — Principal name
- type: string — Admin or User

Token Response
- token: string — JWT token
- token_type: string — Always `Bearer`
- expires_at: string (RFC3339) — Expiry timestamp
- user: string — Principal name associated with token
- role: string — `admin` or `user`

Operator Object
- user: string — Operator username
- description: string|null — Optional description
- active: boolean — Account active flag
- created_at: string (RFC3339) — Creation timestamp
- updated_at: string (RFC3339) — Last update timestamp
- last_login_at: string|null (RFC3339) — Last login timestamp

Agent Object
- name: string — Agent name (primary key)
- created_by: string — Owner username
- state: string — `init|idle|busy|slept`
- description: string|null — Optional description
- parent_agent_name: string|null — Parent agent name if remixed
- created_at: string (RFC3339) — Creation timestamp
- last_activity_at: string|null (RFC3339) — Last activity timestamp
- metadata: object — Arbitrary JSON metadata
- tags: string[] — Array of alphanumeric tags (stored lowercase)
- is_published: boolean — Published state
- published_at: string|null (RFC3339) — When published
- published_by: string|null — Who published
- publish_permissions: object — `{ code: boolean, secrets: boolean, content: boolean }`
- idle_timeout_seconds: int — Idle timeout
- busy_timeout_seconds: int — Busy timeout
- idle_from: string|null (RFC3339) — When idle started
- busy_from: string|null (RFC3339) — When busy started
- context_cutoff_at: string|null (RFC3339) — Current context cutoff timestamp if set

Count Object
- count: int — Count value
- agent_name: string — Agent identifier the count pertains to

Agent Busy/Idle Response
- success: boolean — Always true on success
- state: string — `busy` or `idle`
- timeout_status: string — `paused` (busy) or `active` (idle)

Agent State Update Response
- success: boolean — Always true on success
- state: string — New state string

