# Lattice Studio — Launch Decision Record

**Decision:** Do not declare the current repository production-ready.

**Observed baseline:** The repository `onegayunicorn/MVP` is a public Vite/React project. Its package manifest names React 19 and Vite, with `vite build` and `tsc --noEmit` as the principal build/type gates. The README is still based on an AI Studio template and references `GEMINI_API_KEY`.

**Implication:** The previously described Next.js/Supabase/R2/Stripe/Auth.js architecture is a target architecture, not verified implementation in this repository.

## Required pivot

Move from **"deploy the claimed V1"** to **"establish the verified production foundation, then promote capabilities one gate at a time."**

## Architecture decision process

Evaluate:

1. Continue Vite/React and add a dedicated backend.
2. Migrate to Next.js App Router with server-side API boundaries.
3. Use a hybrid architecture: Vite frontend plus separately deployed API/media worker services.

Score each against:

- delivery time;
- media-processing suitability;
- authentication complexity;
- deployment simplicity;
- observability;
- security isolation;
- cost;
- vendor lock-in;
- future API/enterprise requirements.

The selected architecture must be recorded in a subsequent ADR before implementation of production infrastructure.

## Commercial decision

The `$10/month` Pro price and `$5,000` launch-pool objective remain **planned commercial parameters**. The 50% giving basis must be defined precisely and reviewed before publication. Do not describe the giving calculation as statutory profit, accounting profit, or a charitable deduction without professional review.

## AI decision

Use a provider-neutral interface. Do not hard-code provider prices, latency claims or model availability into business logic. Benchmark current providers at implementation time and record observed results.

## Exit criteria

The launch decision may be revisited only after:

- successful build;
- successful typecheck;
- meaningful automated test coverage;
- security review;
- persistent media path;
- authenticated access;
- billing reconciliation tests;
- quota enforcement tests;
- AI spend controls;
- giving-ledger reconciliation;
- observability;
- rollback test;
- human approval of production activation.
