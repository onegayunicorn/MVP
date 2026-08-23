# Lattice Studio — Global Operations & Pivot Control Plane

**Status:** STAGED / REQUIRES REVIEW
**Repository:** `onegayunicorn/MVP`
**Branch:** `ops/lattice-studio-global-launch`
**Product:** Lattice Studio — Create. Transform. Give Forward.
**Control principle:** Evidence → economics → security → human authorization → execution.

## 1. Reality Baseline

The current repository must be treated as a Vite/React prototype, not as an already-production Next.js 16 application.

Verified repository facts at baseline:

- Repository default branch: `main`.
- Repository is public.
- `package.json` identifies a React 19 + Vite application.
- Build command is `vite build`.
- Type/lint gate is `tsc --noEmit`.
- Runtime uses Vite on port 3000.
- The repository README is based on an AI Studio template and currently instructs use of `GEMINI_API_KEY`.
- No evidence in the inspected repository establishes production authentication, Stripe billing, R2/S3 persistence, Supabase/PostgreSQL, the proposed AI router, or the giving ledger.

Therefore all claims in prior planning documents that describe those components as deployed or production-ready are classified **PLANNED** until repository evidence and runtime verification establish otherwise.

## 2. Operating State Machine

Every material work item has exactly one operational state:

`DISCOVERED → VERIFIED → PLANNED → STAGED → TESTED → APPROVED → DEPLOYED → MONITORED`

Exception states:

- `BLOCKED` — a required dependency, control, or approval is missing.
- `REJECTED` — the proposed action failed policy or review.
- `ROLLED_BACK` — deployment was reverted.
- `DEPRECATED` — replaced by a newer approved implementation.

No state may be skipped by wording alone.

## 3. Authority Classes

### A0 — Read-only

May run automatically:

- repository inspection;
- documentation generation;
- dependency inventory;
- static analysis;
- architecture analysis;
- cost modelling;
- market research;
- simulation;
- test planning;
- telemetry analysis;
- draft generation.

### A1 — Sandboxed engineering

May run in an isolated environment:

- source changes on feature branches;
- unit/integration tests;
- type checks;
- builds;
- dependency compatibility checks;
- artifact generation;
- preview deployments;
- synthetic webhook tests.

### A2 — Controlled external change

Requires explicit human approval before execution:

- production deployment;
- DNS/domain changes;
- production environment changes;
- billing activation;
- public launch;
- partner commitments;
- investor outreach under the company identity;
- publication of financial or impact claims.

### A3 — Restricted actions

Never autonomous:

- movement of company funds;
- charitable transfers;
- binding contracts;
- production secret disclosure or extraction;
- destructive data operations without approved recovery procedure;
- physical actuation;
- laboratory control;
- laser/high-energy/biological-system control;
- changing the authorization policy to approve one's own action.

## 4. Core Operations

### OP-001 — Repository Baseline

**Inputs:** repository, branch, environment metadata.

**Actions:** inspect manifests, source tree, configuration, CI, tests, secrets patterns, dependencies and deployment metadata.

**Evidence:** commit SHA, file inventory, command results, findings.

**Exit:** verified baseline or blocked.

### OP-002 — Production Gap Audit

Compare the verified baseline against V1 acceptance criteria:

- authentication;
- project persistence;
- media storage;
- media validation;
- deterministic processing;
- AI routing;
- quota enforcement;
- billing;
- webhooks;
- giving ledger;
- audit logging;
- observability;
- security controls;
- backup/recovery;
- policies;
- incident response.

### OP-003 — Architecture Transition

Do not force the present Vite prototype into a Next.js architecture merely because the planning document names Next.js. Select the production architecture after an explicit decision record comparing migration cost, operational complexity, deployment target, media-processing requirements and team capability.

### OP-004 — Deterministic Media Path

Preferred path:

`request → validate → classify → deterministic transform → validate output → persist → audit`

Candidate operations:

- crop;
- resize;
- rotate;
- transcode;
- trim;
- format conversion.

No AI provider is called for an operation that can be completed deterministically at acceptable quality.

### OP-005 — AI Router

Router inputs:

- operation;
- media type;
- media dimensions/duration;
- user plan;
- quality target;
- latency target;
- provider availability;
- estimated provider cost;
- account/user budget remaining.

Router output:

- route;
- provider;
- estimated cost;
- timeout;
- retry policy;
- audit metadata.

The router must be policy-driven. Provider names and price assumptions must be configuration, not hard-coded business truth.

### OP-006 — Cost Guard

Before any billable AI operation:

`authorize → estimate → compare budget → execute → record actual cost → reconcile`

Hard limits:

- per-request;
- per-user/day;
- per-account/month;
- provider/day;
- platform/month.

If a limit is reached, fail closed or route to an approved cheaper path. Never silently escalate spend.

### OP-007 — Billing

Stripe or another approved payment provider may be selected after a current vendor/compliance review.

Required controls:

- server-side checkout creation;
- verified webhook signatures;
- idempotency;
- subscription state machine;
- entitlement reconciliation;
- cancellation;
- failed payment handling;
- refund workflow;
- invoice records;
- audit trail.

### OP-008 — Giving Ledger

Use an append-only transaction model rather than a mutable balance as the source of truth.

Minimum events:

`REVENUE_RECORDED`
`PAYMENT_FEE_RECORDED`
`VARIABLE_COST_RECORDED`
`ELIGIBLE_POOL_CALCULATED`
`GIVING_ALLOCATED`
`TRANSFER_AUTHORIZED`
`TRANSFER_COMPLETED`
`TRANSFER_VERIFIED`
`ADJUSTMENT_RECORDED`

The public balance is derived from verified ledger events.

No public donation claim may be published before transfer verification.

### OP-009 — Security Gate

Test:

- authentication;
- authorization;
- tenant isolation;
- upload type/size validation;
- malware scanning boundary;
- path traversal;
- SSRF;
- command injection;
- XSS/CSRF where applicable;
- SQL injection;
- secret exposure;
- webhook forgery/replay;
- rate limiting;
- abuse controls.

Media must be handled as untrusted data.

### OP-010 — Release Gate

`lint → typecheck → unit → integration → security → build → artifact hash → preview → browser verification → approval → production`

Rollback must be tested before declaring production readiness.

## 5. Commercial Operations

### COM-001 — Free Funnel

Target: one free edit/day.

Measure:

- activation;
- edit completion;
- export completion;
- return rate;
- conversion to Pro;
- support load;
- variable cost per active user.

### COM-002 — Pro

Initial target: `$10/month`.

Do not describe the plan as economically sustainable until observed usage and variable-cost data validate the margin model.

### COM-003 — Unit Economics

Track:

`revenue`
`payment_fees`
`AI_cost`
`media_processing_cost`
`storage_cost`
`bandwidth_cost`
`support_cost`
`refunds`
`designated_giving_allocation`

Define the internal contribution metric explicitly before using it in public claims.

### COM-004 — Giving Pool

Initial objective: `$5,000`.

Proposed allocation rule: `50%` of the approved designated launch contribution base.

The exact accounting base must be approved and documented before public launch. Do not equate a management contribution metric with statutory profit, taxable income, or a legal charitable obligation without professional review.

### COM-005 — Investor Pipeline

Stages:

`IDENTIFIED → QUALIFIED → RESEARCHED → PREPARED → APPROVED → CONTACTED → RESPONSE → MEETING → DILIGENCE → TERM DISCUSSION → CLOSED/LOST`

Evidence labels are mandatory:

`VERIFIED | PROJECTED | PLANNED | EXPERIMENTAL | SIMULATED`

No fabricated traction, valuation, revenue, partnership, regulatory status or investor interest.

### COM-006 — Partnership Pipeline

Segment:

- technology;
- distribution;
- creators;
- education;
- social impact;
- enterprise;
- infrastructure;
- media platforms.

No external commitment without authorization.

## 6. Pivot Engine

A pivot is triggered by evidence, not enthusiasm.

### P-01 — Product Pivot

Trigger examples:

- activation below target;
- users repeatedly abandon upload/edit/export;
- a specific workflow dominates usage;
- support volume reveals a simpler high-value job.

Actions:

1. isolate the failing funnel stage;
2. segment users;
3. identify the highest-retention workflow;
4. prototype the smallest change;
5. run controlled test;
6. compare value and cost;
7. adopt, reject, or revert.

### P-02 — Pricing Pivot

Trigger:

`conversion × ARPU × retention × gross margin` fails the approved target range.

Possible moves:

- adjust included AI credits;
- introduce usage-based AI add-ons;
- introduce creator tier;
- introduce annual plan;
- reduce expensive default operations.

No pricing change is launched without impact modelling.

### P-03 — AI Provider Pivot

Trigger:

- quality regression;
- latency breach;
- provider outage;
- cost increase;
- regional restriction;
- policy incompatibility.

Pivot path:

`benchmark → shadow test → compare → canary → promote → monitor`

Never migrate blindly on a marketing claim.

### P-04 — Infrastructure Pivot

Trigger:

- cost per successful edit exceeds target;
- queue latency breaches SLO;
- storage/bandwidth economics deteriorate;
- reliability is below target;
- vendor lock-in becomes material.

Compare at least two viable architectures before a major migration.

### P-05 — Market Pivot

Trigger:

A segment demonstrates materially stronger retention, willingness to pay or acquisition efficiency than the initial target.

Candidate segments:

- creators;
- small businesses;
- educators;
- agencies;
- developers/API users;
- enterprise media teams.

Do not expand every segment simultaneously.

### P-06 — Giving Pivot

The giving mission remains intact, but beneficiary selection may change when verification, geographic relevance, impact evidence, regulatory status or transfer reliability changes.

Never reduce transparency to improve marketing optics.

### P-07 — Corporate/Legal Pivot

Any change to entity structure, ownership, tax position, public-benefit commitments, charitable mechanism or jurisdiction requires qualified professional review before implementation.

## 7. Pivot Decision Matrix

| Signal | Threshold Direction | First Response | Escalation | Decision Owner |
|---|---|---|---|---|
| Activation | Below target | Funnel audit | Product pivot | Human owner |
| Edit success | Below SLO | Error analysis | Processing pivot | Engineering |
| AI cost/edit | Above budget | cheaper route | Provider/architecture pivot | Engineering + finance |
| Pro conversion | Below target | pricing/funnel test | Pricing pivot | Commercial owner |
| Churn | Above target | cohort analysis | Product/pricing pivot | Commercial owner |
| Gross margin | Below target | cost decomposition | Pricing/compute pivot | Human owner |
| Security | Critical finding | block release | Security remediation | Security owner |
| Billing | Reconciliation failure | stop entitlement mutation | Billing rollback | Finance/engineering |
| Giving ledger | Reconciliation failure | freeze publication | Ledger remediation | Human owner |
| Reliability | SLO breach | incident response | infrastructure pivot | Engineering |

## 8. Kill Switches

Implement independent controls for:

- all AI providers;
- each expensive model;
- image generation;
- video generation;
- new user signup;
- billing activation;
- public deployment;
- external webhook ingestion;
- automated outreach;
- giving transfer workflow.

A kill switch must fail closed and must not be controlled solely by the component it disables.

## 9. Command Center Views

### Executive

- active users;
- edits;
- exports;
- Pro subscribers;
- MRR;
- variable cost;
- contribution margin;
- giving pool;
- cash runway when available;
- incidents;
- release state.

### Engineering

- deployment SHA;
- build state;
- test state;
- error rate;
- latency;
- queue depth;
- provider health;
- cost/edit;
- storage/bandwidth.

### Governance

- approvals pending;
- restricted actions attempted;
- audit events;
- policy changes;
- security findings;
- evidence gaps.

### Commercial

- acquisition;
- activation;
- conversion;
- churn;
- cohorts;
- CAC when measurable;
- LTV only when based on sufficient observed data.

## 10. Launch Sequence

### Phase 0 — Truth Baseline

- [ ] Freeze claims about production readiness.
- [ ] Inventory repository and dependencies.
- [ ] Run build/typecheck/tests.
- [ ] Map actual runtime architecture.
- [ ] Identify secret/configuration requirements.
- [ ] Create evidence manifest.

### Phase 1 — Production Core

- [ ] Select production application architecture.
- [ ] Implement authentication.
- [ ] Implement persistent projects/media storage.
- [ ] Implement deterministic processing.
- [ ] Implement secure media lifecycle.
- [ ] Implement observability.
- [ ] Implement audit events.

### Phase 2 — Commercial Core

- [ ] Implement billing.
- [ ] Implement entitlements.
- [ ] Implement free quota.
- [ ] Implement AI cost controls.
- [ ] Implement ledger.
- [ ] Define contribution accounting basis.

### Phase 3 — AI Expansion

- [ ] Build provider adapter interface.
- [ ] Benchmark candidate providers.
- [ ] Add deterministic-first routing.
- [ ] Add budget controls.
- [ ] Add provider failover.
- [ ] Add cost/quality telemetry.

### Phase 4 — Beta

- [ ] Private cohort.
- [ ] Observe usage.
- [ ] Measure unit economics.
- [ ] Fix critical defects.
- [ ] Validate support workflow.
- [ ] Validate privacy/deletion controls.

### Phase 5 — Launch

- [ ] Publish approved policies.
- [ ] Activate production billing.
- [ ] Publish verified giving mechanism.
- [ ] Launch acquisition experiments.
- [ ] Begin approved investor/partner outreach.
- [ ] Monitor daily.

### Phase 6 — Scale

- [ ] API.
- [ ] Creator tier.
- [ ] Enterprise.
- [ ] Marketplace.
- [ ] International expansion.
- [ ] Multi-region architecture only where justified by demand/compliance/reliability.

## 11. Evidence Manifest

Every material claim should point to evidence with:

- claim ID;
- claim text;
- state;
- source;
- commit/deployment ID;
- timestamp;
- verifier;
- expiry/review date.

Example states:

`VERIFIED` — directly evidenced.
`TESTED` — test evidence exists but production behavior is not established.
`PROJECTED` — modelled assumption.
`PLANNED` — approved design intent only.
`EXPERIMENTAL` — active controlled test.
`SIMULATED` — synthetic environment only.

## 12. Non-Negotiable Invariants

1. No uncontrolled spend escalation.
2. No secret extraction.
3. No self-authorization.
4. No fabricated business evidence.
5. No public donation claim without verified transfer evidence.
6. No production release without the release gate.
7. No physical/laboratory actuation from the application plane.
8. No destructive operation without recovery evidence.
9. No provider lock-in at the routing interface.
10. No optimization that materially degrades security, privacy or user trust.

## 13. Immediate Execution Queue

`QUEUE-001` — Verify baseline build/typecheck/test results.

`QUEUE-002` — Inventory actual source architecture and compare it with the stated Next.js MVP specification.

`QUEUE-003` — Produce a dependency/security inventory.

`QUEUE-004` — Decide production application architecture through an ADR.

`QUEUE-005` — Build the deterministic media path first.

`QUEUE-006` — Build persistence and secure media lifecycle.

`QUEUE-007` — Build authentication and authorization.

`QUEUE-008` — Build provider-neutral AI router with hard spend ceilings.

`QUEUE-009` — Build billing and entitlement reconciliation.

`QUEUE-010` — Build append-only giving ledger and reconciliation tests.

`QUEUE-011` — Add CI/CD release gates and rollback.

`QUEUE-012` — Establish observability and unit-economics telemetry.

`QUEUE-013` — Run private beta and collect evidence.

`QUEUE-014` — Trigger pivots only from measured signals.

## 14. Approval Boundary

This branch is documentation/staging work. Production deployment, financial activation, charitable transfers, legal commitments, public claims and other A2/A3 actions remain blocked until explicitly authorized by the responsible human operator.

**Core invariant:**

`Create value → verify value → capture value → give value forward → reinvest intelligently → grow.`
